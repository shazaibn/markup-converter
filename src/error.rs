#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not parse TOML: {0}")]
    BadToml(String),

    #[error("Could not parse YAML: {0}")]
    BadYaml(String),

    #[error("Could not parse JSON: {0}")]
    BadJson(String),

    #[error("Unknown file extension for file '{0}'")]
    UnknownFileExtension(String),

    #[error("Could not read file '{0}', error was: {1}")]
    FileRead(String, String),

    #[error("Could not convert to JSON: {0}")]
    JsonConversion(String),

    #[error("Could not convert to YAML: {0}")]
    YamlConversion(String),

    #[error("Could not convert to TOML: {0}")]
    TomlConversion(String),
}
