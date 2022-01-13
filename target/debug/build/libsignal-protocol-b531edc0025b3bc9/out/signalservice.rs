#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    #[prost(bytes="vec", optional, tag="1")]
    pub data_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="2")]
    pub sync_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="3")]
    pub call_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="4")]
    pub null_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="5")]
    pub receipt_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="6")]
    pub typing_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="7")]
    pub sender_key_distribution_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", optional, tag="8")]
    pub decryption_error_message: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptionErrorMessage {
    /// set to the public ratchet key from the SignalMessage if a 1-1 payload fails to decrypt
    #[prost(bytes="vec", optional, tag="1")]
    pub ratchet_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, optional, tag="2")]
    pub timestamp: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag="3")]
    pub device_id: ::core::option::Option<u32>,
}
