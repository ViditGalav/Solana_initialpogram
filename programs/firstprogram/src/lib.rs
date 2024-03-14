#![feature(build_hasher_simple_hash_one)]

use anchor_lang::prelude::*;

declare_id!("Chn6UPzLDWASs7oLn3mvcpYREEHU5VZBjhX7zXAyyFFz");

#[program]
pub mod firstprogram {
    use super::*;

    pub fn firstfunction(ctx: Context<Firstfunction>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Firstfunction {}
