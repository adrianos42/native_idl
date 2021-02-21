use serde::Serialize;
#[derive(Serialize)]
pub struct CargoGit {
    pub git: String,
}

#[derive(Serialize)]
pub struct CargoDep {
    pub version: String,
    pub features: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct CargoPackage {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
    pub edition: Option<String>,
}

#[derive(Serialize)]
pub struct CargoLib {
    pub name: String,
    #[serde(rename(serialize = "crate-type"))]
    pub crate_type: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct CargoFields<T: Serialize> {
    pub package: CargoPackage,
    pub lib: Option<CargoLib>,
    pub dependencies: Option<T>,
}

pub(crate) fn create_cargo<T: Serialize>(fields: CargoFields<T>) -> String {
    toml::to_string(&fields).unwrap()
}