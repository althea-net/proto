/// Params struct
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag="1")]
    pub xfer_fee_basis_points: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgXfer A Msg meant to send funds from one Althea network wallet to another,
/// via an automated device.
/// SENDER The account sending funds to receiver, must also be the signer of the
/// message
/// RECEIVER The account receiving funds from sender
/// AMOUNTS The tokens and their quantities which should be transferred
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgXfer {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub amounts: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgXferResponse {
}
/// A type for the block's event log, every successful Xfer must create one of
/// these in the event log
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventXfer {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub amounts: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag="4")]
    pub fee: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// Records critical information about a Tokenized Account
/// TOKENIZED_ACCOUNT The bech32 address of the tokenized account
/// OWNER The bech32 address of the account now in control of the token
/// NFT_ADDRESS The EVM address of the token contract in control of the tokenized account's excess profits
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenizedAccount {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tokenized_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub nft_address: ::prost::alloc::string::String,
}
/// MsgTokenizeAccount A Msg meant to convert a wallet into a piece of Liquid Infrastructure,
/// by creating a NonFungibleToken within the Althea L1 EVM which will control all balances
/// held by the Tokenized Account (beyond a configurable threshold)
/// SENDER The bech32 address of the account to tokenize, must also be the signer of the message
/// OWNER The bech32 address of the account initially in control of the token
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTokenizeAccount {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
}
/// MsgTokenizeAccountResponse potentially returns useful information from the tokenization of an account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTokenizeAccountResponse {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<TokenizedAccount>,
}
/// A type for the block's event log, every successful TokenizeAccount must create one of
/// these in the event log
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAccountTokenized {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owned: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub nft_address: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Msg defines the state transitions possible within microtx
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
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
        ) -> MsgClient<InterceptedService<T, F>>
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
            MsgClient::new(InterceptedService::new(inner, interceptor))
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
        /// The Xfer service is a customizeable version of the bank module's Send
        pub async fn xfer(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgXfer>,
        ) -> Result<tonic::Response<super::MsgXferResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/microtx.v1.Msg/Xfer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// The TokenizeAccount service converts a wallet into a piece of Liquid Infrastructure
        pub async fn tokenize_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTokenizeAccount>,
        ) -> Result<tonic::Response<super::MsgTokenizeAccountResponse>, tonic::Status> {
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
                "/microtx.v1.Msg/TokenizeAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Query the current microtx params
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// Query the additional fee paid on MsgXfer, determined by governance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryXferFeeRequest {
    #[prost(uint64, tag="1")]
    pub amount: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryXferFeeResponse {
    #[prost(uint64, tag="1")]
    pub fee_amount: u64,
}
/// Query the tokenized accounts known to the module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizedAccountsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizedAccountsResponse {
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<TokenizedAccount>,
}
/// Query for info about one particular Tokenized Account
/// OWNER if a bech32 address is provided, potenitally many accounts will be returned
/// TOKENIZED_ACCOUNT if a bech32 address is provided, the owner and nft contract address will be returned
/// NFT_ADDRESS if a EVM address is provided and happens to be a TokenizedAccountNFT contract, the owner and tokenized_account will be returned
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizedAccountRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tokenized_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub nft_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTokenizedAccountResponse {
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<TokenizedAccount>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Query defines the possible queries to make of the microtx module
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
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
        /// Query the current microtx params
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/microtx.v1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get an authoritative fee amount which must be paid on Xfer
        pub async fn xfer_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryXferFeeRequest>,
        ) -> Result<tonic::Response<super::QueryXferFeeResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/microtx.v1.Query/XferFee");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get all of the tokenized accounts known to the module
        pub async fn tokenized_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTokenizedAccountsRequest>,
        ) -> Result<
            tonic::Response<super::QueryTokenizedAccountsResponse>,
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
                "/microtx.v1.Query/TokenizedAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get info about one particular tokenized account by owner, bech32 address, or nft address
        /// TODO: Investigate the http API and what we might need to put into this URL
        pub async fn tokenized_account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTokenizedAccountRequest>,
        ) -> Result<
            tonic::Response<super::QueryTokenizedAccountResponse>,
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
                "/microtx.v1.Query/TokenizedAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
