fn main() {
    tonic_build::configure()
        // .type_attribute("tttt.ColorPoint", "#[derive(Default)]")
        .build_client(false)
        .compile(&["../proto/test.proto"], &["../proto"])
        .unwrap();
}
