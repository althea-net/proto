#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

/// Bech32ibc protobuf definitions
#[cfg(feature = "bech32ibc")]
#[cfg_attr(docsrs, doc(cfg(feature = "bech32ibc")))]
pub mod bech32ibc {
    /// Bech32 prefix -> IBC Channel mapping
    pub mod bech32ibc {
        pub mod v1 {
            include!("prost/bech32ibc.bech32ibc.v1beta1.rs");
        }
    }
}

/// Cosmos protobuf definitions.
pub mod cosmos {
    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod v1beta1 {
            include!("prost/cosmos.auth.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.auth.module.v1.rs");
            }
        }
    }

    /// Granting of arbitrary privileges from one account to another.
    pub mod authz {
        pub mod v1beta1 {
            include!("prost/cosmos.authz.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.authz.module.v1.rs");
            }
        }
    }

    /// Balances.
    pub mod bank {
        pub mod v1beta1 {
            include!("prost/cosmos.bank.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.bank.module.v1.rs");
            }
        }
    }

    /// Base functionality.
    pub mod base {
        /// Application BlockChain Interface (ABCI).
        ///
        /// Interface that defines the boundary between the replication engine
        /// (the blockchain), and the state machine (the application).
        pub mod abci {
            pub mod v1beta1 {
                include!("prost/cosmos.base.abci.v1beta1.rs");
            }
        }

        /// Node information
        pub mod node {
            pub mod v1beta1 {
                include!("prost/cosmos.base.node.v1beta1.rs");
            }
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("prost/cosmos.base.query.v1beta1.rs");
            }
        }

        /// Reflection support.
        pub mod reflection {
            pub mod v1beta1 {
                include!("prost/cosmos.base.reflection.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/cosmos.base.v1beta1.rs");
        }

        pub mod tendermint {
            pub mod v1beta1 {
                include!("prost/cosmos.base.tendermint.v1beta1.rs");
            }
        }
    }

    pub mod circuit {
        pub mod v1 {
            include!("prost/cosmos.circuit.v1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.circuit.module.v1.rs");
            }
        }
    }

    /// Circuit breaker
    pub mod consensus {
        pub mod v1 {
            include!("prost/cosmos.consensus.v1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.consensus.module.v1.rs");
            }
        }
    }

    /// Crisis handling
    pub mod crisis {
        pub mod v1beta1 {
            include!("prost/cosmos.crisis.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.crisis.module.v1.rs");
            }
        }
    }

    /// Cryptographic primitives.
    pub mod crypto {
        pub mod hd {
            pub mod v1 {
                include!("prost/cosmos.crypto.hd.v1.rs");
            }   
        }
        pub mod keyring {
            pub mod v1 {
                include!("prost/cosmos.crypto.keyring.v1.rs");
            }
        }
        /// Multi-signature support.
        pub mod multisig {
            include!("prost/cosmos.crypto.multisig.rs");
            pub mod v1beta1 {
                include!("prost/cosmos.crypto.multisig.v1beta1.rs");
            }
        }
        pub mod ed25519 {
            include!("prost/cosmos.crypto.ed25519.rs");
        }
        pub mod secp256k1 {
            include!("prost/cosmos.crypto.secp256k1.rs");
        }
        pub mod secp256r1 {
            include!("prost/cosmos.crypto.secp256r1.rs");
        }
    }

    /// Messages and services handling token distribution
    pub mod distribution {
        pub mod v1beta1 {
            include!("prost/cosmos.distribution.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.distribution.module.v1.rs");
            }
        }
    }

    /// Messages and services handling evidence
    pub mod evidence {
        pub mod v1beta1 {
            include!("prost/cosmos.evidence.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.evidence.module.v1.rs");
            }
        }
    }

    /// Allows accounts to grant fee allowances and to use fees from their accounts.
    pub mod feegrant {
        pub mod v1beta1 {
            include!("prost/cosmos.feegrant.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.feegrant.module.v1.rs");
            }
        }
    }

    /// Messages and services handling gentx's
    pub mod genutil {
        pub mod v1beta1 {
            include!("prost/cosmos.genutil.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.genutil.module.v1.rs");
            }
        }
    }

    /// Messages and services handling governance
    pub mod gov {
        pub mod v1 {
            include!("prost/cosmos.gov.v1.rs");
        }
        pub mod v1beta1 {
            include!("prost/cosmos.gov.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.gov.module.v1.rs");
            }
        }
    }

    pub mod group {
        pub mod v1 {
            include!("prost/cosmos.group.v1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.group.module.v1.rs");
            }
        }
    }

    /// Messages and services handling minting
    pub mod mint {
        pub mod v1beta1 {
            include!("prost/cosmos.mint.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.mint.module.v1.rs");
            }
        }
    }

    pub mod nft {
        pub mod v1beta1 {
            include!("prost/cosmos.nft.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.nft.module.v1.rs");
            }
        }
    }

    /// Messages and services handling chain parameters
    pub mod params {
        pub mod v1beta1 {
            include!("prost/cosmos.params.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.params.module.v1.rs");
            }
        }
    }

    /// Handling slashing parameters and unjailing
    pub mod slashing {
        pub mod v1beta1 {
            include!("prost/cosmos.slashing.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.slashing.module.v1.rs");
            }
        }
    }

    /// Proof-of-Stake layer for public blockchains.
    pub mod staking {
        pub mod v1beta1 {
            // WARNING: This file is problematic due to a namespace conflict, see the README for more info
            include!("prost/cosmos.staking.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.staking.module.v1.rs");
            }
        }
    }

    /// Transactions.
    pub mod tx {
        pub mod config {
            pub mod v1 {
                include!("prost/cosmos.tx.config.v1.rs");
            }
        }

        /// Transaction signing support.
        pub mod signing {
            pub mod v1beta1 {
                include!("prost/cosmos.tx.signing.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("prost/cosmos.tx.v1beta1.rs");
        }
    }

    /// Services for the upgrade module.
    pub mod upgrade {
        pub mod v1beta1 {
            include!("prost/cosmos.upgrade.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.upgrade.module.v1.rs");
            }
        }
    }

    /// Services and tx's for the vesting module.
    pub mod vesting {
        pub mod v1beta1 {
            include!("prost/cosmos.vesting.v1beta1.rs");
        }
        pub mod module {
            pub mod v1 {
                include!("prost/cosmos.vesting.module.v1.rs");
            }
        }
    }
}

/// IBC protobuf definitions.
pub mod ibc {
    /// IBC applications.
    pub mod applications {
        /// Transfer support.
        pub mod transfer {
            pub mod v1 {
                include!("prost/ibc.applications.transfer.v1.rs");
            }

            pub mod v2 {
                include!("prost/ibc.applications.transfer.v2.rs");
            }
        }

        /// ICA
        pub mod interchain_accounts {
            pub mod v1 {
                include!("prost/ibc.applications.interchain_accounts.v1.rs");
            }
            pub mod controller {
                pub mod v1 {
                    include!("prost/ibc.applications.interchain_accounts.controller.v1.rs");
                }
            }
            pub mod host {
                pub mod v1 {
                    include!("prost/ibc.applications.interchain_accounts.host.v1.rs");
                }
            }
        }
    }

    /// IBC core.
    pub mod core {
        /// IBC channels.
        pub mod channel {
            pub mod v1 {
                include!("prost/ibc.core.channel.v1.rs");
            }
        }

        /// IBC client.
        pub mod client {
            pub mod v1 {
                include!("prost/ibc.core.client.v1.rs");
            }
        }

        /// IBC commitments.
        pub mod commitment {
            pub mod v1 {
                include!("prost/ibc.core.commitment.v1.rs");
            }
        }

        /// IBC connections.
        pub mod connection {
            pub mod v1 {
                include!("prost/ibc.core.connection.v1.rs");
            }
        }

        /// IBC types.
        pub mod types {
            pub mod v1 {
                include!("prost/ibc.core.types.v1.rs");
            }
        }
    }

    /// IBC light clients.
    pub mod lightclients {
        pub mod localhost {
            pub mod v1 {
                include!("prost/ibc.lightclients.localhost.v1.rs");
            }
        }
        pub mod solomachine {
            pub mod v1 {
                include!("prost/ibc.lightclients.solomachine.v1.rs");
            }

            pub mod v2 {
                include!("prost/ibc.lightclients.solomachine.v2.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include!("prost/ibc.lightclients.tendermint.v1.rs");
            }
        }
    }
}

/// ICS23 protobuf definitions.
pub mod ics23 {
    include!("prost/ics23.rs");
}

/// Tendermint proto definitions
pub mod tendermint {
    pub mod abci {
        include!("prost/tendermint.abci.rs");
    }
    pub mod crypto {
        include!("prost/tendermint.crypto.rs");
    }
    pub mod libs {
        pub mod bits {
            include!("prost/tendermint.libs.bits.rs");
        }
    }
    pub mod p2p {
        include!("prost/tendermint.p2p.rs");
    }
    pub mod types {
        include!("prost/tendermint.types.rs");
    }
    pub mod version {
        include!("prost/tendermint.version.rs");
    }
}
