use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Read Cargo.toml
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_toml_path = Path::new(&manifest_dir).join("Cargo.toml");
    let cargo_toml = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

    // Parse leptos version
    if let Some(leptos_line) = cargo_toml.lines().find(|line| line.contains("leptos = ")) {
        if let Some(version_start) = leptos_line.find("version = \"") {
            let version_str = &leptos_line[version_start + 11..];
            if let Some(version_end) = version_str.find('"') {
                let version = &version_str[..version_end];
                println!("cargo:rustc-env=LEPTOS_VERSION={}", version);
            }
        }
    }

    // Parse leptos_router version
    if let Some(router_line) = cargo_toml
        .lines()
        .find(|line| line.contains("leptos_router = "))
    {
        if let Some(version_start) = router_line.find('"') {
            let version_str = &router_line[version_start + 1..];
            if let Some(version_end) = version_str.find('"') {
                let version = &version_str[..version_end];
                println!("cargo:rustc-env=LEPTOS_ROUTER_VERSION={}", version);
            }
        }
    }

    // Rerun if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
}
