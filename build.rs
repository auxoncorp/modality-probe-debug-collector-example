#![deny(warnings)]

use modality_probe_cli::{header_gen, lang::Lang, manifest_gen};

fn main() {
    // Generate a component named "example-component", in the directory
    // "example-component" based on searching through the source code in
    // "src/main.rs"
    let manifest_gen_opts = manifest_gen::ManifestGen {
        lang: Some(Lang::Rust),
        component_name: "example-component".into(),
        output_path: "example-component".into(),
        source_path: "src/main.rs".into(),
        ..Default::default()
    };
    manifest_gen::run(manifest_gen_opts, None);

    // Generate Rust definitions in "src/component_definitions.rs"
    // from the component directory "example-component"
    let header_gen_opts = header_gen::HeaderGen {
        lang: Lang::Rust,
        output_path: Some("src/component_definitions.rs".into()),
        component_path: "example-component".into(),
        ..Default::default()
    };
    header_gen::run(header_gen_opts, None);

    println!("cargo:rerun-if-changed=src/main.rs");
}
