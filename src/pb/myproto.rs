#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KvPair {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<Key>,
    #[prost(message, optional, tag="2")]
    pub val: ::core::option::Option<Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(string, tag="1")]
    pub val: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyState {
    #[prost(string, tag="1")]
    pub reply_info: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub kvpair: ::core::option::Option<KvPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestState {
    #[prost(string, tag="1")]
    pub request_info: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod my_rpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MyRpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MyRpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MyRpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MyRpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MyRpcClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        ///simple rpc
        pub async fn set_kv(
            &mut self,
            request: impl tonic::IntoRequest<super::KvPair>,
        ) -> Result<tonic::Response<super::ReplyState>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/myproto.MyRpc/SetKv");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_kv(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<tonic::Response<super::ReplyState>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/myproto.MyRpc/GetKv");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// A server-to-client streaming RPC.
        pub async fn get_kv_list(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestState>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::KvPair>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/myproto.MyRpc/GetKvList");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// A client-to-server streaming RPC.
        pub async fn set_kv_list(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::KvPair>,
        ) -> Result<tonic::Response<super::ReplyState>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/myproto.MyRpc/SetKvList");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod my_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MyRpcServer.
    #[async_trait]
    pub trait MyRpc: Send + Sync + 'static {
        ///simple rpc
        async fn set_kv(
            &self,
            request: tonic::Request<super::KvPair>,
        ) -> Result<tonic::Response<super::ReplyState>, tonic::Status>;
        async fn get_kv(
            &self,
            request: tonic::Request<super::Key>,
        ) -> Result<tonic::Response<super::ReplyState>, tonic::Status>;
        ///Server streaming response type for the GetKvList method.
        type GetKvListStream: futures_core::Stream<
                Item = Result<super::KvPair, tonic::Status>,
            >
            + Send
            + 'static;
        /// A server-to-client streaming RPC.
        async fn get_kv_list(
            &self,
            request: tonic::Request<super::RequestState>,
        ) -> Result<tonic::Response<Self::GetKvListStream>, tonic::Status>;
        /// A client-to-server streaming RPC.
        async fn set_kv_list(
            &self,
            request: tonic::Request<tonic::Streaming<super::KvPair>>,
        ) -> Result<tonic::Response<super::ReplyState>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MyRpcServer<T: MyRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MyRpc> MyRpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MyRpcServer<T>
    where
        T: MyRpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/myproto.MyRpc/SetKv" => {
                    #[allow(non_camel_case_types)]
                    struct SetKvSvc<T: MyRpc>(pub Arc<T>);
                    impl<T: MyRpc> tonic::server::UnaryService<super::KvPair>
                    for SetKvSvc<T> {
                        type Response = super::ReplyState;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KvPair>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_kv(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetKvSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/myproto.MyRpc/GetKv" => {
                    #[allow(non_camel_case_types)]
                    struct GetKvSvc<T: MyRpc>(pub Arc<T>);
                    impl<T: MyRpc> tonic::server::UnaryService<super::Key>
                    for GetKvSvc<T> {
                        type Response = super::ReplyState;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Key>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_kv(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetKvSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/myproto.MyRpc/GetKvList" => {
                    #[allow(non_camel_case_types)]
                    struct GetKvListSvc<T: MyRpc>(pub Arc<T>);
                    impl<
                        T: MyRpc,
                    > tonic::server::ServerStreamingService<super::RequestState>
                    for GetKvListSvc<T> {
                        type Response = super::KvPair;
                        type ResponseStream = T::GetKvListStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestState>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_kv_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetKvListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/myproto.MyRpc/SetKvList" => {
                    #[allow(non_camel_case_types)]
                    struct SetKvListSvc<T: MyRpc>(pub Arc<T>);
                    impl<T: MyRpc> tonic::server::ClientStreamingService<super::KvPair>
                    for SetKvListSvc<T> {
                        type Response = super::ReplyState;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::KvPair>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_kv_list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetKvListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: MyRpc> Clone for MyRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MyRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MyRpc> tonic::transport::NamedService for MyRpcServer<T> {
        const NAME: &'static str = "myproto.MyRpc";
    }
}
