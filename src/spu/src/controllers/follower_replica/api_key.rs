use kf_protocol::derive::Encode;
use kf_protocol::derive::Decode;

#[fluvio_kf(encode_discriminant)]
#[derive(PartialEq, Debug, Encode, Decode, Clone, Copy)]
#[repr(u16)]
pub enum KfFollowerPeerApiEnum {
    SyncRecords = 0,
}

impl Default for KfFollowerPeerApiEnum {
    fn default() -> KfFollowerPeerApiEnum {
        KfFollowerPeerApiEnum::SyncRecords
    }
}