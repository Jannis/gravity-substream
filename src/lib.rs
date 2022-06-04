mod abi;
mod pb;

use hex_literal::hex;
use substreams::{proto, store};
use substreams_ethereum::pb::eth::v1 as eth;

use abi::gravity::events;
use pb::gravity::{Gravatar, GravatarUpdate, GravatarUpdates};

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

    let updates = GravatarUpdates {
        updates: logs
            .filter_map(|log| {
                if events::NewGravatar::match_log(log) {
                    let event = events::NewGravatar::must_decode(log);

                    Some(GravatarUpdate {
                        id: format!("{}", event.id),
                        owner: hex::encode(event.owner),
                        display_name: event.display_name,
                        image_url: event.image_url,
                        ordinal: log.block_index as u64,
                    })
                } else if events::UpdatedGravatar::match_log(log) {
                    let event = events::UpdatedGravatar::must_decode(log);

                    Some(GravatarUpdate {
                        id: format!("{}", event.id),
                        owner: hex::encode(event.owner),
                        display_name: event.display_name,
                        image_url: event.image_url,
                        ordinal: log.block_index as u64,
                    })
                } else {
                    None
                }
            })
            .collect(),
    };

    Ok(updates)
}

#[substreams::handlers::store]
fn gravatars(updates: GravatarUpdates, store: store::StoreSet) {
    for update in updates.updates {
        let GravatarUpdate {
            id,
            owner,
            display_name,
            image_url,
            ..
        } = update;
        let gravatar = Gravatar {
            id: id.clone(),
            owner,
            display_name,
            image_url,
        };
        store.set(update.ordinal, id, &proto::encode(&gravatar).unwrap());
    }
}
