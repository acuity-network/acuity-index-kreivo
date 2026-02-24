use mainnet_metadata::mainnet_metadata::{
    Event,
    runtime_types::{
        frame_system::pallet::Event as SystemEvent,
        pallet_balances::pallet::Event as BalancesEvent,
        pallet_multisig::pallet::Event as MultisigEvent,
        pallet_preimage::pallet::Event as PreimageEvent, pallet_proxy::pallet::Event as ProxyEvent,
        pallet_session::pallet::Event as SessionEvent,
        pallet_transaction_payment::pallet::Event as TransactionPaymentEvent,
        pallet_treasury::pallet::Event as TreasuryEvent,
        pallet_vesting::pallet::Event as VestingEvent,
    },
};
use subxt::utils::H256;

use crate::*;
use acuity_index_substrate::*;

use hex_literal::hex;

pub struct MainnetIndexer;

impl acuity_index_substrate::shared::RuntimeIndexer for MainnetIndexer {
    type RuntimeConfig = subxt::PolkadotConfig;
    type ChainKey = ChainKey;

    fn get_name() -> &'static str {
        "kreivo-mainnet"
    }

    fn get_genesis_hash() -> H256 {
        hex!["c710a5f16adc17bcd212cff0aedcbf1c1212a043cdc0fb2dcba861efe5305b01"].into()
    }

    fn get_versions() -> &'static [u32] {
        &[0]
    }

    fn get_default_url() -> &'static str {
        "wss://kreivo.io:443"
    }

    fn process_event(
        indexer: &acuity_index_substrate::substrate::Indexer<Self>,
        block_number: u32,
        event_index: u16,
        event: subxt::events::EventDetails<Self::RuntimeConfig>,
    ) -> Result<u32, IndexError> {
        Ok(match event.as_root_event::<Event>()? {
            // Substrate pallets.
            Event::System(event) => {
                index_system_event![SystemEvent, event, indexer, block_number, event_index]
            }
            Event::Preimage(event) => {
                index_preimage_event![PreimageEvent, event, indexer, block_number, event_index]
            }
            Event::Balances(event) => {
                index_balances_event![BalancesEvent, event, indexer, block_number, event_index]
            }
            Event::TransactionPayment(event) => {
                index_transaction_payment_event![
                    TransactionPaymentEvent,
                    event,
                    indexer,
                    block_number,
                    event_index
                ]
            }
            Event::Session(event) => {
                index_session_event![SessionEvent, event, indexer, block_number, event_index]
            }
            Event::Treasury(event) => {
                index_treasury_event![TreasuryEvent, event, indexer, block_number, event_index]
            }
            Event::Vesting(event) => {
                index_vesting_event![VestingEvent, event, indexer, block_number, event_index]
            }
            Event::Proxy(event) => {
                index_proxy_event![ProxyEvent, event, indexer, block_number, event_index]
            }
            Event::Multisig(event) => {
                index_multisig_event![MultisigEvent, event, indexer, block_number, event_index]
            }
            _ => 0,
        })
    }
}
