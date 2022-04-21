use anyhow::{anyhow, ensure, Error, Result};
use rust_base58::{FromBase58, ToBase58};

const SS58_PREFIX: &[u8] = b"SS58PRE";
const PUB_KEY_LENGTH: usize = 32;
const CHECK_SUM_LEN: usize = 2;

/// Convert address to ss58
/// 0xD43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D => 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
fn bytes_to_ss58(account: &[u8]) -> String {
    let mut ss58_address = [0; 35];
    ss58_address[0] = 42;
    ss58_address[1..33].copy_from_slice(account);
    let hash = ss58hash(&ss58_address[0..33]);
    ss58_address[33..35].copy_from_slice(&hash.as_bytes()[0..2]);
    ss58_address.to_base58()
}

/// Convert ss58 to address
/// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY => 0xD43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D
fn ss58_to_bytes(ss58: &str) -> Result<[u8; PUB_KEY_LENGTH]> {
    let bs58 = match ss58.from_base58() {
        Ok(bs58) => bs58,
        Err(err) => return Err(anyhow!("Wrong base58:{}", err)),
    };
    ensure!(
        bs58.len() > PUB_KEY_LENGTH + CHECK_SUM_LEN,
        format!(
            "Address length must be equal or greater than {} bytes",
            PUB_KEY_LENGTH + CHECK_SUM_LEN
        )
    );
    let check_sum = &bs58[bs58.len() - CHECK_SUM_LEN..];
    let address = &bs58[bs58.len() - PUB_KEY_LENGTH - CHECK_SUM_LEN..bs58.len() - CHECK_SUM_LEN];

    if check_sum != &ss58hash(&bs58[0..bs58.len() - CHECK_SUM_LEN]).as_bytes()[0..CHECK_SUM_LEN] {
        return Err(anyhow!("Wrong address checksum"));
    }
    let mut addr = [0; PUB_KEY_LENGTH];
    addr.copy_from_slice(address);
    Ok(addr)
}

fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
    let mut context = blake2_rfc::blake2b::Blake2b::new(64);
    context.update(SS58_PREFIX);
    context.update(data);
    context.finalize()
}

#[derive(Debug, Eq, PartialEq)]
pub struct Address([u8; PUB_KEY_LENGTH]);

impl Address {
    pub fn from_hex(value: &str) -> Result<Address> {
        let value = if let Some(value) = value.strip_prefix("0x") {
            value
        } else {
            value
        };
        let h = hex::decode(value)?;
        let mut address = [0; PUB_KEY_LENGTH];
        address.copy_from_slice(&h);
        Ok(Address(address))
    }

    pub fn from_ss58(value: &str) -> Result<Address> {
        Ok(Address(ss58_to_bytes(value)?))
    }
}

impl Address {
    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0).to_uppercase())
    }

    pub fn to_ss58(&self) -> String {
        bytes_to_ss58(&self.0)
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&str> for Address {
    type Error = Error;

    fn try_from(value: &str) -> Result<Address> {
        if value.starts_with("0x") {
            Address::from_hex(value)
        } else {
            Address::from_ss58(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ss58::Address;
    use crate::ss58::{bytes_to_ss58, ss58_to_bytes};

    const ALICE_HEX: &str = "0xD43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D";
    const ALICE_SS58: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

    #[test]
    fn test_bytes_to_ss58() {
        let account = hex::decode(&ALICE_HEX[2..]).unwrap();
        assert_eq!(bytes_to_ss58(&account), ALICE_SS58);
    }

    #[test]
    fn test_ss58_to_address() {
        let account = hex::decode(&ALICE_HEX[2..]).unwrap();
        assert_eq!(account, ss58_to_bytes(ALICE_SS58).unwrap());
    }

    #[test]
    fn test_account() {
        let account = Address::try_from(ALICE_SS58).unwrap();
        assert_eq!(account, Address::try_from(ALICE_HEX).unwrap());
        assert_eq!(&account.to_hex(), ALICE_HEX);
        assert_eq!(&account.to_ss58(), ALICE_SS58);
    }
}
