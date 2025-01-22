use blueprint_sdk::build;

fn main() {
    let contract_dirs: Vec<&str> = vec!["./contracts"];
    build::utils::soldeer_update();
    build::utils::build_contracts(contract_dirs);

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
    build::blueprint_metadata::generate_json();
}
