//! Constants used by the Espresso Systems company. This is stored in a central area to make sure we don't accidentally introduce duplicates.
//!
//! These are grouped by project. In the descendent projects you can re-export the module and use all the constants in there.
//!
//! ```ignore
//! // in CAP
//! use espresso_system_common::cap::tag;
//!
//! #[tagged_blob(tag::ASSET_SEED)]
//! pub struct AssetCodeSeed(...);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "metrics")]
pub mod metrics;

pub mod brand;

pub mod cap {
    /// Tags to be used for [`tagged-base64`](https://github.com/espressosystems/tagged-base64)
    pub mod tag {
        pub const INTERNAL_ASSET_CODE: &str = "INTERNAL_ASSET_CODE";
        pub const ASSET_SEED: &str = "ASSET_SEED";
        pub const ASSET_CODE: &str = "ASSET_CODE";
        pub const ASSET_DEF: &str = "ASSET_DEF";
        pub const BLIND: &str = "BLIND";
        pub const NUL: &str = "NUL";
        pub const REC: &str = "REC";
        pub const CRED: &str = "CRED";
        pub const ID: &str = "ID";
        pub const AUDMEMO: &str = "AUDMEMO";
        pub const RECMEMO: &str = "RECMEMO";
        pub const TXVERKEY: &str = "TXVERKEY";
        pub const USERPUBKEY: &str = "USERPUBKEY";
        pub const USERKEY: &str = "USERKEY";
        pub const CREDPUBKEY: &str = "CREDPUBKEY";
        pub const CREDKEY: &str = "CREDKEY";
        pub const AUDPUBKEY: &str = "AUDPUBKEY";
        pub const AUDKEY: &str = "AUDKEY";
        pub const FREEZEPUBKEY: &str = "FREEZEPUBKEY";
        pub const FREEZEKEY: &str = "FREEZEKEY";
        pub const NULKEY: &str = "NULKEY";
    }
}

pub mod cape {
    /// Tags to be used for [`tagged-base64`](https://github.com/espressosystems/tagged-base64)
    pub mod tag {
        pub const EADDR: &str = "EADDR";
        pub const ERC20: &str = "ERC20";
        pub const CMTMNT_CAPE_TRNSTN: &str = "CMTMNT_CAPE_TRNSTN";
    }
}

pub mod espresso {
    /// Tags to be used for [`tagged-base64`](https://github.com/espressosystems/tagged-base64)
    pub mod tag {
        pub const TXN: &str = "TXN";
        pub const BLOCK: &str = "BLOCK";
        pub const STATE: &str = "STATE";
        pub const SET: &str = "SET";
        pub const SEED: &str = "SEED";
    }
}

pub mod hotshot {
    /// Tags to be used for [`tagged-base64`](https://github.com/espressosystems/tagged-base64)
    pub mod tag {
        pub const PEER_ID: &str = "PEER_ID";
        pub const PRIVKEY_ID: &str = "PEER_PRIVKEY";

        pub const ORCHESTRATOR_VRF_BLOCK: &str = "ORCH_VRF_BLOCK";
        pub const ORCHESTRATOR_VRF_TXN: &str = "ORCH_VRF_TXN";
        pub const ORCHESTRATOR_VRF_STATE: &str = "ORCH_VRF_STATE";

        pub const ORCHESTRATOR_BLOCK: &str = "ORCH_BLOCK";
        pub const ORCHESTRATOR_TXN: &str = "ORCH_TXN";
        pub const ORCHESTRATOR_STATE: &str = "ORCH_STATE";

        pub const DENTRY_BLOCK: &str = "DENTRY_BLOCK";
        pub const DENTRY_TXN: &str = "DENTRY_TXN";
        pub const DENTRY_STATE: &str = "DENTRY_STATE";

        pub const DUMMY_BLOCK: &str = "DUMMY_BLOCK";
        pub const DUMMY_TXN: &str = "DUMMY_TXN";
        pub const DUMMY_STATE: &str = "DUMMY_STATE";

        pub const STATIC_VOTE_TOKEN: &str = "STATIC_VOTE";
        pub const VRF_VOTE_TOKEN: &str = "VRF_VOTE";

        pub const QC: &str = "QC";
        pub const LEAF: &str = "LEAF";
    }
}

pub mod jellyfish {
    /// Tags to be used for [`tagged-base64`](https://github.com/espressosystems/tagged-base64)
    pub mod tag {
        pub const NODE: &str = "NODE";
        pub const LEAF: &str = "LEAF";
        pub const PROOF: &str = "PROOF";
        pub const BATCHPROOF: &str = "BATCHPROOF";

        pub const BLS_VER_KEY: &str = "BLS_VER_KEY";
        pub const BLS_SIGNING_KEY: &str = "BLS_SIGNING_KEY_";
        pub const BLS_SIG: &str = "BLS_SIG";

        pub const SCHNORR_KEY_PAIR: &str = "SCHNORR_KEY_PAIR";
        pub const SCHNORR_VERK_EY: &str = "SCHNORR_VERKEY";
        pub const SCHNORR_SIGNING_KEY: &str = "SCHNORR_SIGNING_KEY";
        pub const SCHNORR_SIG: &str = "SCHNORR_SIG";
    }
}
