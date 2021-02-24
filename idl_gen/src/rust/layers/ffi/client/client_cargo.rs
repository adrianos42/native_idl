use super::FFIClientError;
use crate::cargo_m::*;
use core::fmt;
use idl::ids;
use std::collections::HashMap;

pub struct FFIClientCargo {
    cargo_toml: Option<String>,
}

impl fmt::Display for FFIClientCargo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cargo_toml.as_ref().unwrap())
    }
}

impl FFIClientCargo {
    pub fn generate(
        analyzer: &ids::analyzer::Analyzer,
        client_name: &str,
    ) -> Result<Self, FFIClientError> {
        let package_name = analyzer.get_package().name();
        let target_client = analyzer
            .find_client(client_name)
            .ok_or(FFIClientError::UnknownType(client_name.to_owned()))?;

        let mut context = FFIClientCargo::new();

        let mut dependencies = HashMap::<String, HashMap<String, String>>::new();

        let mut git = HashMap::<String, String>::new();
        git.insert(
            "git".to_owned(),
            "https://github.com/adrianos42/native_idl".to_owned(),
        );
        dependencies.insert("idl_internal".to_owned(), git);

        let fields = CargoFields {
            package: CargoPackage {
                name: package_name.to_owned(),
                authors: None,
                edition: Some("2018".to_owned()),
                version: "0.1.0".to_owned(),
            },
            lib: None,
            dependencies: Some(dependencies),
        };

        context.cargo_toml = Some(create_cargo(fields));

        Ok(context)
    }

    fn new() -> Self {
        Self { cargo_toml: None }
    }
}
