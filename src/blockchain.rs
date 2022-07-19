use crate::block::{Block, TailHash};
use crate::transaction::Transaction;
use anyhow::{Context, Result};
use leveldb::database::Database;
use leveldb_orm::EncodedKey;
use leveldb_orm::{KVOrm, KeyOrm};
use tracing::info;

const FIRST_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";
const INIT_DIFFICUTY: u32 = 0;
const BLOCK_DB_DIR: &str = "./bc_db";
const TAIL_DB_DIR: &str = "./tail_db";
const TAIL_TAG: &str = "tail";

pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub tail: TailHash,
    pub block_db: Database<EncodedKey<Block>>,
    pub tail_db: Database<EncodedKey<TailHash>>,
}

impl BlockChain {
    // 数据库不存在或为空时，调用此函数
    pub fn new() -> Result<Self> {
        let mut options = leveldb::options::Options::new();
        options.create_if_missing = true;
        let block_db = Database::open(std::path::Path::new(BLOCK_DB_DIR), options)?;
        let mut options = leveldb::options::Options::new();
        options.create_if_missing = true;
        let tail_db = Database::open(std::path::Path::new(TAIL_DB_DIR), options)?;

        let first_trans = Transaction::new(
            "0x0000".to_owned(),
            "0x0000".to_owned(),
            0,
            0,
            0,
            "创世区块".to_owned(),
        )?;
        let first_block = Block::new(vec![first_trans], FIRST_HASH.to_string(), INIT_DIFFICUTY)?
            .expect("Create first block failed");

        let tail = TailHash {
            tail_tag: TAIL_TAG.to_owned(),
            header_hash: first_block.header_hash.clone(),
        };

        first_block.put(&block_db).map_err(anyhow::Error::msg)?;
        tail.put(&tail_db).map_err(anyhow::Error::msg)?;

        Ok(BlockChain {
            blocks: vec![first_block],
            tail,
            block_db,
            tail_db,
        })
    }

    // 加载blocks，数据库不存在时err
    pub fn load() -> Result<Self> {
        use leveldb::iterator::Iterable;
        use leveldb::options::{Options, ReadOptions};
        use std::path::Path;

        let block_db = Database::open(Path::new(BLOCK_DB_DIR), Options::new())?;
        let tail_db = Database::open(Path::new(TAIL_DB_DIR), Options::new())?;

        let blocks: Vec<_> = block_db
            .iter(ReadOptions::new())
            .map(|(_k, v)| Block::decode(&v).map_err(anyhow::Error::msg))
            .try_collect()?;

        let key = TailHash::encode_key(&TAIL_TAG.to_owned()).map_err(anyhow::Error::msg)?;
        let tail = TailHash::get(&tail_db, &key)
            .map_err(anyhow::Error::msg)?
            .context("Fail to get `tail` from tail_db")?;

        info!("Load {} blocks from {BLOCK_DB_DIR}", blocks.len());
        Ok(BlockChain {
            blocks,
            tail,
            block_db,
            tail_db,
        })
    }

    pub fn add_block(&mut self, trans: Vec<Transaction>) -> Result<()> {
        let pre_hash = self
            .blocks
            .last()
            .context("Uninit BlockChain")?
            .header_hash
            .clone();

        let difficuty = 0; // TODO: calc difficuty

        if let Some(block) = Block::new(trans, pre_hash, difficuty)? {
            let tail = TailHash {
                tail_tag: TAIL_TAG.to_owned(),
                header_hash: block.header_hash.clone(),
            };
            block.put(&self.block_db).map_err(anyhow::Error::msg)?;
            tail.put(&self.tail_db).map_err(anyhow::Error::msg)?;

            self.blocks.push(block);
        }

        Ok(())
    }
}
