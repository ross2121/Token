use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::Metadata, token::{Mint, TokenAccount}, token_interface::TokenInterface};

declare_id!("Gek9iUND53ww6PB5jxWa5XB5ndnS9QZuVzrszwPGJtWp");

#[program]
pub mod token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,start_time:u64,end:u64,price:u64) -> Result<()> {
        ctx.accounts.token_lottery.bump=ctx.bumps.token_lottery;
       ctx.accounts.token_lottery.authority=*ctx.accounts.signer.key;
      ctx.accounts.token_lottery.end_time=end;
      ctx.accounts.token_lottery.start_time=start_time;
      ctx.accounts.token_lottery.ticket_price=price;
      ctx.accounts.token_lottery.winner_claimed=false;
      ctx.accounts.token_lottery.randomness_account=Pubkey::default();
       
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(init,space=8+TokenLottery::INIT_SPACE,payer=signer,seeds=[b"token",signer.key().as_ref()],
bump)]
pub token_lottery:Account<'info,TokenLottery>,
pub system_program:Program<'info,System>

}
#[account]
#[derive(InitSpace)]
pub struct  TokenLottery{
    pub bump:u8,
    pub start_time:u64,
    pub end_time:u64,
    pub lootery_pot_amount:u64,
    pub total_tickets:u64,    
    pub winner_claimed:bool,
    pub ticket_price:u64,
    pub authority:Pubkey,
    pub randomness_account:Pubkey
     
}
#[derive(Accounts)]
pub struct  InitializeToken<'info>{
 #[account(mut)]
 pub signer:Signer<'info>,
 #[account(init,payer=signer,mint::decimals=0,mint::authority=signer,mint::freeze_authority=signer,seeds=[b"collectionmint"],
bump)]
pub collection_mint:Account<'info,Mint>, 
#[account(
    init,
    payer=signer,
    associated_token::mint=collection_mint,
    associated_token::authority=signer
)]
pub collection_token:Account<'info,TokenAccount>, 
 pub token_program:Interface<'info,TokenInterface>,
 pub associated_token_program:Program<'info,AssociatedToken>,
 pub token_metadata_program:Program<'info,Metadata>,
 pub  system_program:Program<'info,System>
}