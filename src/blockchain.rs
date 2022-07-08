use crate::block::Block;
use anyhow::{Context, Result};

const FIRST_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";
const INIT_DIFFICUTY: u32 = 0;
#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![Block::new(
                "First block".to_string(),
                FIRST_HASH.to_string(),
                INIT_DIFFICUTY,
            )
            .expect("Failed to create first block")],
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let pre_hash = self
            .blocks
            .last()
            .context("Uninit BlockChain")?
            .header_hash
            .clone();
        let difficuty = 0;
        let block = Block::new(data, pre_hash, difficuty)?;
        self.blocks.push(block);
        Ok(())
    }
}
