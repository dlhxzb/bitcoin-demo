use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::*;

use crate::power::*;
use crate::util::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    pub pre_hash: String,
    pub tranxs_hash: String,
    pub time: DateTime<Utc>,
    pub noice: u32,
    pub difficuty: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub header_hash: String,
    pub tranxs: String,
}

impl Block {
    pub fn new(tranxs: String, pre_hash: String, difficuty: u32) -> Result<Self> {
        info!("Creating block...");
        let tranxs_hash = hash_str(&tranxs)?;
        let header = BlockHeader {
            pre_hash,
            tranxs_hash,
            time: Utc::now(),
            noice: 0,
            difficuty,
        };
        let header_hash = hash_str(&header)?;
        let block = Block {
            header,
            header_hash,
            tranxs,
        };
        mining(&block, difficuty);

        Ok(block)
    }
}
