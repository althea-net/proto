//! Protobuf source files are in the althea-chain repo, this binary copyies the result to the althea_proto crate
//! for import and use. While this builder generates about a dozen files only one contains all the module
//! proto info and the rest are discarded in favor of upstream cosmos-sdk-proto

// Building new Althea rust proto definitions
// run 'cargo run'
// go to althea_proto/prost
// re-write calls to super::super::cosmos as cosmos-sdk-proto::cosmos

use std::{path::{Path, PathBuf}, fs::{self, remove_dir_all, create_dir_all}, ffi::OsStr, io};

use althea::althea_main;
use regex::Regex;
use walkdir::WalkDir;

pub mod althea;

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto` or `cosmos-sdk-proto`)
pub const EXCLUDED_PROTO_PACKAGES: &[&str] = &[
    "gogoproto",
    "google",
    "tendermint",
    "cosmos.base",
    "cosmos_proto",
];
/// Attribute preceeding a Tonic client definition
pub const TONIC_CLIENT_ATTRIBUTE: &str = "#[doc = r\" Generated client implementations.\"]";
/// Attributes to add to gRPC clients
pub const GRPC_CLIENT_ATTRIBUTES: &[&str] = &[
    "#[cfg(feature = \"grpc\")]",
    "#[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
    TONIC_CLIENT_ATTRIBUTE,
];
/// Regex for locating instances of `cosmos-sdk-proto` in prost/tonic build output
pub const COSMOS_SDK_PROTO_REGEX: &str = "(super::)+cosmos";

/// Root directories of the managed projects
pub const ALTHEA_ROOT: &str = "../althea-chain/";
pub const COSMOS_SDK_ROOT: &str = "../cosmos-sdk/";
pub const GRAVITY_ROOT: &str = "../gravity/";
/// A temporary directory for proto building
pub const TMP_PATH: &str = "/tmp/proto/";
/// the output directory
pub const ALTHEA_OUT_PATH: &str = "../althea_proto/src/prost/";
pub const COSMOS_OUT_PATH: &str = "../cosmos_sdk_proto/src/prost/";
pub const GRAVITY_OUT_PATH: &str = "../gravity_proto/src/prost/";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.
pub struct ProtoDirectories {
    pub root: String,
    pub tmp: String,
    pub out_dir: String,
}

fn main() {
    // Initiate Althea
    althea_main(&ALTHEA_ROOT, &TMP_PATH, &ALTHEA_OUT_PATH);
    // Initiate Cosmos SDK

    // Initiate Gravity
    
    // let out_path = Path::new(&OUT_PATH);
    // let tmp_path = Path::new(&TMP_PATH);
    // compile_protos(out_path, tmp_path);
}

fn compile_protos(proto_paths: &[PathBuf], proto_include_paths: &[PathBuf], tmp_path: &Path, out_path: &Path) {
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
    println!("[info ] Compiling proto clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(tmp_path)
        .compile(&protos, &proto_include_paths)
        .unwrap();

    copy_generated_files(tmp_path, out_path);

    println!("[info ] => Done!");
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    println!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(to_dir).unwrap_or_default();
    create_dir_all(to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let contents = fs::read_to_string(src)?;

    // `prost-build` output references types from `tendermint-proto` crate
    // relative paths, which we need to munge into `tendermint_proto` in
    // order to leverage types from the upstream crate.
    let contents = Regex::new(COSMOS_SDK_PROTO_REGEX)
        .unwrap()
        .replace_all(&contents, "cosmos_sdk_proto::cosmos");

    // Patch each service definition with a feature attribute
    let patched_contents =
        contents.replace(TONIC_CLIENT_ATTRIBUTE, &GRPC_CLIENT_ATTRIBUTES.join("\n"));

    fs::write(dest, patched_contents)
}
