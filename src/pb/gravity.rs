#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gravatars {
    #[prost(message, repeated, tag="1")]
    pub gravatars: ::prost::alloc::vec::Vec<Gravatar>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gravatar {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub image_url: ::prost::alloc::string::String,
}
