fn main() {
    cynic_codegen::register_schema("allanime")
        .from_sdl_file("schemas/allanime.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
