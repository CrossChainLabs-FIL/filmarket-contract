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
    europe: String,
    asia: String,
    north_america: String,
    other: String,
}

#[derive(Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageProvider {
    id: String,
    region: String,
    power: String,
    price: String,
    price_fil: String, 
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FilMarket {
    storage_providers: UnorderedMap<String, StorageProvider>,
    active_per_region: ActivePerRegion,
    price_per_region: PricePerRegion,
    global_price: String,
}

#[near_bindgen]
impl FilMarket {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            storage_providers: UnorderedMap::new(b"a".to_vec()),
            active_per_region: ActivePerRegion {
                europe:0, 
                asia:0, 
                north_america:0, 
                other:0
            },
            price_per_region: PricePerRegion {
                europe:'0'.to_string(), 
                asia:'0'.to_string(), 
                north_america:'0'.to_string(), 
                other:'0'.to_string()
            },
            global_price: '0'.to_string()
        }
    }

    // add or update storage providers
    pub fn update_storage_providers(&mut self, storage_providers: Vec<StorageProvider>) {
        let account_id = env::predecessor_account_id();

        env::log_str(&format!("update_storage_providers(): account_id {} storage providers {}", account_id, storage_providers.len()));

        for sp in storage_providers.iter() {
            let empty_sp = StorageProvider {
                id: "".to_string(),
                region: "".to_string(),
                power: "".to_string(),
                price: "".to_string(),
                price_fil: "".to_string()
            };

            let mut storage_provider = self.storage_providers.get(&sp.id).unwrap_or(empty_sp);
            if storage_provider.id.is_empty() {
                storage_provider.id = sp.id.clone();
                storage_provider.region = sp.region.clone();
            } 

            storage_provider.power = sp.power.clone();
            storage_provider.price = sp.price.clone();
            storage_provider.price_fil = sp.price_fil.clone();

            self.storage_providers.insert(&storage_provider.id, &storage_provider);
        }
    }

    // delete the given storage providers
    pub fn delete_storage_providers(&mut self, storage_providers: Vec<StorageProvider>) {
        let account_id = env::predecessor_account_id();

        env::log_str(&format!("delete_storage_providers(): account_id {} storage providers {}", account_id, storage_providers.len()));
    }

    // get the storage provider's list
    pub fn get_storage_providers(&self) -> Vec<StorageProvider> {
        let storage_providers = self.storage_providers.values_as_vector().to_vec();
        return storage_providers;
    }

    // set the total of active storage providers per region
    pub fn set_active_per_region(&mut self, active_per_region: ActivePerRegion) {
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
        self.price_per_region = price_per_region;
    }

    // get the average storage price per region
    pub fn get_price_per_region(&self) -> PricePerRegion {
        let price_per_region = PricePerRegion {
            europe: self.price_per_region.europe.clone(), 
            asia: self.price_per_region.asia.clone(), 
            north_america: self.price_per_region.north_america.clone(), 
            other: self.price_per_region.other.clone()
        };

        return price_per_region;
    }

    // set the global average storage price
    pub fn set_global_price(&mut self, global_price: String) {
        self.global_price = global_price;
    }

    // get the global average storage price
     pub fn get_global_price(&self) -> String {
        return self.global_price.clone();
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
    fn set_then_get_storage_providers() {
        let context = get_context();
        testing_env!(context);
        let mut contract = FilMarket::new();
        let sp_list = vec![
            StorageProvider {
                id: "id1".to_string(),
                region: "europe".to_string(),
                power: "500000".to_string(),
                price: "0.001 USD".to_string(),
                price_fil: "20 nanoFIL".to_string()
            },
            StorageProvider {
                id: "id2".to_string(),
                region: "asia".to_string(),
                power: "600000".to_string(),
                price: "0.003 USD".to_string(),
                price_fil: "30 nanoFIL".to_string()
            },
        ];

        contract.update_storage_providers(sp_list);
        let result = contract.get_storage_providers();

        assert_eq!(2, result.len());
        assert_eq!("id1".to_string(), result[0].id);
        assert_eq!("id2".to_string(), result[1].id);
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
            europe: "0.00013 USD".to_string(),
            asia: "0.0004 USD".to_string(),
            north_america: "0.0002 USD".to_string(),
            other: "0.00005 USD".to_string(),
        };

        contract.set_price_per_region(price_per_region);
        let result = contract.get_price_per_region();

        assert_eq!("0.00013 USD".to_string(), result.europe);
        assert_eq!("0.0004 USD".to_string(), result.asia);
        assert_eq!("0.0002 USD".to_string(), result.north_america);
        assert_eq!("0.00005 USD".to_string(), result.other);
    }

    #[test]
    fn set_then_global_price() {
        let context = get_context();
        testing_env!(context);
        let mut contract = FilMarket::new();

        contract.set_global_price("0.00003 USD".to_string());
        let result = contract.get_global_price();

        assert_eq!("0.00003 USD".to_string(), result);
    }
}
