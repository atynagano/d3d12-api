fn main() {
    println!(concat!("cargo:rustc-link-search=native=", env!("CARGO_MANIFEST_DIR"), "/libs/"));
}