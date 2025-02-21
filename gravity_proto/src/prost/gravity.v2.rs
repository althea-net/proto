#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutgoingTransferAndBatch {
    #[prost(message, optional, tag = "1")]
    pub transfer: ::core::option::Option<super::v1::OutgoingTransferTx>,
    #[prost(message, optional, tag = "2")]
    pub batch: ::core::option::Option<super::v1::OutgoingTxBatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEthV2 {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEthV2Response {
    #[prost(message, repeated, tag = "1")]
    pub transfers_in_batches: ::prost::alloc::vec::Vec<OutgoingTransferAndBatch>,
    #[prost(message, repeated, tag = "2")]
    pub unbatched_transfers: ::prost::alloc::vec::Vec<super::v1::OutgoingTransferTx>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEthV2BySender {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSendToEthV2BySenderResponse {
    #[prost(message, repeated, tag = "1")]
    pub transfers_in_batches: ::prost::alloc::vec::Vec<OutgoingTransferAndBatch>,
    #[prost(message, repeated, tag = "2")]
    pub unbatched_transfers: ::prost::alloc::vec::Vec<super::v1::OutgoingTransferTx>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingTxBatchesByAddrRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutgoingTxBatchesByAddrResponse {
    #[prost(message, repeated, tag = "1")]
    pub batches: ::prost::alloc::vec::Vec<super::v1::OutgoingTxBatch>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
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
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_pending_send_to_eth_v2(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPendingSendToEthV2>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPendingSendToEthV2Response>,
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
            let path = http::uri::PathAndQuery::from_static(
                "/gravity.v2.Query/GetPendingSendToEthV2",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gravity.v2.Query", "GetPendingSendToEthV2"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_pending_send_to_eth_v2_by_sender(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPendingSendToEthV2BySender>,
        ) -> std::result::Result<
            tonic::Response<super::QueryPendingSendToEthV2BySenderResponse>,
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
            let path = http::uri::PathAndQuery::from_static(
                "/gravity.v2.Query/GetPendingSendToEthV2BySender",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("gravity.v2.Query", "GetPendingSendToEthV2BySender"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_outgoing_tx_batches_by_addr(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOutgoingTxBatchesByAddrRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryOutgoingTxBatchesByAddrResponse>,
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
            let path = http::uri::PathAndQuery::from_static(
                "/gravity.v2.Query/GetOutgoingTxBatchesByAddr",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("gravity.v2.Query", "GetOutgoingTxBatchesByAddr"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
