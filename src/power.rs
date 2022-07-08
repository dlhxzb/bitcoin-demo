use anyhow::Result;
use bigint::U256;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::channel,
    Arc,
};
use tracing::*;

use crate::block::Block;
use crate::util::*;

// nonce 最大值
const MAX_NONCE: u32 = 0x7FFFFFFF;

pub fn mining(block: &Block, difficuty: u32) {
    info!(?difficuty, "Start mining");
    // let max_target = U256::MAX >> 32;
    let max_target = U256::MAX >> 24;
    let target = max_target >> difficuty as usize;
    info!(
        "target = 0x{:0>64}",
        format!("{:#x}", target).split('x').last().unwrap()
    );
    let thread_count = 1;
    let (tx, rx) = channel();

    std::thread::scope(|_| {
        let b_found = Arc::new(AtomicBool::new(false));
        for i in 1..=thread_count {
            let b_found = b_found.clone();
            let tx = tx.clone();
            let mut block = block.clone();
            let start = MAX_NONCE / thread_count * (i - 1);
            let end = MAX_NONCE / thread_count * i;
            std::thread::spawn(move || {
                {
                    for noice in start..end {
                        if noice % 10000 == 0 {
                            info!(?i, ?noice);
                        }
                        if b_found.load(Ordering::Relaxed) {
                            break;
                        }
                        block.header.noice = noice;
                        let hash = U256::from(hash_u8(&block)?);
                        if hash <= target {
                            if b_found
                                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                                .is_ok()
                            {
                                info!(
                                    "noice = {}, hash = 0x{:0>64}",
                                    noice,
                                    format!("{:#x}", hash).split('x').last().unwrap()
                                );
                            }
                            tx.send(block)?;
                            return Ok(());
                        }
                    }
                    Result::<(), anyhow::Error>::Ok(())
                }
                .inspect_err(|e| error!(?e))
            });
        }
    });
    if let Ok(block) = rx.try_recv() {
        info!(?block, "Mining succeed ");
    } else {
        info!("Mining failed");
    };
}
