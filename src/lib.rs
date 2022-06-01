mod abi;
mod pb;

use hex_literal::hex;
use substreams::{log, store, Hex};
use substreams_ethereum::pb::eth::v1 as eth;

use abi::gravity::events;
use pb::gravity::{GravatarUpdate, GravatarUpdates};

// Gravity contract
const GRAVITY_ADDRESS: [u8; 20] = hex!("2e645469f354bb4f5c8a05b3b30a929361cf77ec");

substreams_ethereum::init!();

/// Extracts transfers events from the contract
#[substreams::handlers::map]
fn gravatar_updates(block: eth::Block) -> Result<GravatarUpdates, substreams::errors::Error> {
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

    Ok(GravatarUpdates {
        updates: logs
            .filter_map(|log| {
                if events::NewGravatar::match_log(log) {
                    let event = events::NewGravatar::must_decode(log);

                    Some(GravatarUpdate {
                        id: format!("{}", event.id),
                        owner: event.owner,
                        display_name: event.display_name,
                        image_url: event.image_url,
                        ordinal: log.block_index as u64,
                    })
                } else if events::UpdatedGravatar::match_log(log) {
                    let event = events::UpdatedGravatar::must_decode(log);

                    Some(GravatarUpdate {
                        id: format!("{}", event.id),
                        owner: event.owner,
                        display_name: event.display_name,
                        image_url: event.image_url,
                        ordinal: log.block_index as u64,
                    })
                } else {
                    None
                }
            })
            .collect(),
    })
}
