use crate::util::hash_str;
use anyhow::Result;
use serde::{Deserialize, Serialize};

// 交易体
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub nonce: u64,  // 交易记录值
    pub amount: u64, // 金额
    pub fee: u64,    // 手续费
    pub from: String,
    pub to: String,
    pub sign: String, // 交易信息
    pub hash: String,
}

impl Transaction {
    pub fn new(
        from: String,
        to: String,
        amount: u64,
        fee: u64,
        nonce: u64,
        sign: String,
    ) -> Result<Self> {
        let mut tx = Transaction {
            nonce,
            amount,
            fee,
            from,
            to,
            sign,
            hash: "".to_string(),
        };
        tx.hash = hash_str(&tx)?;

        Ok(tx)
    }
}
