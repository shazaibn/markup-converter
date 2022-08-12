# markup-converter

A utility to convert between YAML, TOML, and JSON

## Usage

Instantiate a new Transcoder instance with a JSON, YAML, or TOML path and use `.to_json()` or `.to_yaml()` to convert it to the specified format.

```rust
use markup_converter::Transcoder;

fn main() -> anyhow::Result<()> {
  let transcoder = Transcoder::from_path("tests/test.yaml")?;

  let json_val = transcoder.to_json()?;

  println!("{}", json_val);

  Ok(())
}
```
