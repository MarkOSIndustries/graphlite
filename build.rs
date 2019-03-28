use tower_grpc_build;

fn main() {
    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(false)
        .build(&["proto/graphlite_protocol.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}