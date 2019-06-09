use capnpc::{CompilerCommand, RustEdition};

fn main() {
    CompilerCommand::new()
        .src_prefix("src/schema")
        .file("src/schema/grid_world.capnp")
        .edition(RustEdition::Rust2018)
        .import_path("src")
        .run()
        .unwrap();
}
