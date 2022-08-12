use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
/// An enum with variants that contain the deserialized representations of supported formats.
pub enum Format {
    /// JSON Format
    Json(serde_json::Value),
    /// TOML Format
    Toml(toml::Value),
    /// YAML format
    Yaml(serde_yaml::Value),
}

impl Format {
    /// Parse a string as YAML and return a Format::Yaml
    pub fn yaml(src: &str) -> Result<Format> {
        serde_yaml::from_str(src)
            .map(Format::Yaml)
            .map_err(|e| Error::BadYaml(e.to_string()))
    }
    /// Parse a string as TOML and return a Format::Toml
    pub fn toml(src: &str) -> Result<Format> {
        toml::from_str(src)
            .map(Format::Toml)
            .map_err(|e| Error::BadToml(e.to_string()))
    }
    /// Parse a string as JSON and return a Format::Json
    pub fn json(src: &str) -> Result<Format> {
        serde_json::from_str(src)
            .map(Format::Json)
            .map_err(|e| Error::BadJson(e.to_string()))
    }
}

#[derive(Debug)]
/// Transcoder wraps the parsed file in its intermediary format before conversion.
pub struct Transcoder {
    input: Format,
}

fn parse(path: impl AsRef<std::path::Path>) -> Result<Format> {
    let path = path.as_ref();
    let ext = path
        .extension()
        .ok_or_else(|| Error::UnknownFileExtension(path.to_string_lossy().to_string()))?;

    let markup = std::fs::read_to_string(path)
        .map_err(|e| Error::FileRead(path.to_string_lossy().to_string(), e.to_string()))?;

    let value = if ext.eq_ignore_ascii_case("toml") {
        Format::toml(&markup)?
    } else if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") {
        Format::yaml(&markup)?
    } else if ext.eq_ignore_ascii_case("json") {
        Format::json(&markup)?
    } else {
        return Err(Error::UnknownFileExtension(
            path.to_string_lossy().to_string(),
        ));
    };

    Ok(value)
}

impl Transcoder {
    /// Constructor for [Transcoder] that takes a [Format]
    pub fn new(input: Format) -> Result<Self> {
        Ok(Self { input })
    }

    /// Constructor for [Transcoder] that reads a file path and infers the type from the extension.
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let input = parse(path)?;

        Ok(Self { input })
    }

    /// Convert the parsed document to a JSON Value.
    pub fn to_json(&self) -> Result<serde_json::Value> {
        let json_val = match &self.input {
            Format::Json(val) => val.clone(),
            Format::Toml(val) => {
                serde_json::to_value(val).map_err(|e| Error::JsonConversion(e.to_string()))?
            }
            Format::Yaml(val) => {
                serde_json::to_value(val).map_err(|e| Error::JsonConversion(e.to_string()))?
            }
        };

        Ok(json_val)
    }

    /// Convert the parsed document to a YAML Value.
    pub fn to_yaml(&self) -> Result<serde_yaml::Value> {
        let json_val = match &self.input {
            Format::Json(val) => {
                serde_yaml::to_value(val).map_err(|e| Error::YamlConversion(e.to_string()))?
            }
            Format::Toml(val) => {
                serde_yaml::to_value(val).map_err(|e| Error::YamlConversion(e.to_string()))?
            }
            Format::Yaml(val) => val.clone(),
        };

        Ok(json_val)
    }
}

#[cfg(test)]
mod test {
    use super::Format;
    use anyhow::Result;

    #[test]
    fn test_parse_yaml() -> Result<()> {
        let format = super::parse("tests/test.yaml")?;
        assert!(matches!(format, Format::Yaml(_)));

        Ok(())
    }

    #[test]
    fn test_parse_json() -> Result<()> {
        let format = super::parse("tests/test.json")?;
        assert!(matches!(format, Format::Json(_)));

        Ok(())
    }

    #[test]
    fn test_parse_toml() -> Result<()> {
        let format = super::parse("tests/test.toml")?;
        println!("{:?}", format);
        assert!(matches!(format, Format::Toml(_)));

        Ok(())
    }
}
