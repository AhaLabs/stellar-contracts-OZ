mod config;
mod fragments;
mod gen_cargo;
mod gen_contract;
mod gen_lib;
mod gen_test;

use config::COMBINATIONS;
use std::fs;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let combinations_dir = root.join("combinations");

    // Clean and recreate
    if combinations_dir.exists() {
        fs::remove_dir_all(&combinations_dir).expect("Failed to clean combinations/");
    }

    println!("Generating {} combinations...", COMBINATIONS.len());

    for cfg in COMBINATIONS {
        let crate_dir = combinations_dir.join(cfg.name);
        let src_dir = crate_dir.join("src");
        fs::create_dir_all(&src_dir).expect("Failed to create directories");

        // Cargo.toml
        let cargo = gen_cargo::generate_cargo_toml(cfg);
        fs::write(crate_dir.join("Cargo.toml"), cargo).expect("Failed to write Cargo.toml");

        // src/lib.rs
        let lib = gen_lib::generate_lib();
        fs::write(src_dir.join("lib.rs"), lib).expect("Failed to write lib.rs");

        // src/contract.rs
        let contract = gen_contract::generate_contract(cfg);
        fs::write(src_dir.join("contract.rs"), contract).expect("Failed to write contract.rs");

        // src/test.rs
        let test = gen_test::generate_test(cfg);
        fs::write(src_dir.join("test.rs"), test).expect("Failed to write test.rs");

        println!("  Generated: {}", cfg.name);
    }

    println!(
        "\nDone! Generated {} combinations in combinations/",
        COMBINATIONS.len()
    );
    println!("Run `cargo +nightly fmt` to format the generated code.");
}
