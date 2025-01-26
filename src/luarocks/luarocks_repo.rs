#[derive(Debug)]
pub struct LuaRocksNamespaceRepo {
    #[allow(dead_code)]
    pub namespace: String,
    pub packages: Vec<LuaRocksRepoPackage>,
}

impl LuaRocksNamespaceRepo {
    pub fn new(namespace: String) -> Self {
        let packages: Vec<LuaRocksRepoPackage> = Vec::new();
        Self {
            namespace,
            packages,
        }
    }

    pub fn get_package_by_name(&self, package_name: &str) -> Option<&LuaRocksRepoPackage> {
        self.packages.iter().find(|p| p.name == package_name)
    }
}

#[derive(Debug)]
pub struct LuaRocksRepoPackage {
    pub name: String,
    pub versions: Vec<LuaRocksRepoPackageVersion>,
}

impl LuaRocksRepoPackage {
    pub fn new(name: String) -> Self {
        let versions: Vec<LuaRocksRepoPackageVersion> = Vec::new();
        Self { name, versions }
    }

    pub fn get_latest_package_version(&self) -> Option<&LuaRocksRepoPackageVersion> {
        self.versions.iter().last()
    }

    pub fn get_specific_package_version(
        &self,
        version: &str,
    ) -> Option<&LuaRocksRepoPackageVersion> {
        self.versions.iter().find(|v| v.version == version)
    }
}

#[derive(Debug)]
pub struct LuaRocksRepoPackageVersion {
    pub version: String,
    #[allow(dead_code)]
    pub arch: String,
}

impl LuaRocksRepoPackageVersion {
    pub fn new(version: String, arch: String) -> Self {
        Self { version, arch }
    }
}
