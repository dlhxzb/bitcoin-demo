#![feature(result_option_inspect)]
#![feature(iterator_try_collect)]

mod block;
mod blockchain;
mod pow;
mod util;

use anyhow::Result;
use tracing::*;
use tracing_subscriber::FmtSubscriber;

use blockchain::BlockChain;

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
    let tranxs = "0xabcd -> 0xabce: 5 btc".to_string();
    bc.add_block(String::from(tranxs))?;
    let tranxs = "0xabcd -> 0xabcf: 2.5 btc".to_string();
    bc.add_block(String::from(tranxs))?;

    info!(?bc.blocks);
    info!("End");
    Ok(())
}
