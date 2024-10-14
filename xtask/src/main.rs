// disable warnings in release mode
#![cfg_attr(not(debug_assertions), allow(warnings))]

use std::{
    env::var,
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
};

use clap::{
    builder::{ArgPredicate, ValueParser},
    Arg, ArgAction, Command,
};
use toml;
use xshell::{cmd, Shell};

const APP_CRATE_NAME: &str = "app";

#[derive(Ord, PartialEq, Eq, PartialOrd, Clone)]
struct Target {
    pub crate_name: String,
    pub rust_target: String,
    pub features: Vec<String>,
}

enum FlashMethod {
    DFU { usb_id: Option<String> },
    UF2 { path: String },
}

struct CargoManifest {
    pub name: String,
    pub features: Vec<String>,
}

struct CargoConfig {
    pub target: String,
}

fn get_workspace_dir() -> PathBuf {
    Path::new(&var("CARGO_MANIFEST_DIR").unwrap()).join("..")
}

fn get_targets_dir() -> PathBuf {
    get_workspace_dir().join("targets")
}

fn read_cargo_manifest(crate_dir: &PathBuf) -> CargoManifest {
    let manifest_raw = read_to_string(crate_dir.join("Cargo.toml")).unwrap();
    let manifest: toml::Value = toml::from_str(&manifest_raw).unwrap();

    let table_package = manifest.get("package").unwrap().as_table().unwrap();
    let no_features = &toml::Value::Table(toml::Table::new());
    let table_features = manifest.get("features").unwrap_or(no_features).as_table().unwrap();

    CargoManifest {
        name: table_package.get("name").unwrap().as_str().unwrap().to_string(),
        features: table_features.keys().map(|k| k.to_string()).collect(),
    }
}

fn read_cargo_config(crate_dir: &PathBuf) -> CargoConfig {
    let config_raw = read_to_string(crate_dir.join(".cargo").join("config.toml")).unwrap();
    let config: toml::Value = toml::from_str(&config_raw).unwrap();

    let table_build = config.get("build").unwrap().as_table().unwrap();

    CargoConfig { target: table_build.get("target").unwrap().as_str().unwrap().to_string() }
}

fn list_features() -> Vec<String> {
    let manifest = read_cargo_manifest(&get_workspace_dir().join("app"));
    manifest.features.iter().map(|f| f.to_string()).filter(|f| f != "default").collect()
}

fn list_targets() -> Vec<Target> {
    let mut targets = read_dir(get_targets_dir())
        .unwrap()
        .filter_map(|entry| {
            let e = entry.unwrap();
            if e.metadata().unwrap().is_dir() {
                let manifest = read_cargo_manifest(&e.path());
                let config = read_cargo_config(&e.path());

                Some(Target {
                    crate_name: manifest.name,
                    rust_target: config.target,
                    features: manifest.features,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    targets.sort();
    targets
}

fn main() {
    let cmd = Command::new("xtask")
        .bin_name("xtask")
        .subcommand_required(true)
        // setup subcommand
        .subcommand(Command::new("setup"))
        // list-targets subcommand
        .subcommand(Command::new("list-targets"))
        .subcommand(Command::new("list-features"))
        // firmware subcommand
        .subcommand(
            Command::new("fw")
                .subcommand_required(true)
                .args([
                    // select one specific target
                    Arg::new("targets")
                        .conflicts_with("all-targets")
                        .long("target")
                        .short('t')
                        .value_parser(ValueParser::new(move |s: &str| {
                            if list_targets()
                                .iter()
                                .any(|target| target.crate_name.to_string() == s.to_string())
                            {
                                Ok(s.to_string())
                            } else {
                                Err(format!("target '{}' does not exist", s))
                            }
                        }))
                        .action(ArgAction::Append)
                        .global(true), // .value_parser(parser_target),
                    // select all targets
                    Arg::new("all-targets")
                        .conflicts_with("targets")
                        .long("all-targets")
                        .action(ArgAction::SetTrue)
                        .global(true),
                    // select one specific feature
                    Arg::new("features")
                        .conflicts_with("all-features")
                        .long("features")
                        .short('f')
                        .value_parser(ValueParser::new(move |s: &str| {
                            if list_features().contains(&s.to_string()) {
                                Ok(s.to_string())
                            } else {
                                Err(format!("feature '{}' does not exist", s))
                            }
                        }))
                        .action(ArgAction::Append)
                        .global(true),
                    // select all features
                    Arg::new("all-features")
                        .conflicts_with("features")
                        .long("all-features")
                        .action(ArgAction::SetTrue)
                        .global(true),
                    // defmt logging level
                    Arg::new("log-level")
                        .long("log-level")
                        .short('l')
                        .value_parser(["trace", "debug", "info", "warn", "error", "off"])
                        .default_value("debug")
                        .default_value_if("release", ArgPredicate::IsPresent, "off")
                        .global(true),
                    // build release version
                    Arg::new("release")
                        .long("release")
                        .short('r')
                        .action(ArgAction::SetTrue)
                        .global(true),
                ])
                .subcommand(Command::new("build"))
                .subcommand(
                    Command::new("flash")
                        .arg(
                            Arg::new("method")
                                .long("method")
                                .short('m')
                                .value_parser(["dfu", "uf2"])
                                .required(true),
                        )
                        .arg(Arg::new("device").long("device").short('d')),
                ),
        );

    // parse first-level subcommand
    match cmd.get_matches().subcommand().unwrap() {
        ("setup", _) => {
            cmd_setup();
        }
        ("list-targets", args) => {
            cmd_list_targets();
        }
        ("list-features", _) => {
            cmd_list_features();
        }
        ("fw", cmd_fw) => {
            let all_targets = list_targets();
            let all_features = list_features();

            let arg_targets: Vec<Target> = match cmd_fw.get_flag("all-targets") {
                true => all_targets,
                false => cmd_fw
                    .get_many::<String>("targets")
                    .into_iter()
                    .flatten()
                    .map(|t| all_targets.iter().find(|a| a.crate_name.to_string() == *t).unwrap())
                    .map(|t| t.to_owned())
                    .collect(),
            };

            let arg_features: Vec<String> = match cmd_fw.get_flag("all-features") {
                true => all_features,
                false => cmd_fw
                    .get_many::<String>("features")
                    .into_iter()
                    .flatten()
                    .map(|f| f.to_owned())
                    .collect(),
            };

            let arg_log_level = cmd_fw.get_one::<String>("log-level").unwrap().to_owned();

            let arg_release = cmd_fw.get_flag("release");

            // parse subcommand of "fw"
            match cmd_fw.subcommand().unwrap() {
                ("build", _) => {
                    for t in &arg_targets {
                        cmd_fw_build(&t, &arg_features, &arg_log_level, arg_release);
                    }
                }
                ("flash", args_flash) => {
                    let device = match args_flash.get_one::<String>("device") {
                        Some(id) => Some(id.to_string()),
                        None => None,
                    };

                    let flash_method =
                        match args_flash.get_one::<String>("method").unwrap().as_str() {
                            "dfu" => FlashMethod::DFU { usb_id: device },
                            "uf2" => FlashMethod::UF2 { path: device.expect("missing device") },
                            _ => unreachable!(),
                        };

                    for t in &arg_targets {
                        let out_file = cmd_fw_build(&t, &arg_features, &arg_log_level, arg_release);
                        cmd_fw_flash(&out_file, &flash_method);
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };
}

fn cmd_setup() {
    let sh = Shell::new().unwrap();
    cmd!(sh, "cargo install cargo-binutils").run().unwrap();
}

fn cmd_list_targets() {
    let targets = list_targets();

    for t in &targets {
        println!("{}", t.crate_name);
    }
}

fn cmd_list_features() {
    for f in &list_features() {
        println!("{}", f);
    }
}

fn cmd_fw_build(
    target: &Target,
    features: &Vec<String>,
    log_level: &String,
    release: bool,
) -> String {
    let manifest_path = get_targets_dir().join(&target.crate_name).join("Cargo.toml");
    let out_file_path =
        format!("{}.bin", &get_workspace_dir().join("out").join(&target.crate_name).display());

    let features_joined = features
        .iter()
        .map(|t| format!("{}/{}", APP_CRATE_NAME, t))
        .collect::<Vec<String>>()
        .join(",");

    let arg_features: &[&str] = match features.is_empty() {
        true => &[],
        false => &["--no-default-features", "true", "--features", features_joined.as_str()],
    };

    let arg_release: &[&str] = match release {
        true => &["--release", "true"],
        false => &[],
    };

    let t = &target.rust_target;

    let sh = Shell::new().unwrap();
    let _ = sh.push_env("DEFMT_LOG", log_level);
    cmd!(sh, "cargo objcopy --manifest-path {manifest_path} --target {t} {arg_features...} {arg_release...} -- -O binary {out_file_path}")
        .run()
        .unwrap();

    out_file_path
}

fn cmd_fw_flash(bin_file: &String, method: &FlashMethod) {
    match method {
        FlashMethod::DFU { usb_id } => {
            let id = usb_id.clone().unwrap_or_else(|| "".to_string());

            let arg_device: &[&str] =
                if usb_id.is_some() { &["--device", id.as_str()] } else { &[] };

            let sh = Shell::new().unwrap();
            cmd!(sh, "dfu-util -a 0 -s 0x08000000:leave {arg_device...} -D {bin_file}")
                .run()
                .unwrap()
        }
        FlashMethod::UF2 { path } => {
            println!("flashing uf2 to: {}", path);
            unimplemented!("Pico UF2 flashing method is not implemented yet");
        }
    }
}
