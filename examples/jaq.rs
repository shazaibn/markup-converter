use jaq_core::{parse, Ctx, Definitions, Val};
use markup_converter::Transcoder;

fn main() -> anyhow::Result<()> {
    let input = Transcoder::from_path("tests/test.yaml")?.to_json()?;

    let filter = ".name";

    // start out only from core filters,
    // which do not include filters in the standard library
    // such as `map`, `select` etc.
    let defs = Definitions::core();

    // parse the filter in the context of the given definitions
    let mut errs = Vec::new();
    let f = parse::parse(filter, parse::main()).0.unwrap();
    let f = defs.finish(f, Vec::new(), &mut errs);

    // iterator over the output values
    let out = f.run(Ctx::new(), Val::from(input));

    for val in out {
        println!("{}", val.unwrap());
    }
    Ok(())
}
