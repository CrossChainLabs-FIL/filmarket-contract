/*
 * FilMarket contract
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, PanicOnDefault};
use near_sdk::serde::Serialize;
use near_sdk::serde::Deserialize;

#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ActivePerRegion {
    europe: u32,
    asia: u32,
    north_america: u32,
    other: u32,
}

#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PricePerRegion {
    europe: f64,        // FIL
    asia: f64,          // FIL
    north_america: f64, // FIL
    other: f64,         // FIL
    global: f64,        // FIL
    fil_price: f64,     // USD
    power: u128,        // network power in TiB
    timestamp: u64,     // epoch time in seconds
}

#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageProvider {
    id: String,
    region: u8,  // "North America":1, "Europe":2, "Asia":3, "Other":4 
    power: f64,  // GiB
    price: f64,  // FIL
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FilMarket {
    storage_providers: UnorderedMap<String, StorageProvider>,
    price_per_region: UnorderedMap<u64, PricePerRegion>,
    active_per_region: ActivePerRegion,
    latest_timestamp: u64,
    owner: String,
}

#[near_bindgen]
impl FilMarket {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            storage_providers: UnorderedMap::new(b"a".to_vec()),
            price_per_region: UnorderedMap::new(b"b".to_vec()),
            active_per_region: ActivePerRegion {
                europe: 0, 
                asia: 0, 
                north_america: 0, 
                other: 0
            },
            latest_timestamp: 0,
            owner: env::predecessor_account_id().to_string(),
        }
    }

    // add or update storage providers
    pub fn update_storage_providers(&mut self, storage_providers: Vec<StorageProvider>) {
        let account_id = env::predecessor_account_id();

        if account_id.to_string() != self.owner {
            env::log_str(&format!("update_storage_providers(): account_id {} is not owner", account_id));
            return;
        }

        env::log_str(&format!("update_storage_providers(): account_id {} storage providers {}", account_id, storage_providers.len()));

        for sp in storage_providers.iter() {
            let empty_sp = StorageProvider {
                id: "".to_string(),
                region: 0 as u8,
                power: 0.0 as f64,
                price: 0.0 as f64,
            };

            let mut storage_provider = self.storage_providers.get(&sp.id).unwrap_or(empty_sp);
            if storage_provider.id.is_empty() {
                storage_provider.id = sp.id.clone();
                storage_provider.region = sp.region.clone();
            } 

            storage_provider.power = sp.power.clone();
            storage_provider.price = sp.price.clone();

            self.storage_providers.insert(&storage_provider.id, &storage_provider);
        }
    }

    // delete the given storage providers
    pub fn delete_storage_providers(&mut self, storage_providers: Vec<String>) {
        let account_id = env::predecessor_account_id();

        if account_id.to_string() != self.owner {
            env::log_str(&format!("delete_storage_providers(): account_id {} is not owner", account_id));
            return;
        }

        for iter in storage_providers.iter() {
            self.storage_providers.remove(iter);
        }

        env::log_str(&format!("delete_storage_providers(): account_id {} storage providers {}", account_id, storage_providers.len()));
    }

    // get the storage provider's list
    pub fn get_storage_providers(&self) -> Vec<StorageProvider> {
        let storage_providers = self.storage_providers.values_as_vector().to_vec();
        return storage_providers;
    }

    // set the total of active storage providers per region
    pub fn set_active_per_region(&mut self, active_per_region: ActivePerRegion) {
        let account_id = env::predecessor_account_id();

        if account_id.to_string() != self.owner {
            env::log_str(&format!("set_active_per_region(): account_id {} is not owner", account_id));
            return;
        }

        self.active_per_region = active_per_region;
    }

    // get the total of active storage providers per region
    pub fn get_active_per_region(&self) -> ActivePerRegion {
        let active_per_region = ActivePerRegion {
            europe: self.active_per_region.europe, 
            asia: self.active_per_region.asia, 
            north_america: self.active_per_region.north_america, 
            other: self.active_per_region.other
        };

        return active_per_region;
    }

    // set the average storage price per region
    pub fn set_price_per_region(&mut self, price_per_region: PricePerRegion) {
        let account_id = env::predecessor_account_id();

        if account_id.to_string() != self.owner {
            env::log_str(&format!("set_price_per_region(): account_id {} is not owner", account_id));
            return;
        }

        let empty_ppr = PricePerRegion {
            europe: 0.0 as f64,
            asia: 0.0 as f64,
            north_america: 0.0 as f64,
            other: 0.0 as f64,
            global: 0.0 as f64,
            fil_price: 0.0 as f64,
            power: 0 as u128,
            timestamp: 0 as u64,
        };

        let mut ppr = self.price_per_region.get(&price_per_region.timestamp).unwrap_or(empty_ppr);
        if ppr.timestamp == 0 {
            ppr.timestamp = price_per_region.timestamp;
        }

        ppr.europe = price_per_region.europe;
        ppr.asia = price_per_region.asia;
        ppr.north_america = price_per_region.north_america;
        ppr.other = price_per_region.other;
        ppr.global = price_per_region.global;
        ppr.fil_price = price_per_region.fil_price;
        ppr.power = price_per_region.power;

        self.price_per_region.insert(&ppr.timestamp, &ppr);
        self.latest_timestamp = price_per_region.timestamp;
    }

    // get the average storage price per region
    pub fn get_price_per_region_list(&self) -> Vec<PricePerRegion> {
        let ppr = self.price_per_region.values_as_vector().to_vec();
        return ppr;
    }

    // get the latest storage price per region
    pub fn get_latest_price_per_region(&self) ->PricePerRegion {
        let empty_ppr = PricePerRegion {
            europe: 0.0 as f64,
            asia: 0.0 as f64,
            north_america: 0.0 as f64,
            other: 0.0 as f64,
            global: 0.0 as f64,
            fil_price: 0.0 as f64,
            power: 0 as u128,
            timestamp: 0 as u64,
        };

        let ppr = self.price_per_region.get(&self.latest_timestamp).unwrap_or(empty_ppr);
        return ppr;
    }

    // delete the given timestamps
    pub fn delete_price_per_region(&mut self, timestamps: Vec<u64>) {
        let account_id = env::predecessor_account_id();

        if account_id.to_string() != self.owner {
            env::log_str(&format!("set_price_per_region(): account_id {} is not owner", account_id));
            return;
        }
    
        for iter in timestamps.iter() {
            self.price_per_region.remove(iter);
        }
    
         env::log_str(&format!("delete_price_per_region(): account_id {} entries {}", account_id, timestamps.len()));
    }
}

/*
 * To run from contract directory:
 * cargo test -- --nocapture
 */
#[cfg(test)]

mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext, AccountId};

    fn carol() -> AccountId {
        AccountId::new_unchecked("carol_near".to_string())
    }

    fn get_context() -> VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(carol())
            .attached_deposit(10000)
            .build()
    }

    #[test]
    fn set_then_get_remove_storage_providers() {
        let context = get_context();
        enum Regions {
            NorthAmerica = 1,
            Europe = 2,
            Asia = 3,
            Other = 4,
        }

        testing_env!(context);
        let mut contract = FilMarket::new();
        let sp_list = vec![
            StorageProvider {
                id: "id1".to_string(),
                region: Regions::Europe as u8,
                power: 24.64,
                price: 0.46
            },
            StorageProvider {
                id: "id2".to_string(),
                region: Regions::Asia as u8,
                power: 5693.0,
                price: 0.6778
            },
            StorageProvider {
                id: "id3".to_string(),
                region: Regions::NorthAmerica as u8,
                power: 54.64,
                price: 0.43
            },
            StorageProvider {
                id: "id4".to_string(),
                region: Regions::Other as u8,
                power: 454.64,
                price: 0.143
            },
        ];

        contract.update_storage_providers(sp_list);
        contract.delete_storage_providers(vec!["id4".to_string()]);
        let result = contract.get_storage_providers();

        assert_eq!(3, result.len());
        assert_eq!("id1".to_string(), result[0].id);
        assert_eq!("id2".to_string(), result[1].id);
        assert_eq!("id3".to_string(), result[2].id);
    }

    #[test]
    fn set_then_get_active_per_region() {
        let context = get_context();
        testing_env!(context);
        let mut contract = FilMarket::new();

        let active_per_region = ActivePerRegion {
            europe: 3,
            asia: 24,
            north_america: 12,
            other: 45,
        };

        contract.set_active_per_region(active_per_region);
        let result = contract.get_active_per_region();

        assert_eq!(3, result.europe);
        assert_eq!(24, result.asia);
        assert_eq!(12, result.north_america);
        assert_eq!(45, result.other);
    }

    #[test]
    fn set_then_get_price_per_region() {
        let context = get_context();
        testing_env!(context);
        let mut contract = FilMarket::new();

        let price_per_region = PricePerRegion {
            europe: 0.00013,
            asia: 0.0004,
            north_america: 0.0002,
            other: 0.00005,
            global: 0.00034,
            fil_price: 64.245,
            power: 1024,
            timestamp: 1,
        };

        contract.set_price_per_region(price_per_region);
        let result = contract.get_price_per_region_list();

        assert_eq!(0.00013, result[0].europe);
        assert_eq!(0.0004, result[0].asia);
        assert_eq!(0.0002, result[0].north_america);
        assert_eq!(0.00005, result[0].other);
        assert_eq!(0.00034, result[0].global);
        assert_eq!(64.245, result[0].fil_price);
        assert_eq!(1024, result[0].power);
        assert_eq!(1, result[0].timestamp);
    }
}
