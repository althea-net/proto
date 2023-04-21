//! Protobuf source files are in the althea-chain repo, this binary copyies the result to the althea_proto crate
//! for import and use. While this builder generates about a dozen files only one contains all the module
//! proto info and the rest are discarded in favor of upstream cosmos-sdk-proto

// Building new Althea rust proto definitions
// run 'cargo run'
// go to althea_proto/prost
// re-write calls to super::super::cosmos as cosmos-sdk-proto::cosmos

use std::{path::Path};
use crate::{compile_protos, RegexReplace, COSMOS_SDK_PROTO_CRATE_REGEX_REPLACE};

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are generated in 'cosmos-sdk-proto`)
pub const EXCLUDED_PROTO_PACKAGES: &[&'static str] = &[
    "gogoproto",
    "google",
    "cosmos_proto",
    "cosmos",
    "tendermint",
];

/// Compiles all the protos for the althea-chain project, including the upstream dependencies from canto and evmos
pub fn althea_main(root: &str, tmp: &str, out: &str) {
    // Regex fixes for super::[super::, ...]cosmos
    let regex_replacements: Vec<RegexReplace> = vec![COSMOS_SDK_PROTO_CRATE_REGEX_REPLACE];

    let root_path = Path::new(root);
    let tmp_path = Path::new(tmp);
    let out_path = Path::new(out);
    compile_althea_protos(root_path, tmp_path, out_path, &regex_replacements);
}

// Aggregates all of the directories needed for protoc to compile the Althea protos + upstream proto dependencies
fn compile_althea_protos(root_path: &Path, tmp_path: &Path, out_path: &Path, regex_replacements: &[RegexReplace]) {
    info!(
        "[info] Compiling .proto files to Rust into '{}'...",
        out_path.display()
    );

    let root = root_path.to_path_buf();

    // Althea modules which should be compiled
    let mut lockup_proto_dir = root.clone();
    lockup_proto_dir.push("proto/lockup/v1/");
    let mut microtx_proto_dir = root.clone();
    microtx_proto_dir.push("proto/microtx/v1/");

    // Protobuf requires the root of the above modules + any third party deps to be included
    let mut althea_proto_include_dir = root.clone();
    althea_proto_include_dir.push("proto/");
    let mut third_party_proto_include_dir = root.clone();
    third_party_proto_include_dir.push("third_party/proto");

    // TODO: Generate these in their own crates
    // Third party protos which are not maintained
    let mut ethermint_proto_dir = root.clone();
    ethermint_proto_dir.push("third_party/proto/ethermint");
    let mut canto_proto_dir = root;
    canto_proto_dir.push("third_party/proto/canto");

    // Paths
    let proto_paths = [
        lockup_proto_dir,
        microtx_proto_dir,
        ethermint_proto_dir,
        canto_proto_dir,
    ];
    // we need to have an include which is just the folder of our protos to satisfy protoc
    // which insists that any passed file be included in a directory passed as an include
    let proto_include_paths = [althea_proto_include_dir, third_party_proto_include_dir];

    compile_protos(&proto_paths, &proto_include_paths, regex_replacements, EXCLUDED_PROTO_PACKAGES, tmp_path, out_path, true, true);
}