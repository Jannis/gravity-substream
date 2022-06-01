#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GravatarUpdates {
    #[prost(message, repeated, tag="1")]
    pub updates: ::prost::alloc::vec::Vec<GravatarUpdate>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GravatarUpdate {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub image_url: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
