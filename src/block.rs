use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::*;

use crate::util::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub pre_hash: String,
    pub tranxs_hash: String,
    pub time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub header_hash: String,
    pub tranxs: String,
}

impl Block {
    pub fn new(tranxs: String, pre_hash: String) -> Result<Self> {
        // TODO: mining
        info!("Start mining");

        std::thread::sleep(std::time::Duration::from_secs(3));
        let tranxs_hash = hash_str(&tranxs)?;
        let header = BlockHeader {
            pre_hash,
            tranxs_hash,
            time: Utc::now(),
        };
        let header_hash = hash_str(&header)?;
        Ok(Block {
            header,
            header_hash,
            tranxs,
        })
    }
}
