#[macro_use]
extern crate solana_misc_macro;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

/// A sample struct with which to play.
pub struct AddressBookEntry {
    name: String,
    pay_to: Pubkey,
}

impl AddressBookEntry {
    fn new(name: String, address: Pubkey) -> Self {
        Self {
            name,
            pay_to: address,
        }
    }
}
pub fn main() {
    let address = "Fd7btgySsrjuo25CJCj7oE7VPMyezDhnx7pZkj2v69Nk";
    let payto = AddressBookEntry::new(
        "Richard".to_string(),
        Pubkey::from_str("Fd7btgySsrjuo25CJCj7oE7VPMyezDhnx7pZkj2v69Nk").unwrap(),
    );
    assert_encodings_match!(payto.pay_to, "Fd7btgySsrjuo25CJCj7oE7VPMyezDhnx7pZkj2v69Nk");
}
