fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .input("proto/account.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");

}