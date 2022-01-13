#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalFingerprint {
    /// bytes identifier = 2;
    #[prost(bytes="vec", optional, tag="1")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinedFingerprints {
    #[prost(uint32, optional, tag="1")]
    pub version: ::core::option::Option<u32>,
    #[prost(message, optional, tag="2")]
    pub local_fingerprint: ::core::option::Option<LogicalFingerprint>,
    #[prost(message, optional, tag="3")]
    pub remote_fingerprint: ::core::option::Option<LogicalFingerprint>,
}
