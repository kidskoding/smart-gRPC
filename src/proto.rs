extern crate tonic;

tonic::include_proto!("sentinel");

pub const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("sentinel_descriptor");