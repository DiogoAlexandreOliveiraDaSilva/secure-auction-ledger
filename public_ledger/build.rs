// build.rs
fn main() {
    tonic_build::configure()
    .compile_protos(&["src/proto/communication.proto"], &["src/proto"])
    .expect("Failed to compile proto files");
}
