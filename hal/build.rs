fn main() {
    // this crate requires valid target selection
    if cfg!(not(feature = "_hal_target_selected")) {
        println!("cargo:error=Please select a target.");
        std::process::exit(1);
    }
}
