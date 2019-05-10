extern crate protobuf;
extern crate grpc;
#[path = "../compiled/rust/example.rs"] // Here
mod example;
#[path = "../compiled/rust/example_grpc.rs"] // Here
mod example_grpc;