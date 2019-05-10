# rust example grpc protobuf

## dependencies
* protoc compiler for your OS

* cargo install grpc-compiler
* cargo install protobuf-codegen


## generate .rs from proto

```bash
$ protoc --rust_out=compiled/rust/ proto/example.proto
```
 
```bash
$ protoc --rust-grpc_out=compiled/rust/ proto/example.proto
```


