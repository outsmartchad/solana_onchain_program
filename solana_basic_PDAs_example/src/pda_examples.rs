use solana_program::{
    pubkey::Pubkey,
    program_pack::Pack,
};
use std::str::FromStr;


// Function to find a PDA
pub fn find_pda(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(seeds, program_id)
}

// Example usage
pub fn main() {
    // Program id: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
    // Example 1: Pumpfun bonding curve
    let program_id = Pubkey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P")
        .expect("Invalid program ID");
    let mint_pubkey = Pubkey::from_str("HqVw4bRwK9doGb1RojSqUyhFB6jTHYV1Xfac9EqWpump") // put any current pump.fun mint here, will get the bonding curve address
        .expect("Invalid mint pubkey");
    
    println!("program id: {}", program_id);
    let (bonding_curve_pda, bump) = find_pda(
        &[
            b"bonding-curve",
            mint_pubkey.as_ref(),
        ],
        &program_id
    );
    println!("bonding_curve_id: {}", bonding_curve_pda); // 8enSvFUseS5P7ZAi9KKgNfemcNcZVsdGWQJbahG2Keqg
    // link: https://solscan.io/account/8enSvFUseS5P7ZAi9KKgNfemcNcZVsdGWQJbahG2Keqg
    println!("bump: {}", bump);
} 