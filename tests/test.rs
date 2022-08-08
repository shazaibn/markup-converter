use markup_converter::Transcoder;

use anyhow::Result;

#[test]
fn test_to_json() -> Result<()> {
    let yaml_converter = Transcoder::new("tests/test.yaml")?;
    let json_converter = Transcoder::new("tests/test.json")?;
    let toml_converter = Transcoder::new("tests/test.toml")?;

    let yaml_jsonified = yaml_converter.to_json()?;
    let json_jsonified = json_converter.to_json()?;
    let toml_jsonified = toml_converter.to_json()?;

    assert_eq!(yaml_jsonified, json_jsonified);
    assert_eq!(yaml_jsonified, toml_jsonified);
    Ok(())
}

#[test]
fn test_to_yaml() -> Result<()> {
    let yaml_converter = Transcoder::new("tests/test.yaml")?;
    let json_converter = Transcoder::new("tests/test.json")?;
    let toml_converter = Transcoder::new("tests/test.toml")?;

    let yaml_jsonified = yaml_converter.to_yaml()?;
    let json_jsonified = json_converter.to_yaml()?;
    let toml_jsonified = toml_converter.to_yaml()?;

    assert_eq!(yaml_jsonified, json_jsonified);
    assert_eq!(yaml_jsonified, toml_jsonified);
    Ok(())
}
