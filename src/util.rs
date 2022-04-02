use anyhow::{anyhow, Result};
use crypto::digest::Digest;
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
