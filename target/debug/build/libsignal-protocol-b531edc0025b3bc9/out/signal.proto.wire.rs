#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignalMessage {
    #[prost(bytes="vec", optional, tag="1")]
    pub ratchet_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="2")]
    pub counter: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub previous_counter: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub ciphertext: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreKeySignalMessage {
    #[prost(uint32, optional, tag="5")]
    pub registration_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="1")]
    pub pre_key_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="6")]
    pub signed_pre_key_id: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="2")]
    pub base_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub identity_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// SignalMessage
    #[prost(bytes="vec", optional, tag="4")]
    pub message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SenderKeyMessage {
    #[prost(bytes="vec", optional, tag="1")]
    pub distribution_uuid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="2")]
    pub chain_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub iteration: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub ciphertext: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SenderKeyDistributionMessage {
    #[prost(bytes="vec", optional, tag="1")]
    pub distribution_uuid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, optional, tag="2")]
    pub chain_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag="3")]
    pub iteration: ::core::option::Option<u32>,
    #[prost(bytes="vec", optional, tag="4")]
    pub chain_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub signing_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
