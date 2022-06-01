mod abi;
mod pb;

use hex_literal::hex;
use substreams::{log, store, Hex};
use substreams_ethereum::pb::eth::v1 as eth;

use abi::gravity::events;
use pb::gravity::{Gravatar, Gravatars};

// Gravity contract
const GRAVITY_ADDRESS: [u8; 20] = hex!("2e645469f354bb4f5c8a05b3b30a929361cf77ec");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn gravatars(block: eth::Block) -> Result<Gravatars, substreams::errors::Error> {
    let logs = block
        .transaction_traces
        .iter()
        .map(|tx| {
            tx.receipt
                .as_ref()
                .unwrap()
                .logs
                .iter()
                .filter(|log| log.address == GRAVITY_ADDRESS)
        })
        .flatten();

    // log::info!("{:?}", logs);

    let mut gravatars = vec![];

    for log in logs {
        if events::NewGravatar::match_log(log) {
            let event = events::NewGravatar::must_decode(log);

            gravatars.push(Gravatar {
                id: format!("{}", event.id),
                owner: event.owner,
                display_name: event.display_name,
                image_url: event.image_url,
            })
        }
    }

    Ok(Gravatars { gravatars })
}
