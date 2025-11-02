//! CLI client for Younger Dryas Civilizations

use anyhow::Result;

fn main() -> Result<()> {
    println!("Younger Dryas Civilizations CLI");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    
    let state = simcore::State::new();
    let hash = simcore::state_hash(&state);
    println!("Initial state hash: {:?}", hash);
    
    Ok(())
}

