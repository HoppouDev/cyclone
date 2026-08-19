use std::{env, fs, path::PathBuf};

use schemars::schema::RootSchema;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let schema_path = manifest_dir.join("openrouter/images.schema.json");
    println!("cargo:rerun-if-changed={}", schema_path.display());

    let schema_json = fs::read_to_string(&schema_path).expect("failed to read openrouter schema");
    let root_schema: RootSchema =
        serde_json::from_str(&schema_json).expect("failed to parse openrouter schema");

    let mut settings = TypeSpaceSettings::default();
    settings.with_struct_builder(true);

    let mut type_space = TypeSpace::new(&settings);
    type_space
        .add_root_schema(root_schema)
        .expect("failed to convert openrouter schema to rust types");

    let contents = type_space.to_stream().to_string();
    let contents = rustfmt_wrapper::rustfmt(contents.clone()).unwrap_or_else(|e| {
        println!("cargo:warning=failed to format generated openrouter types: {e}");
        contents
    });

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("openrouter_types.rs"), contents)
        .expect("failed to write generated openrouter types");
}
