use crate::luarocks::formats::rockspec::RockSpec;
use crate::luarocks::luarocks_repo::{
    LuaRocksNamespaceRepo, LuaRocksRepoPackage, LuaRocksRepoPackageVersion,
};
use mlua::Lua;

/// Parses a LuaRocksNamespaceRepo struct from a lua object loaded from the remote repository.
///
/// # Arguments
///
/// * `lua` - Lua (mlua::Lua) object created from the remote repository.
/// * `namespace` - Repository namespaced (the first part of the name as NAMESPACE/PACKAGE)
///
fn parse_namespace_repo_from_lua(
    lua: &Lua,
    namespace: &str,
) -> Result<LuaRocksNamespaceRepo, String> {
    let repo_lua_globals = lua.globals();
    let repo_packages = match repo_lua_globals.get::<mlua::Table>("repository") {
        Ok(v) => v,
        Err(_) => return Err("Failed to get repository table".to_string()),
    };

    let mut result = LuaRocksNamespaceRepo::new(namespace.to_string());

    // Build every package in the repository
    for package_name_pair in repo_packages.pairs::<String, mlua::Table>() {
        let (package_name, package_versions) = match package_name_pair {
            Ok(v) => v,
            Err(_) => return Err("Failed to parse package name".to_string()),
        };

        let mut package = LuaRocksRepoPackage::new(package_name);

        // For every version of the package
        for package_version_pair in package_versions.pairs::<String, mlua::Table>() {
            let (package_version, package_spec) = match package_version_pair {
                Ok(v) => v,
                Err(_) => return Err("Failed to parse package version".to_string()),
            };

            /*
               Packages are structured as:

               ['package'] = {
                   ['version'] = {
                      {
                         arch = "rockspec"
                      }
                   }
               }

               Notice the extra brackets inside "version", that makes "version" a list. So we need to access arch with
               something like "package.version[1].arch".
            */
            let mut version_content_list = package_spec.sequence_values::<mlua::Table>();
            let version_first_element = match version_content_list.next() {
                Some(v) => v,
                None => return Err("Failed to parse package version".to_string()),
            };

            let version_first_element_table = match version_first_element {
                Ok(v) => v,
                Err(_) => return Err("Failed to parse package version".to_string()),
            };

            let arch = match version_first_element_table.get::<String>("arch") {
                Ok(v) => v,
                Err(_) => return Err("Failed to parse package arch".to_string()),
            };

            let package_version = LuaRocksRepoPackageVersion::new(package_version, arch);
            package.versions.push(package_version);
        }

        result.packages.push(package);
    }

    Ok(result)
}

pub async fn load_namespace_repository(
    repo: &str,
    namespace: &str,
) -> Result<LuaRocksNamespaceRepo, String> {
    let repo_request = reqwest::get(format!("{}/manifests/{}", repo, namespace).as_str()).await;

    let repo_req_content = match repo_request {
        Ok(content) => content,
        Err(e) => {
            return Err(format!(
                "Failed to download manifest: {}",
                e.status().unwrap()
            ));
        }
    };

    match repo_req_content.error_for_status_ref() {
        Ok(_) => (),
        Err(e) => {
            let status = e.status().unwrap();
            return match status {
                reqwest::StatusCode::NOT_FOUND => {
                    Err(format!("Namespace '{}' not found", namespace))
                }
                _ => Err(status.to_string()),
            };
        }
    }

    let response_text = match repo_req_content.text().await {
        Ok(text) => text,
        Err(_) => return Err("Failed to read manifest".to_string()),
    };

    let repo_lua = Lua::new();
    let repo_lua_exec = repo_lua.load(response_text).exec();

    if repo_lua_exec.is_err() {
        return Err("Failed to parse Lua response".to_string());
    }

    parse_namespace_repo_from_lua(&repo_lua, &namespace)
}

pub async fn load_package_rockspec(
    repo: &str,
    namespace: &str,
    package: &str,
    version: &str,
) -> Result<RockSpec, String> {
    let uri = format!(
        "{}/manifests/{}/{}-{}.rockspec",
        repo, namespace, package, version
    );
    let request = match reqwest::get(uri.as_str()).await {
        Ok(content) => content,
        Err(_) => return Err("Failed to request package manifest".to_string()),
    };

    match request.error_for_status_ref() {
        Ok(_) => (),
        Err(e) => {
            let status = e.status().unwrap();
            return match status {
                reqwest::StatusCode::NOT_FOUND => Err("Not found".to_string()),
                _ => Err(status.to_string()),
            };
        }
    }

    let response_text = match request.text().await {
        Ok(text) => text,
        Err(_) => return Err("Failed to read package manifest".to_string()),
    };

    let lua = Lua::new();
    match lua.load(response_text).eval::<RockSpec>() {
        Ok(r) => Ok(r),
        Err(e) => Err(format!("Failed to parse package manifest: {}", e)),
    }
}
