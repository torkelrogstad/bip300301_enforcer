// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetSidechainProposalsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSidechainProposalsResponse {
    #[prost(message, repeated, tag="1")]
    pub sidechain_proposals: ::prost::alloc::vec::Vec<SidechainProposal>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SidechainProposal {
    #[prost(uint32, tag="1")]
    pub sidechain_number: u32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="4")]
    pub vote_count: u32,
    #[prost(uint32, tag="5")]
    pub proposal_height: u32,
    #[prost(uint32, tag="6")]
    pub proposal_age: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetSidechainsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSidechainsResponse {
    #[prost(message, repeated, tag="1")]
    pub sidechains: ::prost::alloc::vec::Vec<Sidechain>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sidechain {
    #[prost(uint32, tag="1")]
    pub sidechain_number: u32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub vote_count: u32,
    #[prost(uint32, tag="4")]
    pub proposal_height: u32,
    #[prost(uint32, tag="5")]
    pub activation_height: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectBlockRequest {
    #[prost(uint32, tag="1")]
    pub height: u32,
    #[prost(bytes="vec", tag="2")]
    pub block: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ConnectBlockResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectBlockRequest {
    #[prost(bytes="vec", tag="1")]
    pub block: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DisconnectBlockResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCoinbasePsbtRequest {
    #[prost(message, repeated, tag="1")]
    pub propose_sidechains: ::prost::alloc::vec::Vec<ProposeSidechain>,
    #[prost(message, repeated, tag="2")]
    pub ack_sidechains: ::prost::alloc::vec::Vec<AckSidechain>,
    #[prost(message, repeated, tag="3")]
    pub propose_bundles: ::prost::alloc::vec::Vec<ProposeBundle>,
    #[prost(message, optional, tag="4")]
    pub ack_bundles: ::core::option::Option<AckBundles>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCoinbasePsbtResponse {
    #[prost(bytes="vec", tag="1")]
    pub psbt: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckBundles {
    #[prost(enumeration="BundleTag", tag="1")]
    pub tag: i32,
    #[prost(uint32, repeated, tag="2")]
    pub upvotes: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckSidechain {
    #[prost(uint32, tag="1")]
    pub sidechain_number: u32,
    #[prost(bytes="vec", tag="2")]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposeSidechain {
    #[prost(uint32, tag="1")]
    pub sidechain_number: u32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposeBundle {
    #[prost(uint32, tag="1")]
    pub sidechain_number: u32,
    #[prost(bytes="vec", tag="2")]
    pub bundle_txid: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetDepositsRequest {
    #[prost(uint32, tag="1")]
    pub sidechain_number: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDepositsResponse {
    #[prost(message, repeated, tag="1")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deposit {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub value: u64,
    #[prost(uint64, tag="3")]
    pub sequence_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetCtipRequest {
    #[prost(uint32, tag="1")]
    pub sidechain_number: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCtipResponse {
    #[prost(message, optional, tag="1")]
    pub ctip: ::core::option::Option<Ctip>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ctip {
    #[prost(bytes="vec", tag="1")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="2")]
    pub vout: u32,
    #[prost(uint64, tag="3")]
    pub value: u64,
    #[prost(uint64, tag="4")]
    pub sequence_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetMainBlockHeightRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetMainBlockHeightResponse {
    #[prost(uint32, tag="1")]
    pub height: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetMainChainTipRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMainChainTipResponse {
    #[prost(bytes="vec", tag="1")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BundleTag {
    RepeatUnspecified = 0,
    RepeatPrevious = 1,
    LeadingBy50 = 2,
    Upvotes = 3,
}
impl BundleTag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BundleTag::RepeatUnspecified => "BUNDLE_TAG_REPEAT_UNSPECIFIED",
            BundleTag::RepeatPrevious => "BUNDLE_TAG_REPEAT_PREVIOUS",
            BundleTag::LeadingBy50 => "BUNDLE_TAG_LEADING_BY_50",
            BundleTag::Upvotes => "BUNDLE_TAG_UPVOTES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BUNDLE_TAG_REPEAT_UNSPECIFIED" => Some(Self::RepeatUnspecified),
            "BUNDLE_TAG_REPEAT_PREVIOUS" => Some(Self::RepeatPrevious),
            "BUNDLE_TAG_LEADING_BY_50" => Some(Self::LeadingBy50),
            "BUNDLE_TAG_UPVOTES" => Some(Self::Upvotes),
            _ => None,
        }
    }
}
include!("validator.v1.tonic.rs");
// @@protoc_insertion_point(module)