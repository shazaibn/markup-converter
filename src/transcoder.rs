use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
enum Format {
    Json(serde_json::Value),
    Toml(toml::Value),
    Yaml(serde_yaml::Value),
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
        match toml::from_str(&markup) {
            Ok(toml_value) => Format::Toml(toml_value),
            Err(e) => {
                return Err(Error::BadToml(e.to_string()));
            }
        }
    } else if ext.eq_ignore_ascii_case("yaml") || ext.eq_ignore_ascii_case("yml") {
        match serde_yaml::from_str(&markup) {
            Ok(val) => Format::Yaml(val),
            Err(e) => {
                return Err(Error::BadYaml(e.to_string()));
            }
        }
    } else if ext.eq_ignore_ascii_case("json") {
        match serde_json::from_str(&markup) {
            Ok(val) => Format::Json(val),
            Err(e) => {
                return Err(Error::BadJson(e.to_string()));
            }
        }
    } else {
        return Err(Error::UnknownFileExtension(
            path.to_string_lossy().to_string(),
        ));
    };

    Ok(value)
}

impl Transcoder {
    /// Constructor for [Transcoder], takes a file path.
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self> {
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
