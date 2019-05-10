// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait ExampleService {
    fn add(&self, o: ::grpc::RequestOptions, p: super::example::AddRequest) -> ::grpc::SingleResponse<super::example::AddResponse>;
}

// client

pub struct ExampleServiceClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Add: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::example::AddRequest, super::example::AddResponse>>,
}

impl ::grpc::ClientStub for ExampleServiceClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        ExampleServiceClient {
            grpc_client: grpc_client,
            method_Add: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/netsaj.ExampleService/Add".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl ExampleService for ExampleServiceClient {
    fn add(&self, o: ::grpc::RequestOptions, p: super::example::AddRequest) -> ::grpc::SingleResponse<super::example::AddResponse> {
        self.grpc_client.call_unary(o, p, self.method_Add.clone())
    }
}

// server

pub struct ExampleServiceServer;


impl ExampleServiceServer {
    pub fn new_service_def<H : ExampleService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/netsaj.ExampleService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/netsaj.ExampleService/Add".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add(o, p))
                    },
                ),
            ],
        )
    }
}
