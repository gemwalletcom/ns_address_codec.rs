use ton_smart_contract_address::{RawAddress, UserFriendlyAddress, UserFriendlyFlag};

use crate::codec::Codec;

struct TON {}

impl Codec for TON {
    fn decode(string: &str) -> Vec<u8> {
        let r = UserFriendlyAddress::from_user_friendly_str(string);
        match r {
            Ok(address) => address.as_bytes().to_vec(),
            Err(_) => Vec::new(),
        }
    }

    fn encode(bytes: Vec<u8>) -> String {
        unsafe {
            let address = RawAddress::from_bytes(bytes);
            return address.to_user_friendly_str(UserFriendlyFlag::Bounceable);
        }
    }
}
