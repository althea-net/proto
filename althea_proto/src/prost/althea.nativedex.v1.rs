/// GenesisState defines the nativedex module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub verified_native_dex_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub verified_croc_policy_address: ::prost::alloc::string::String,
}
/// UpgradeProxyProposal will replace one of the nativedex callpath contracts (or install a new one)
/// if passes, calls CrocPolicy.treasuryResolution(CrocSwapDex, 0, <ABI Encoded Bytes(21, <callpath_address>, <callpath_index>)>)
///
/// BE VERY CAREFUL EXECUTING THIS PROPOSAL, AS IT CAN COMPLETELY BREAK THE DEX CONTRACT
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeProxyProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<UpgradeProxyMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeProxyMetadata {
    /// the address of the contract to install
    #[prost(string, tag = "1")]
    pub callpath_address: ::prost::alloc::string::String,
    /// the callpath index to write to, see solidity-dex/contracts/mixins/StorageLayout.sol for the default values
    #[prost(uint64, tag = "2")]
    pub callpath_index: u64,
}
/// CollectTreasuryProposal will pay out protocol fees to the registered (and timelocked) `treasury_` account
/// If passes, calls CrocPolicy.treasuryResolution(CrocSwapDex, 3, <ABI Encoded Bytes(40, <token_address>)>)
///
/// Note that by default the protocol fees will be set to zero, see the governance history or use CrocQuery with a pool
/// to determine the current protocol take
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectTreasuryProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<CollectTreasuryMetadata>,
    /// must be true if the DEX is in safe mode, false otherwise
    #[prost(bool, tag = "4")]
    pub in_safe_mode: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectTreasuryMetadata {
    /// the ERC20 address to collect into the `treasury_` account, (0x0 for the native token)
    #[prost(string, tag = "1")]
    pub token_address: ::prost::alloc::string::String,
}
/// SetTreasuryProposal will change the `treasury_` address
/// the treasury_ address will be restricted from receiving protocol fees for a period of time (stored in treasuryStartTime_)
/// If passes, calls CrocPolicy.treasuryResolution(CrocSwapDex, 3, <ABI Encoded Bytes(41, <treasury_address>)>)
///
/// Note that by default the protocol fees will be set to zero, see the governance history or use CrocQuery with a pool
/// to determine the current protocol take
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTreasuryProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<SetTreasuryMetadata>,
    /// must be true if the DEX is in safe mode, false otherwise
    #[prost(bool, tag = "4")]
    pub in_safe_mode: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTreasuryMetadata {
    /// the address to set `treasury_` to
    #[prost(string, tag = "1")]
    pub treasury_address: ::prost::alloc::string::String,
}
/// AuthorityTransferProposal will change the `authority_` address, which can be used to upgrade or remove the CrocPolicy
/// contract (and therefore this module must be upgraded to work with the replacement, and configured as the new authority)
/// If passes, calls CrocPolicy.treasuryResolution(CrocSwapDex, 3, <ABI Encoded Bytes(20, <auth_address>)>)
///
/// BE VERY CAREFUL EXECUTING THIS PROPOSAL, AS IT CAN COMPLETELY DISABLE THE NATIVEDEX MODULE AND REMOVE STAKER AUTHORITY
/// OVER THE DEX CONTRACT
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorityTransferProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<AuthorityTransferMetadata>,
    /// must be true if the DEX is in safe mode, false otherwise
    #[prost(bool, tag = "4")]
    pub in_safe_mode: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorityTransferMetadata {
    /// the address to set `authority_` to
    #[prost(string, tag = "1")]
    pub auth_address: ::prost::alloc::string::String,
}
/// HotPathOpenProposal will change the `hotPathOpen_` flag, which controls if users are able to call swap directly on the dex contract
/// The primary purpose of this seems to be enabling upgradeability of the HotProxy contract, which would require users to switch
/// to calling CrocSwapDex.userCmd(1, <ABI Encoded Args>) instead of CrocSwapDex.swap(<args>) so that they call the new code.
/// If passes, calls CrocPolicy.treasuryResolution(CrocSwapDex, 3, <ABI Encoded Bytes(22, <open>)>)
///
/// BE VERY CAREFUL EXECUTING THIS PROPOSAL, AS IT CAN BREAK INFLEXIBLE DEX USER INTERFACES
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotPathOpenProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<HotPathOpenMetadata>,
    /// must be true if the DEX is in safe mode, false otherwise
    #[prost(bool, tag = "4")]
    pub in_safe_mode: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotPathOpenMetadata {
    /// If true, users can call swap directly on the dex contract.
    /// If false, they must call CrocSwapDex.userCmd(1, <ABI Encoded Args>)
    #[prost(bool, tag = "1")]
    pub open: bool,
}
/// SetSafeModeProposal will lock down the DEX for emergency changes. This can also be used by the emergency multisig to halt the DEX more quickly.
/// When the DEX is in safe mode only a UpgradeProxy, CollectTreasury, SetTreasury, AuthorityTransfer, HotPathOpen, or SetSafeMode Proposal can be executed,
/// and these proposals can only be executed under the SafeMode or Boot Proxy callpaths.
/// If passes, calls CrocPolicy.treasuryResolution(CrocSwapDex, 3, <ABI Encoded Bytes(23, <lock_dex>)>)
///
/// BE VERY CAREFUL EXECUTING THIS PROPOSAL, IT SHOULD ONLY BE USED TO DISABLE THE DEX OR RECOVER FROM DISABLES
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSafeModeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<SetSafeModeMetadata>,
    /// must be true if the DEX is already in safe mode, false otherwise
    #[prost(bool, tag = "4")]
    pub in_safe_mode: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSafeModeMetadata {
    /// If true, the DEX will be disabled
    #[prost(bool, tag = "1")]
    pub lock_dex: bool,
}
/// TransferGovernanceProposal will update the governance role addresses on CrocPolicy.
/// If passes, calls CrocPolicy.transferGovernance(<ops>, <nativedex module address>, <emergency>)
///
/// BE VERY CAREFUL EXECUTING THIS PROPOSAL, THE OPS AND EMERGENCY ADDRESSES SHOULD BE CAREFULLY CHOSEN MULTISIGS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferGovernanceProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<TransferGovernanceMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferGovernanceMetadata {
    /// The address to use for the Ops governance role, the least privileged role
    #[prost(string, tag = "1")]
    pub ops: ::prost::alloc::string::String,
    /// The address to use for the Emergency governance role, which can halt the DEX or perform Ops functions
    #[prost(string, tag = "2")]
    pub emergency: ::prost::alloc::string::String,
}
/// OpsProposal will execute a non-sudo `protocolCmd()` call on the DEX via CrocPolicy.
/// If passes, calls CrocPolicy.opsResolution (CrocSwapDex, <callpath>, <ABI Encoded Bytes(<cmd args>)>)
///
/// This proposal enables nativedex governance to perform everyday Ops functions on the DEX,
/// and so the Ops or Emergency addresses could override any decisions made by this proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<OpsMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpsMetadata {
    /// The callpath index to use, see solidity-dex/contracts/mixins/StorageLayout.sol for the default values
    #[prost(uint64, tag = "1")]
    pub callpath: u64,
    /// The ABI encoded bytes to pass to the opsResolution() call
    #[prost(bytes = "vec", tag = "2")]
    pub cmd_args: ::prost::alloc::vec::Vec<u8>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the gRPC querier service.
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
        /// Parameters queries the parameters of the module.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
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
                "/althea.nativedex.v1.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("althea.nativedex.v1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
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
    }
}
