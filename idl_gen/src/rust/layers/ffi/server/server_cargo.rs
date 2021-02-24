use super::FFIServerError;
use crate::cargo_m::*;
use core::fmt;
use idl::ids;
use std::collections::HashMap;

pub struct FFIServerCargo {
    cargo_toml: Option<String>,
}

impl fmt::Display for FFIServerCargo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cargo_toml.as_ref().unwrap())
    }
}

impl FFIServerCargo {
    pub fn generate(
        analyzer: &ids::analyzer::Analyzer,
        server_name: &str,
    ) -> Result<Self, FFIServerError> {
        let package_name = analyzer.get_package().name();
        let target_server = analyzer
            .find_server(server_name)
            .ok_or(FFIServerError::Undefined)?;

        let mut context = FFIServerCargo::new();

        let mut dependencies = HashMap::<String, HashMap<String, String>>::new();

        let path = target_server
            .get_field("path")
            .and_then(|v| v.as_string_value())
            .or_else(|| {
                target_server
                    .get_field("git")
                    .and_then(|v| v.as_string_value())
            })
            .unwrap_or_else(|| "rust/".to_owned()); // TODO replace with the right path

        let mut git = HashMap::<String, String>::new();
        git.insert(
            "git".to_owned(),
            "https://github.com/adrianos42/native_idl".to_owned(),
        );
        dependencies.insert("idl_internal".to_owned(), git);

        let mut lib_path = HashMap::<String, String>::new();
        lib_path.insert("path".to_owned(), path);
        dependencies.insert(package_name.to_owned(), lib_path);

        let fields = CargoFields {
            package: CargoPackage {
                name: "idl_ffi".to_owned(),
                authors: None,
                edition: Some("2018".to_owned()),
                version: "0.1.0".to_owned(),
            },
            lib: Some(CargoLib {
                crate_type: Some(vec!["staticlib".to_owned(), "cdylib".to_owned()]),
                name: package_name.to_owned(),
            }),
            dependencies: Some(dependencies),
        };

        context.cargo_toml = Some(create_cargo(fields));

        Ok(context)
    }

    fn new() -> Self {
        Self { cargo_toml: None }
    }
}
