#![feature(result_option_inspect)]

mod block;
mod blockchain;
mod power;
mod util;

use anyhow::*;
use tracing::*;
use tracing_subscriber::FmtSubscriber;

use blockchain::BlockChain;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Start");
    let mut bc = BlockChain::new();
    let tranxs = "0xabcd -> 0xabce: 5 btc".to_string();
    bc.add_block(String::from(tranxs))?;
    let tranxs = "0xabcd -> 0xabcf: 2.5 btc".to_string();
    bc.add_block(String::from(tranxs))?;

    info!(?bc);
    info!("End");
    Ok(())
}
