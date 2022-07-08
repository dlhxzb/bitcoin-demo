use anyhow::{anyhow, Result};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::sha3::Sha3;
use serde::Serialize;

pub fn hash_str<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let value_ser = bincode::serialize(value).map_err(|e| anyhow!(e))?;
    let mut hasher = Sha3::sha3_256();
    hasher.input(&value_ser);
    Ok(hasher.result_str())
}

pub fn hash_u8<T>(value: &T) -> Result<[u8; 32]>
where
    T: Serialize,
{
    let value_ser = bincode::serialize(value).map_err(|e| anyhow!(e))?;
    // let mut hasher = Sha3::sha3_256();
    let mut hasher = Sha256::new();
    hasher.input(&value_ser);
    let mut out: [u8; 32] = [0; 32];
    hasher.result(&mut out);
    Ok(out)
}
