//! Protobuf source files are in the gravity repo's module folder, this binary copyies the result to the gravity_proto crate
//! for import and use. This builder generates about a dozen files, but only one contains all the module
//! proto info and the rest are discarded in favor of upstream cosmos-sdk-proto

use crate::{
    compile_protos, RegexReplace, COSMOS_SDK_PROTO_CRATE_REGEX_REPLACE, GOOGLE_COMMON_ROOT,
};
use std::path::Path;

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto` or `cosmos-sdk-proto`)
pub const EXCLUDED_PROTO_PACKAGES: &[&str] = &[
    "gogoproto",
    "google",
    "cosmos_proto",
    "cosmos",
    "tendermint",
];

/// Compiles all the protos for the gravity-bridge project, including the upstream dependencies from canto and evmos
pub fn gravity_main(root: &str, tmp: &str, out: &str) {
    // Regex fixes for super::[super::, ...]cosmos
    let regex_replacements: Vec<RegexReplace> = vec![COSMOS_SDK_PROTO_CRATE_REGEX_REPLACE];

    let root_path = Path::new(root);
    let tmp_path = Path::new(tmp);
    let out_path = Path::new(out);
    compile_gravity_protos(root_path, tmp_path, out_path, &regex_replacements);
}

/// Compiles the test protos for the gravity-bridge project
pub fn gravity_test(root: &str, tmp: &str, out: &str) {
    // Regex fixes for super::[super::, ...]cosmos
    let regex_replacements: Vec<RegexReplace> = vec![COSMOS_SDK_PROTO_CRATE_REGEX_REPLACE];

    let root_path = Path::new(root);
    let tmp_path = Path::new(tmp);
    let out_path = Path::new(out);
    compile_gravity_test_protos(root_path, tmp_path, out_path, &regex_replacements);
}

// Aggregates all of the directories needed for protoc to compile the Althea protos + upstream proto dependencies
fn compile_gravity_protos(
    root_path: &Path,
    tmp_path: &Path,
    out_path: &Path,
    regex_replacements: &[RegexReplace],
) {
    info!(
        "[info] Compiling .proto files to Rust into '{}'...",
        out_path.display()
    );

    let root = root_path.to_path_buf();

    let gravity_proto_paths = [
        &format!("{}module/proto/gravity/v1", root.display()),
        &format!("{}module/proto/auction/v1", root.display()),
    ]
    .map(Path::new)
    .map(Path::to_path_buf);
    let mut gravity_proto_include_dir = root.clone();
    gravity_proto_include_dir.push("module/proto");
    let mut third_party_proto_include_dir = root;
    third_party_proto_include_dir.push("module/third_party/proto");

    // we need to have an include which is just the folder of our protos to satisfy protoc
    // which insists that any passed file be included in a directory passed as an include
    let proto_include_paths = [
        gravity_proto_include_dir,
        GOOGLE_COMMON_ROOT.into(),
        third_party_proto_include_dir,
    ];

    compile_protos(
        &gravity_proto_paths,
        &proto_include_paths,
        regex_replacements,
        EXCLUDED_PROTO_PACKAGES,
        tmp_path,
        out_path,
        true,
        true,
    );
}

// Compiles the ibc-test-chain protos for Gravity testing use
fn compile_gravity_test_protos(
    root_path: &Path,
    tmp_path: &Path,
    out_path: &Path,
    regex_replacements: &[RegexReplace],
) {
    info!(
        "[info] Compiling .proto files to Rust into '{}'...",
        out_path.display()
    );

    let root = root_path.to_path_buf();

    let proto_paths = [
        &format!("{}proto/gaia/globalfee", root.display()),
        &format!("{}proto/gaia/icaauth", root.display()),
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

    compile_protos(
        &proto_paths,
        &proto_include_paths,
        regex_replacements,
        EXCLUDED_PROTO_PACKAGES,
        tmp_path,
        out_path,
        false,
        false,
    );
}
