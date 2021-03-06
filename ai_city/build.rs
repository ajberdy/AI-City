use capnpc::{CompilerCommand, RustEdition};

fn main() {
    CompilerCommand::new()
        .src_prefix("../schema")
        .file("../schema/grid_world.capnp")
        .file("../schema/city_server.capnp")
        .edition(RustEdition::Rust2018)
        .import_path("src")
        .run()
        .unwrap();
}
