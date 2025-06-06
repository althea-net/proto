//! This crate provides Gravity proto definitions in Rust and also re-exports cosmos_sdk_proto for use by downstream
//! crates. By default around a dozen proto files are generated and places into the prost folder. We could then proceed
//! to fix up all these files and use them as the required dependencies to the Gravity file, but we chose instead to replace
//! those paths with references ot upstream cosmos-sdk-proto and delete the other files. This reduces cruft in this repo even
//! if it does make for a somewhat more confusing proto generation process.

pub use cosmos_sdk_proto;
pub mod gravity {
    pub mod v1 {
        include!("prost/gravity.v1.rs");
        include!("ethereum_claim.rs");
    }
    pub mod v2 {
        include!("prost/gravity.v2.rs");
    }
}
pub mod auction {
    include!("prost/auction.v1.rs");
}
/// note to future readers I have hand copied the RegisteredInterchainAccount file from ibc.applications.interchain_accounts.v1.rs which is compiled
/// but not checked in into gaia.icaauth.v1.rs
pub mod gravity_test {
    pub mod gaia {
        pub mod globalfee {
            pub mod v1beta1 {
                include!("prost/gaia.globalfee.v1beta1.rs");
            }
        }
        pub mod icaauth {
            pub mod v1 {
                include!("prost/gaia.icaauth.v1.rs");
            }
        }
    }
}
