use log::*;
use redis::Commands;
use serde_derive::Deserialize;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
    ReplicaTransactionInfoVersions, Result, SlotStatus,
};
use solana_measure::measure::Measure;
use solana_metrics::*;
use std::fmt::Write;
use {bs58, toml};

#[derive(Default, Deserialize)]
pub struct Config {
    pub accounts_filter: String,
    pub owners_filter: String,
    pub mentioned_addresses_filter: String,
}

impl Config {
    fn matches_account(&self, pk: &[u8]) -> bool {
        hex(pk).contains(&self.accounts_filter)
    }
}

pub struct GeyserPluginPaloma {
    pub config: Config,
    batch_starting_slot: Option<u64>,
    redis_client: redis::Client,
}

impl GeyserPluginPaloma {
    fn new() -> Self {
        Self {
            config: Default::default(),
            batch_starting_slot: Default::default(),
            redis_client: redis::Client::open("redis://127.0.0.1/").expect("redis must be running"),
        }
    }

    fn redis(&self) -> Result<redis::Connection> {
        self.redis_client
            .get_connection()
            .map_err(|e| GeyserPluginError::Custom(Box::new(e)))
    }
}

impl std::fmt::Debug for GeyserPluginPaloma {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl GeyserPlugin for GeyserPluginPaloma {
    fn name(&self) -> &'static str {
        "GeyserPluginPaloma"
    }

    fn on_load(&mut self, config_file: &str) -> Result<()> {
        solana_logger::setup_with_default("info");
        info!(
            "Loading plugin {:?} from config_file {:?}",
            self.name(),
            config_file
        );
        let config = std::fs::read_to_string(config_file)?;
        self.config =
            toml::from_str(&config).map_err(|_| GeyserPluginError::ConfigFileReadError {
                msg: "cannot parse toml".to_string(),
            })?;

        // TODO: This will need to be read out of Redis.
        self.batch_starting_slot = None;

        Ok(())
    }

    fn on_unload(&mut self) {
        info!("Unloading plugin: {:?}", self.name());
    }

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> Result<()> {
        // skip updating account on startup if batch_optimize_by_skipping_older_slots
        // is configured
        if is_startup
            && self
                .batch_starting_slot
                .map(|slot_limit| slot < slot_limit)
                .unwrap_or(false)
        {
            return Ok(());
        }

        let mut measure_all = Measure::start("geyser-plugin-paloma-update-account-main");
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(ai) => message_types::AccountInfo {
                pubkey: ai.pubkey,
                lamports: ai.lamports,
                owner: ai.owner,
                executable: ai.executable,
                rent_epoch: ai.rent_epoch,
                data: ai.data,
                write_version: ai.write_version,
            },
            ReplicaAccountInfoVersions::V0_0_2(ai) => message_types::AccountInfo {
                pubkey: ai.pubkey,
                lamports: ai.lamports,
                owner: ai.owner,
                executable: ai.executable,
                rent_epoch: ai.rent_epoch,
                data: ai.data,
                write_version: ai.write_version,
            },
        };

        let mut measure_select = Measure::start("geyser-plugin-paloma-update-account-select");
        if !self.config.matches_account(account_info.pubkey) {
            return Ok(());
        }
        measure_select.stop();
        inc_new_counter_debug!(
            "geyser-plugin-paloma-update-account-select-us",
            measure_select.as_us() as usize,
            100000,
            100000
        );

        debug!(
            "Updating account {} with owner {} at slot {:?} using account selector {:?}",
            bs58::encode(account_info.pubkey).into_string(),
            bs58::encode(account_info.owner).into_string(),
            slot,
            self.config.accounts_filter,
        );

        let mut measure_update = Measure::start("geyser-plugin-paloma-update-account-client");
        let message = serde_json::to_string(&account_info)
            .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;
        let message: &str = &message;
        self.redis()?
            .publish("accounts", message)
            .map_err(|e| GeyserPluginError::Custom(Box::new(e)))?;
        measure_update.stop();
        inc_new_counter_debug!(
            "geyser-plugin-paloma-update-account-client-us",
            measure_update.as_us() as usize,
            100000,
            100000
        );

        measure_all.stop();

        inc_new_counter_debug!(
            "geyser-plugin-paloma-update-account-main-us",
            measure_all.as_us() as usize,
            100000,
            100000
        );

        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> Result<()> {
        info!("Notifying the end of startup for accounts notifications");
        Ok(())
    }

    fn update_slot_status(
        &mut self,
        slot: u64,
        _parent: Option<u64>,
        _status: SlotStatus,
    ) -> Result<()> {
        // XXX: I feel like I should really use slot status here. It can be one of
        // Processed/Rooted/Confirmed
        redis::cmd("SET")
            .arg("slot")
            .arg(slot)
            .execute(&mut self.redis()?);
        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction_info: ReplicaTransactionInfoVersions,
        _slot: u64,
    ) -> Result<()> {
        let (message, meta) = match transaction_info {
            ReplicaTransactionInfoVersions::V0_0_1(transaction_info) => (
                transaction_info.transaction.message(),
                transaction_info.transaction_status_meta,
            ),
            ReplicaTransactionInfoVersions::V0_0_2(transaction_info) => (
                transaction_info.transaction.message(),
                transaction_info.transaction_status_meta,
            ),
        };
        if !(message
            .account_keys()
            .iter()
            .any(|pk| self.config.matches_account(&pk.to_bytes())))
        {
            return Ok(());
        }

        // XXX: Not actually sure what part of this we want to grab.
        info!("SEND {:?}", meta);
        Ok(())
    }

    fn notify_block_metadata(&mut self, block_info: ReplicaBlockInfoVersions) -> Result<()> {
        match block_info {
            ReplicaBlockInfoVersions::V0_0_1(block_info) => {
                info!("{:?}", block_info);
            }
        }
        Ok(())
    }

    /// Check if the plugin is interested in account data
    fn account_data_notifications_enabled(&self) -> bool {
        !self.config.accounts_filter.is_empty()
    }

    /// Check if the plugin is interested in transaction data
    fn transaction_notifications_enabled(&self) -> bool {
        !self.config.accounts_filter.is_empty()
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// This function returns the GeyserPluginPostgres pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = GeyserPluginPaloma::new();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}

/// Convert a PubKey to hexadecimal.
///
/// We're using this for now as a silly way to select accounts.
fn hex(bytes: &[u8]) -> String {
    let mut buf = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(&mut buf, "{:02x}", b).unwrap();
    }
    buf
}
