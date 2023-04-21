//! Protobuf source files are in the gravity repo's module folder, this binary copyies the result to the gravity_proto crate
//! for import and use. This builder generates about a dozen files, but only one contains all the module
//! proto info and the rest are discarded in favor of upstream cosmos-sdk-proto

use std::{path::Path};
use crate::compile_protos;

/// Compiles all the protos for the althea-chain project, including the upstream dependencies from canto and evmos
pub fn gravity_main(root: &str, tmp: &str, out: &str) {
    let root_path = Path::new(root);
    let tmp_path = Path::new(tmp);
    let out_path = Path::new(out);
    compile_gravity_protos(root_path, tmp_path, out_path);
}

// Aggregates all of the directories needed for protoc to compile the Althea protos + upstream proto dependencies
fn compile_gravity_protos(root_path: &Path, tmp_path: &Path, out_path: &Path) {
    info!(
        "[info] Compiling .proto files to Rust into '{}'...",
        out_path.display()
    );

    let root = root_path.to_path_buf();

    let mut gravity_proto_dir = root.clone();
    gravity_proto_dir.push("module/proto/gravity/v1");
    let mut gravity_proto_include_dir = root.clone();
    gravity_proto_include_dir.push("module/proto");
    let mut third_party_proto_include_dir = root;
    third_party_proto_include_dir.push("module/third_party/proto");

    let proto_paths = [gravity_proto_dir];
    // we need to have an include which is just the folder of our protos to satisfy protoc
    // which insists that any passed file be included in a directory passed as an include
    let proto_include_paths = [gravity_proto_include_dir, third_party_proto_include_dir];

    compile_protos(&proto_paths, &proto_include_paths, tmp_path, out_path, true, true);
}