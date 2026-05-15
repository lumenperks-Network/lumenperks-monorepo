#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol};

#[contracttype]
pub struct Customer {
    pub name: Symbol,
    pub points: u64,
}

#[contract]
pub struct LumenPerksContract;

#[contractimpl]
impl LumenPerksContract {
    /// Register a new customer with zero points.
    pub fn register(env: Env, wallet: Address, name: Symbol) {
        wallet.require_auth();
        let mut customers: Map<Address, Customer> =
            env.storage().persistent().get(&Symbol::new(&env, "customers"))
                .unwrap_or(Map::new(&env));
        customers.set(wallet, Customer { name, points: 0 });
        env.storage().persistent().set(&Symbol::new(&env, "customers"), &customers);
    }

    /// Add loyalty points to a customer (admin only via invoker auth).
    pub fn add_points(env: Env, wallet: Address, amount: u64) {
        let mut customers: Map<Address, Customer> =
            env.storage().persistent().get(&Symbol::new(&env, "customers"))
                .unwrap_or(Map::new(&env));
        let mut customer = customers.get(wallet.clone()).expect("customer not found");
        customer.points += amount;
        customers.set(wallet, customer);
        env.storage().persistent().set(&Symbol::new(&env, "customers"), &customers);
    }

    /// Get a customer's current points.
    pub fn get_points(env: Env, wallet: Address) -> u64 {
        let customers: Map<Address, Customer> =
            env.storage().persistent().get(&Symbol::new(&env, "customers"))
                .unwrap_or(Map::new(&env));
        customers.get(wallet).map(|c| c.points).unwrap_or(0)
    }

    /// Redeem points (deduct from balance).
    pub fn redeem(env: Env, wallet: Address, amount: u64) {
        wallet.require_auth();
        let mut customers: Map<Address, Customer> =
            env.storage().persistent().get(&Symbol::new(&env, "customers"))
                .unwrap_or(Map::new(&env));
        let mut customer = customers.get(wallet.clone()).expect("customer not found");
        assert!(customer.points >= amount, "insufficient points");
        customer.points -= amount;
        customers.set(wallet, customer);
        env.storage().persistent().set(&Symbol::new(&env, "customers"), &customers);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, Symbol};

    #[test]
    fn test_register_and_points() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, LumenPerksContract);
        let client = LumenPerksContractClient::new(&env, &contract_id);

        let wallet = Address::generate(&env);
        let name = Symbol::new(&env, "Alice");

        client.register(&wallet, &name);
        assert_eq!(client.get_points(&wallet), 0);

        client.add_points(&wallet, &100);
        assert_eq!(client.get_points(&wallet), 100);

        client.redeem(&wallet, &40);
        assert_eq!(client.get_points(&wallet), 60);
    }
}
