#![feature(result_option_inspect)]
#![feature(iterator_try_collect)]

mod block;
mod blockchain;
mod pow;
mod transaction;
mod util;

use anyhow::Result;
use tracing::*;
use tracing_subscriber::FmtSubscriber;

use blockchain::BlockChain;
use transaction::Transaction;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Start");
    let mut bc = match BlockChain::load() {
        Ok(bc) => bc,
        Err(e) => {
            warn!("Failed to open LevelDB:{e}, will create new");
            BlockChain::new()?
        }
    };
    let from = "0xabcd".to_string();
    let to = "0xabce".to_string();
    let sign = format!("{from} -> {to}: 9 btc");
    let tx = Transaction::new(from, to, 9, 1, 0, sign)?;
    bc.add_block(vec![tx])?;

    let from = "0xabce".to_string();
    let to = "0xabcf".to_string();
    let sign = format!("{from} -> {to}: 6 btc");
    let tx = Transaction::new(from, to, 6, 1, 0, sign)?;
    bc.add_block(vec![tx])?;

    info!(?bc.blocks);
    info!("End");
    Ok(())
}
