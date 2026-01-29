use std::env::var;
use std::fs;
use std::fs::read_to_string;
use std::path::Path;

use schemars::schema::RootSchema;
use serde_json::from_str;
use typify::{TypeSpace, TypeSpaceSettings};

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = var("OUT_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);

    println!("cargo:rerun-if-changed=vk-schema.json");
    println!("cargo:rerun-if-changed=vk.json");

    // Generate schema.rs using typify as a library.
    let schema_json = read_to_string(manifest_path.join("vk-schema.json"))
        .expect("failed to read vk-schema.json");
    let schema: RootSchema = from_str(&schema_json).expect("failed to parse vk-schema.json");
    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());
    type_space
        .add_root_schema(schema)
        .expect("failed to process schema");
    let contents = type_space.to_stream().to_string();
    fs::write(manifest_path.join("src/schema.rs"), contents).expect("failed to write schema.rs");

    // Copy vk.json to OUT_DIR.
    let vk_json_src = manifest_path.join("vk.json");
    let vk_json_dst = Path::new(&out_dir).join("vk.json");
    fs::copy(&vk_json_src, &vk_json_dst).expect("failed to copy vk.json to OUT_DIR");
}
