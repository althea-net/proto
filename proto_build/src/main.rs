//! Protobuf source files are in the althea-chain repo, this binary copyies the result to the althea_proto crate
//! for import and use. While this builder generates about a dozen files only one contains all the module
//! proto info and the rest are discarded in favor of upstream cosmos-sdk-proto

// Building new Althea rust proto definitions
// run 'cargo run'
// go to althea_proto/prost
// re-write calls to super::super::cosmos as cosmos-sdk-proto::cosmos

use std::{path::{Path, PathBuf}, fs::{self, remove_dir_all, create_dir_all}, ffi::OsStr, io, borrow::Cow};

use althea::althea_main;
use cosmos_sdk::{RootDirs, cosmos_main};
use env_logger::Env;
use gravity::gravity_main;
use regex::Regex;
use walkdir::WalkDir;

pub mod althea;
pub mod cosmos_sdk;
pub mod gravity;

#[macro_use]
extern crate log;

/// Attribute preceeding a Tonic client definition
pub const TONIC_CLIENT_ATTRIBUTE: &str = "#[doc = r\" Generated client implementations.\"]";
/// Attributes to add to gRPC clients
pub const GRPC_CLIENT_ATTRIBUTES: &[&str] = &[
    "#[cfg(feature = \"grpc\")]",
    "#[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
    TONIC_CLIENT_ATTRIBUTE,
];
/// Root directories of the managed projects
pub const ALTHEA_ROOT: &str = "../althea-chain/";

pub const COSMOS_SDK_ROOT: &str = "../cosmos-sdk/";
pub const BECH32IBC_ROOT: &str = "../bech32-ibc/";
pub const TENDERMINT_ROOT: &str = "../tendermint/";
pub const IBC_ROOT: &str = "../ibc-go/";

pub const GRAVITY_ROOT: &str = "../gravity/";

/// A temporary directory for proto building
pub const TMP_PATH: &str = "/tmp/proto/";
/// the output directory
pub const ALTHEA_OUT_PATH: &str = "../althea_proto/src/prost/";
pub const COSMOS_OUT_PATH: &str = "../cosmos_sdk_proto/src/prost/";
pub const GRAVITY_OUT_PATH: &str = "../gravity_proto/src/prost/";

pub struct RegexReplace {
    pub regex: Cow<'static, str>, // A regular expression to match faulty generated code with
    pub replace: Cow<'static, str>, // The string to replace the faulty code with
}
impl RegexReplace {
    pub const fn new(regex: &'static str, replace: &'static str) -> Self {
        RegexReplace{regex: Cow::Borrowed(regex), replace: Cow::Borrowed(replace)}
    }
}
pub const COSMOS_SDK_PROTO_IMPORT_REGEX: &str = "(super::)+cosmos";
pub const COSMOS_SDK_PROTO_CRATE_REPLACE: &str = "cosmos_sdk_proto::cosmos";
pub const COSMOS_SDK_PROTO_CRATE_REGEX_REPLACE: RegexReplace =
    RegexReplace::new(COSMOS_SDK_PROTO_IMPORT_REGEX, COSMOS_SDK_PROTO_CRATE_REPLACE);

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Initiate Althea
    althea_main(&ALTHEA_ROOT, &TMP_PATH, &ALTHEA_OUT_PATH);
    // Initiate Cosmos SDK
    cosmos_main(
        RootDirs{cosmos:COSMOS_SDK_ROOT.to_string(), bech32ibc:BECH32IBC_ROOT.to_string(), tendermint:TENDERMINT_ROOT.to_string(), ibc:IBC_ROOT.to_string()},
        &TMP_PATH,
        &COSMOS_OUT_PATH,
    );
    // Initiate Gravity
    gravity_main(&GRAVITY_ROOT, &TMP_PATH, &GRAVITY_OUT_PATH);
}

fn compile_protos(proto_paths: &[PathBuf], proto_include_paths: &[PathBuf], replacements: &[RegexReplace], exclusions: &[&str], tmp_path: &Path, out_path: &Path, clean_tmp: bool, clean_out: bool) {
    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }

    // create directories for temporary build dirs
    fs::create_dir_all(tmp_path)
        .unwrap_or_else(|_| panic!("Failed to create {:?}", tmp_path.to_str()));

    // Compile all proto files
    let mut config = prost_build::Config::default();
    config.out_dir(tmp_path);
    config
        .compile_protos(&protos, &proto_include_paths)
        .unwrap();

    // Compile all proto client for GRPC services
    info!("[info ] Compiling proto clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(tmp_path)
        .compile(&protos, &proto_include_paths)
        .unwrap();

    copy_generated_files(tmp_path, out_path, replacements, exclusions, clean_tmp, clean_out);

    info!("[info ] => Done!");
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path, replacements: &[RegexReplace], exclusions: &[&str], clean_from: bool, clean_to: bool) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    if clean_to {
        remove_dir_all(to_dir).unwrap_or_default();
        create_dir_all(to_dir).unwrap();
    }

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(replacements, exclusions, e.path(), Path::new(&format!("{}/{}", to_dir.display(), &filename)))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }

    if clean_from {
        remove_dir_all(from_dir).unwrap_or_default();
        create_dir_all(from_dir).unwrap();
    }
}

fn copy_and_patch(replacements: &[RegexReplace], exclusions: &[&str], src: &Path, dest: &Path) -> io::Result<()> {
    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in exclusions {
        if let Some(filename) = src.file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = fs::read_to_string(src)?;

    // In plenty of situations we have files which rely on upstream dependencies or simply
    // misreferenced files in the same crate, use the replacements to modify the contents
    for replace in replacements {
        contents = Regex::new(&replace.regex).unwrap().replace_all(&contents, &replace.replace).to_string();
    }
    // Patch each service definition with a feature attribute
    let patched_contents =
        contents.replace(TONIC_CLIENT_ATTRIBUTE, &GRPC_CLIENT_ATTRIBUTES.join("\n"));

    fs::write(dest, patched_contents)
}
