fn main() {
    // this crate needs a target to be selected
    if cfg!(feature = "target-none") {
        println!("cargo:error=Please select a target.");
        std::process::exit(1);
    }
}
