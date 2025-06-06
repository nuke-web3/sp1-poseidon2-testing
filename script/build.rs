use sp1_build::{build_program_with_args, BuildArgs};

fn main() {
    // Greatly reduces cycle count when executing the zkVM program
    // Also reflected in program/Cargo.toml
    let args = BuildArgs {
        rustflags: vec![
            "-Copt-level=3".into(),
            "-Cembed-bitcode=yes".into(), // Resolves conflict with lto use
            "-Clto=fat".into(),
            "-Ccodegen-units=1".into(),
            "-Cdebuginfo=1".into(),
        ],
        ..Default::default()
    };

    build_program_with_args("../program", args);
}
