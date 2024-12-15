use mlua::prelude::LuaValue;
use mlua::{FromLua, Lua};

#[derive(Debug)]
pub struct RockSpec {
    // metadata
    pub rockspec_format: Option<String>,
    pub package: String,
    pub version: String,
    pub description: Option<RockSpecDescription>,

    pub source: RockSpecSource,
}

impl FromLua for RockSpec {
    fn from_lua(_: LuaValue, lua: &Lua) -> mlua::Result<Self> {
        let globals = lua.globals();
        Ok(RockSpec {
            rockspec_format: globals.get("rockspec_format")?,
            package: globals.get("package")?,
            version: globals.get("version")?,
            description: globals.get("description")?,
            source: globals.get("source")?,
        })
    }
}

#[derive(Debug)]
pub struct RockSpecSource {
    pub url: String,
}

impl FromLua for RockSpecSource {
    fn from_lua(value: LuaValue, _: &Lua) -> Result<Self, mlua::Error> {
        match value {
            LuaValue::Table(table) => Ok(RockSpecSource {
                url: table.get("url")?,
            }),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "RockSpecSource".to_string(),
                message: Some("expected a Lua table".into()),
            }),
        }
    }
}

#[derive(Debug)]
pub struct RockSpecDescription {
    pub summary: Option<String>,
    pub detailed: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub issues_url: Option<String>,
    pub maintainer: Option<String>,
    pub labels: Option<Vec<String>>,
}

impl FromLua for RockSpecDescription {
    fn from_lua(value: LuaValue, _: &Lua) -> Result<Self, mlua::Error> {
        match value {
            LuaValue::Table(table) => Ok(RockSpecDescription {
                summary: table.get("summary")?,
                detailed: table.get("detailed")?,
                license: table.get("license")?,
                homepage: table.get("homepage")?,
                issues_url: table.get("issues_url")?,
                maintainer: table.get("maintainer")?,
                labels: table.get("labels")?,
            }),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "RockSpecDescription".to_string(),
                message: Some("expected a Lua table".into()),
            }),
        }
    }
}
