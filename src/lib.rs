use soroban_sdk::{Env, Symbol, Binary, vec, Address, IntoVal, Bytes};

struct Fraction {
    owner: Address,
    amount: u32, 
}

struct NFT {
    id: Symbol,
    name: String,
    description: String,

}

pub struct FractionalNFTContract;

#[contractimpl]
impl FractionalNFTContract {
    pub fn initialize(
        &self, 
        e: Env, 
        nft: NFT, 
        total_fractions: u32
    ) {
        
        e.data().set(Symbol::from_str("nft"), nft.into_val(&e));
        e.data().set(Symbol::from_str("total_fractions"), total_fractions);

        
        let fractions = Binary::new(e, Symbol::from_str("fractions"));
        e.data().set(Symbol::from_str("fractions"), fractions);
    }

    pub fn buy_fraction(&self, e: Env, buyer: Address, amount: u32) -> Result<(), String> {
        
        let fractions = Binary::new(e, Symbol::from_str("fractions"));
        fractions.set(&buyer, &Fraction { owner: buyer, amount: amount });

        Ok(()) 
    }

    pub fn sell_fraction(&self, e: Env, seller: Address, amount: u32) -> Result<(), String> {
        pub fn buy_fraction(&self, e: Env, buyer: Address, amount: u32) -> Result<(), String> {
            let total_fractions = e.data().get(Symbol::from_str("total_fractions"))
                                         .unwrap().try_into().unwrap();
            let fractions = Binary::new(e, Symbol::from_str("fractions"));
        
            
            let mut owned_fractions = 0;
            for _ in 0..total_fractions {
                if fractions.get(&buyer).is_some() { 
                    owned_fractions += 1;
                }
            }
            let available_fractions = total_fractions - owned_fractions;
        
            
            if amount > available_fractions {
                return Err("Not enough fractions available".to_string());
            }
        
            // Calculate price per fraction (assuming a starting valuation for the NFT)
            let nft_value = 1000; // Example: 1000 XLM
            let price_per_fraction = nft_value / total_fractions;
        
            // Total price for the purchase
            let total_price = price_per_fraction * amount;
        
            
        
        
            fractions.set(&buyer, &Fraction { owner: buyer, amount: amount });
        
            Ok(())
        }
        
    pub fn get_nft(&self, e: Env) -> NFT {
        e.data().get(Symbol::from_str("nft")).unwrap().try_into().unwrap()
    }
}
}
