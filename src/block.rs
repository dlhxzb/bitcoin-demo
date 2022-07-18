use crate::pow::*;
use crate::util::*;
use anyhow::Result;
use chrono::{DateTime, Utc};
use leveldb_orm::LeveldbOrm;
use serde::{Deserialize, Serialize};
use tracing::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    pub pre_hash: String,
    pub tranxs_hash: String,
    pub time: DateTime<Utc>,
    pub noice: u32,
    pub difficuty: u32,
}

#[derive(LeveldbOrm, Serialize, Deserialize, Clone, Debug)]
#[leveldb_key(header_hash)]
pub struct Block {
    pub header: BlockHeader,
    pub header_hash: String,
    pub tranxs: String,
}

#[derive(LeveldbOrm, Serialize, Deserialize, Clone, Debug)]
#[leveldb_key(tail_tag)]
pub struct TailHash {
    pub tail_tag: String,
    pub header_hash: String,
}

impl Block {
    pub fn new(tranxs: String, pre_hash: String, difficuty: u32) -> Result<Option<Self>> {
        info!("Creating block...");
        let tranxs_hash = hash_str(&tranxs)?;
        let header = BlockHeader {
            pre_hash,
            tranxs_hash,
            time: Utc::now(),
            noice: 0,
            difficuty,
        };
        let block = Block {
            header,
            header_hash: "".to_owned(),
            tranxs,
        };

        Ok(mining(&block, difficuty))
    }
}
