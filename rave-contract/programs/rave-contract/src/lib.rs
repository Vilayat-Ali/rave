use anchor_lang::prelude::*;

declare_id!("FoYYE3RhxoArMi5JxMQiBGQuANo8jqLnY44vYdsLu11h");

#[program]
pub mod rave_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
