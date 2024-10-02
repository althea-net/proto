//! Note: The following code was largely taken from github.com/cosmos/cosmos-rust/cosmos-sdk-proto and modified for
//! use in Gravity Bridge and Althea-Chain.
//! Some unneded features have been removed, like auto checkout and wasm proto compliation.
//!
//! Build CosmosSDK/Tendermint/IBC proto files. This build script clones the CosmosSDK version
//! specified in the COSMOS_SDK_REV constant and then uses that to build the required
//! proto files for further compilation. This is based on the proto-compiler code
//! in github.com/informalsystems/ibc-rs

use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::{compile_protos, copy_generated_files, RegexReplace, GOOGLE_COMMON_ROOT};

pub struct RootDirs {
    pub cosmos: String,
    pub bech32ibc: String,
    pub tendermint: String,
    pub ibc: String,
}

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto` or `cosmos-sdk-proto`)
pub const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "cosmos_proto"];
/// Regex fixes for super::[super::, ...]cosmos and similarly for tendermint
pub const COSMOS_SDK_PROTO_REGEX: &str = "(super::)+cosmos";
pub const COSMOS_SDK_PROTO_REPLACE: &str = "crate::cosmos";
pub const TENDERMINT_PROTO_REGEX: &str = "(super::)+tendermint";
pub const TENDERMINT_PROTO_REPLACE: &str = "crate::tendermint";

pub const COSMOS_REGEX_REPLACE: RegexReplace =
    RegexReplace::new(COSMOS_SDK_PROTO_REGEX, COSMOS_SDK_PROTO_REPLACE);
pub const TENDERMINT_REGEX_REPLACE: RegexReplace =
    RegexReplace::new(TENDERMINT_PROTO_REGEX, TENDERMINT_PROTO_REPLACE);

/// Initiates the compilation of the cosmos-sdk, tendermint, ibc, and bech32-ibc protos
pub fn cosmos_main(roots: RootDirs, tmp_dir: &str, out_dir: &str) {
    // Make sure that imports are fixed when compiling
    let regex_replacements = vec![COSMOS_REGEX_REPLACE, TENDERMINT_REGEX_REPLACE];
    // Note that this order is very important, as any project with the potential to clobber compiled rust files will clobber another project
    // this can easily be avoided by splitting each project into its own directory + crate, but for historical reasons it is being left in
    // the manner cosmos-sdk-proto performs this work (for now)

    // TODO: Split each project off into its own compilation directory + crate
    compile_bech32ibc_protos_and_services(
        Path::new(&roots.bech32ibc),
        Path::new(tmp_dir),
        Path::new(out_dir),
        &regex_replacements,
    );
    compile_ibc_protos_and_services(
        Path::new(&roots.ibc),
        Path::new(tmp_dir),
        Path::new(out_dir),
        &regex_replacements,
    );
    compile_sdk_protos_and_services(
        Path::new(&roots.cosmos),
        Path::new(tmp_dir),
        Path::new(out_dir),
        &regex_replacements,
    );
    compile_tendermint_protos_and_services(
        Path::new(&roots.tendermint),
        Path::new(tmp_dir),
        Path::new(out_dir),
        &regex_replacements,
    );
}

fn compile_sdk_protos_and_services(
    root: &Path,
    tmp_path: &Path,
    out_path: &Path,
    regex_replacements: &[RegexReplace],
) {
    info!(
        "Compiling cosmos-sdk .proto files to Rust into '{}'...",
        out_path.display(),
    );

    // Paths
    let proto_paths = [
        &format!("{}proto/cosmos/auth", root.display()),
        &format!("{}proto/cosmos/authz", root.display()),
        &format!("{}proto/cosmos/bank", root.display()),
        &format!("{}proto/cosmos/base", root.display()),
        &format!("{}proto/cosmos/base/abci", root.display()),
        &format!("{}proto/cosmos/base/tendermint", root.display()),
        &format!("{}proto/cosmos/capability", root.display()),
        &format!("{}proto/cosmos/crisis", root.display()),
        &format!("{}proto/cosmos/crypto", root.display()),
        &format!("{}proto/cosmos/distribution", root.display()),
        &format!("{}proto/cosmos/evidence", root.display()),
        &format!("{}proto/cosmos/feegrant", root.display()),
        &format!("{}proto/cosmos/genutil", root.display()),
        &format!("{}proto/cosmos/gov", root.display()),
        &format!("{}proto/cosmos/mint", root.display()),
        &format!("{}proto/cosmos/params", root.display()),
        &format!("{}proto/cosmos/slashing", root.display()),
        &format!("{}proto/cosmos/staking", root.display()),
        &format!("{}proto/cosmos/tx", root.display()),
        &format!("{}proto/cosmos/upgrade", root.display()),
        &format!("{}proto/cosmos/vesting", root.display()),
    ]
    .map(Path::new)
    .map(Path::to_path_buf);

    let proto_include_paths = [
        &format!("{}proto", root.display()),
        &GOOGLE_COMMON_ROOT.to_string(),
        // Cosmos sdk removed their third party proto directory
        // so now we use a copy in this repo
        "../third_party/proto",
    ]
    .map(Path::new)
    .map(Path::to_path_buf);

    compile_protos(crate::CompileArgs {
        proto_paths: &proto_paths,
        proto_include_paths: &proto_include_paths,
        replacements: regex_replacements,
        exclusions: EXCLUDED_PROTO_PACKAGES,
        tmp_path,
        out_path,
        clean_tmp: false,
        clean_out: false,
    });

    info!("=> Done!");
}

fn compile_tendermint_protos_and_services(
    root: &Path,
    tmp_path: &Path,
    out_path: &Path,
    regex_replacements: &[RegexReplace],
) {
    info!(
        "Compiling tendermint .proto files to Rust into '{}'...",
        out_path.display()
    );

    // paths for only proto generation, these can not be combined
    // because the service generator writes to the same files and will
    // not create struct definitions if there are no services
    // folders here will have all protos in them compiled
    let proto_paths = [
        &format!("{}proto/tendermint/abci", root.display()),
        &format!("{}proto/tendermint/blockchain", root.display()),
        &format!("{}proto/tendermint/consensus", root.display()),
        &format!("{}proto/tendermint/crypto", root.display()),
        &format!("{}proto/tendermint/libs/bits", root.display()),
        &format!("{}proto/tendermint/mempool", root.display()),
        &format!("{}proto/tendermint/p2p", root.display()),
        &format!("{}proto/tendermint/privval", root.display()),
        &format!("{}proto/tendermint/rpc/grpc", root.display()),
        &format!("{}proto/tendermint/state", root.display()),
        &format!("{}proto/tendermint/statesync", root.display()),
        &format!("{}proto/tendermint/store", root.display()),
        &format!("{}proto/tendermint/types", root.display()),
        &format!("{}proto/tendermint/version", root.display()),
    ]
    .map(Path::new)
    .map(Path::to_path_buf);

    let proto_include_paths = [
        &format!("{}proto", root.display()),
        &GOOGLE_COMMON_ROOT.to_string(),
        &format!("{}proto/tendermint", root.display()),
        &format!("{}third_party/proto", root.display()),
    ]
    .map(Path::new)
    .map(Path::to_path_buf);

    // Tendermint protos are clobbered by the tonic_build grpc step, so we do a custom compilation
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

    copy_generated_files(
        tmp_path,
        out_path,
        regex_replacements,
        EXCLUDED_PROTO_PACKAGES,
        true,
        false,
    );

    info!("[info ] => Done!");
}

fn compile_ibc_protos_and_services(
    root: &Path,
    tmp_path: &Path,
    out_path: &Path,
    regex_replacements: &[RegexReplace],
) {
    info!(
        "Compiling ibc .proto files to Rust into '{}'...",
        out_path.display()
    );

    let proto_paths = [
        &format!("{}proto/ibc/applications/transfer", root.display()),
        &format!(
            "{}proto/ibc/applications/interchain_accounts",
            root.display()
        ),
        &format!("{}proto/ibc/core/channel", root.display()),
        &format!("{}proto/ibc/core/client", root.display()),
        &format!("{}proto/ibc/core/commitment", root.display()),
        &format!("{}proto/ibc/core/connection", root.display()),
        &format!("{}proto/ibc/core/port", root.display()),
        &format!("{}proto/ibc/core/types", root.display()),
        &format!("{}proto/ibc/lightclients/localhost", root.display()),
        &format!("{}proto/ibc/lightclients/solomachine", root.display()),
        &format!("{}proto/ibc/lightclients/tendermint", root.display()),
    ]
    .map(Path::new)
    .map(Path::to_path_buf);

    let proto_include_paths = [
        &format!("{}proto", root.display()),
        &GOOGLE_COMMON_ROOT.to_string(),
        &format!("{}third_party/proto", root.display()),
    ]
    .map(Path::new)
    .map(Path::to_path_buf);

    compile_protos(crate::CompileArgs {
        proto_paths: &proto_paths,
        proto_include_paths: &proto_include_paths,
        replacements: regex_replacements,
        exclusions: EXCLUDED_PROTO_PACKAGES,
        tmp_path,
        out_path,
        clean_tmp: true,
        clean_out: false,
    });
}

fn compile_bech32ibc_protos_and_services(
    root: &Path,
    tmp_path: &Path,
    out_path: &Path,
    regex_replacements: &[RegexReplace],
) {
    info!(
        "Compiling bech32-ibc .proto files to Rust into '{}'...",
        out_path.display(),
    );

    // Paths
    let proto_paths = [&format!("{}proto/bech32ibc/", root.display())]
        .map(Path::new)
        .map(Path::to_path_buf);

    let proto_include_paths = [
        &format!("{}proto", root.display()),
        &GOOGLE_COMMON_ROOT.to_string(),
        &format!("{}third_party/proto", root.display()),
        &format!("{}proto/bech32ibc", root.display()),
    ]
    .map(Path::new)
    .map(Path::to_path_buf);

    compile_protos(crate::CompileArgs {
        proto_paths: &proto_paths,
        proto_include_paths: &proto_include_paths,
        replacements: regex_replacements,
        exclusions: EXCLUDED_PROTO_PACKAGES,
        tmp_path,
        out_path,
        clean_tmp: true,
        clean_out: false,
    });
}
