use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::Metadata, 
    token::{Mint, Token, TokenAccount}
};
use anchor_spl::{metadata::{create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::{ types::{CollectionDetails, Creator, DataV2}}, set_and_verify_sized_collection_item, sign_metadata, CreateMasterEditionV3, CreateMetadataAccountsV3, SetAndVerifySizedCollectionItem, SignMetadata}, token_2022::{mint_to, MintTo}};
use switchboard_on_demand::RandomnessAccountData;
declare_id!("Gek9iUND53ww6PB5jxWa5XB5ndnS9QZuVzrszwPGJtWp");
#[constant]
pub const NAME: &str="TOKEN LOTTERY TICKET";

#[constant]
pub const SYMBOL:&str="TLT";
#[constant]
pub const  URI:&str="https://www.edepotindia.com/wp-content/uploads/2018/12/west-bengal-lottery-ticket.jpg";

#[program]
pub mod token {
    use std::any::Any;
    

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
    pub fn buy_ticket(ctx: Context<Buyticket>)->Result<()>{
        let clock=Clock::get()?;
        let ticket_name=NAME.to_owned()+ctx.accounts.token_lottery.total_tickets.to_string().as_str();
        if(clock.slot<ctx.accounts.token_lottery.start_time||clock.slot>ctx.accounts.token_lottery.end_time){
            return  Err(ErrorCode:: LotteryNotOpen.into());
        }
        system_program::transfer(CpiContext::new(ctx.accounts.system_program.to_account_info(),system_program::Transfer { from: (ctx.accounts.payer.to_account_info()), to: (ctx.accounts.token_lottery.to_account_info()) }), ctx.accounts.token_lottery.ticket_price)?;
        let signer_seeds:&[&[&[u8]]]=&[&[b"collectionmint".as_ref(),&[ctx.bumps.collection_mint]]];
        mint_to(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), MintTo { mint: ctx.accounts.ticket_mint.to_account_info(), to: ctx.accounts.destination.to_account_info(), authority: ctx.accounts.collection_mint.to_account_info() }, signer_seeds), 1)?;
                  
        create_metadata_accounts_v3(CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),CreateMetadataAccountsV3{
            metadata:ctx.accounts.ticket_metadata.to_account_info(),
            mint:ctx.accounts.ticket_mint.to_account_info(),
            mint_authority:ctx.accounts.collection_mint.to_account_info(),
             payer:ctx.accounts.payer.to_account_info(),
             update_authority:ctx.accounts.collection_mint.to_account_info(),
             system_program:ctx.accounts.system_program.to_account_info(),
             rent:ctx.accounts.rent.to_account_info()
        }, signer_seeds),DataV2{
                 name:ticket_name,
                 symbol:SYMBOL.to_string(),
                 uri:URI.to_string(),
                 seller_fee_basis_points:0,
                 creators:Some(vec![Creator {
                    address:ctx.accounts.collection_mint.key(),
                    verified:false,
                    share:100
                 }]),
                 collection:None,
                 uses:None 
        },true, true,None)?;
        
        create_master_edition_v3(CpiContext::new_with_signer(ctx.accounts.token_metadata_program.to_account_info(),CreateMasterEditionV3{
            edition:ctx.accounts.ticket_master_edition.to_account_info(),
            mint:ctx.accounts.ticket_mint.to_account_info(),
            update_authority:ctx.accounts.collection_mint.to_account_info(),
            payer:ctx.accounts.payer.to_account_info(),
            token_program:ctx.accounts.token_program.to_account_info(),
            system_program:ctx.accounts.system_program.to_account_info(),
            rent:ctx.accounts.rent.to_account_info(),
            metadata:ctx.accounts.ticket_metadata.to_account_info(),
            mint_authority:ctx.accounts.collection_mint.to_account_info()
        }, signer_seeds),Some(0))?;
        set_and_verify_sized_collection_item(CpiContext::new_with_signer(ctx.accounts.token_metadata_program.to_account_info(), SetAndVerifySizedCollectionItem{
            metadata:ctx.accounts.ticket_metadata.to_account_info(),
            collection_authority:ctx.accounts.collection_mint.to_account_info(),
            payer:ctx.accounts.payer.to_account_info(),
            update_authority:ctx.accounts.collection_mint.to_account_info(),
            collection_metadata:ctx.accounts.collection_metadata.to_account_info(),
            collection_master_edition:ctx.accounts.collection_master_edition.to_account_info(),
            collection_mint:ctx.accounts.collection_mint.to_account_info()
        }, signer_seeds), None)?;
        ctx.accounts.token_lottery.total_tickets+=1;
        Ok(())
    }
    pub fn commit_randomness(ctx:Context<CommitRandomness>)->Result<()>{
        let clock:Clock=Clock::get()?;
        let token_lottery=&mut ctx.accounts.token_lottery;
        if(ctx.accounts.payer.key()!=token_lottery.authority){
            return Err(ErrorCode::Notauthorized.into());
        }
        let randomness_data=RandomnessAccountData::parse(ctx.accounts.randomness_account.data.borrow()).unwrap();
        if(randomness_data.seed_slot!=clock.slot-1){
            return Err(ErrorCode:: RandomnessAlredyRevealed.into());
        }
        token_lottery.randomness_account=ctx.accounts.randomness_account.key();
        Ok(())
    }
    
    pub fn initialize_lottery(ctx: Context<InitializeToken>) -> Result<()> {
          let signer_seeds:&[&[&[u8]]]=&[&[b"collectionmint",&[ctx.bumps.collection_mint],]];
        
          mint_to(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
             MintTo{
                mint:ctx.accounts.collection_mint.to_account_info(),to:ctx.accounts.collection_token.to_account_info(),authority:ctx.accounts.collection_mint.to_account_info()}, signer_seeds),1)?;
                      
                create_metadata_accounts_v3(CpiContext::new_with_signer(
                    ctx.accounts.token_metadata_program.to_account_info(),CreateMetadataAccountsV3{
                    metadata:ctx.accounts.metadata.to_account_info(),
                    mint:ctx.accounts.collection_mint.to_account_info(),
                    mint_authority:ctx.accounts.collection_mint.to_account_info(),
                     payer:ctx.accounts.signer.to_account_info(),
                     update_authority:ctx.accounts.collection_mint.to_account_info(),
                     system_program:ctx.accounts.system_program.to_account_info(),
                     rent:ctx.accounts.rent.to_account_info()
                }, signer_seeds),DataV2{
                         name:NAME.to_string(),
                         symbol:SYMBOL.to_string(),
                         uri:URI.to_string(),
                         seller_fee_basis_points:0,
                         creators:Some(vec![Creator {
                            address:ctx.accounts.collection_mint.key(),
                            verified:false,
                            share:100
                         }]),
                         collection:None,
                         uses:None 
                },true, true, Some(CollectionDetails::V1 {size:0}))?;
                
                create_master_edition_v3(CpiContext::new_with_signer(ctx.accounts.token_metadata_program.to_account_info(),CreateMasterEditionV3{
                    edition:ctx.accounts.master_edition.to_account_info(),
                    mint:ctx.accounts.collection_mint.to_account_info(),
                    update_authority:ctx.accounts.collection_mint.to_account_info(),
                    payer:ctx.accounts.signer.to_account_info(),
                    token_program:ctx.accounts.token_program.to_account_info(),
                    system_program:ctx.accounts.system_program.to_account_info(),
                    rent:ctx.accounts.rent.to_account_info(),
                    metadata:ctx.accounts.metadata.to_account_info(),
                    mint_authority:ctx.accounts.collection_mint.to_account_info()
                }, signer_seeds),Some(0))?;
                
                sign_metadata(CpiContext::new_with_signer(ctx.accounts.token_metadata_program.to_account_info(),SignMetadata{creator:ctx.accounts.collection_mint.to_account_info(),metadata:ctx.accounts.metadata.to_account_info()}, signer_seeds))?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account(init,space=8+TokenLottery::INIT_SPACE,payer=signer,seeds=[b"token_lottery"],
bump)]
pub token_lottery:Account<'info,TokenLottery>,
pub system_program:Program<'info,System>

}
#[derive(Accounts)]
pub struct Buyticket<'info>{
    #[account(mut)]
    pub payer:Signer<'info>,
    #[account(mut,
    seeds=[b"token_lottery".as_ref()],
    bump=token_lottery.bump
    )]
    pub token_lottery:Account<'info,TokenLottery>,
    #[account(init,payer=payer,seeds=[token_lottery.total_tickets.to_le_bytes().as_ref()],bump,
    mint::decimals=0,mint::authority=collection_mint,mint::freeze_authority=collection_mint,mint::token_program=token_program
   )]
   pub ticket_mint:Account<'info,Mint>,
   #[account(
    mut,
    seeds = [
        b"metadata", 
        token_metadata_program.key().as_ref(),
        ticket_mint.key().as_ref()
    ],
    bump,
    seeds::program = token_metadata_program.key()
)]
/// CHECK: This account is checked by the metadata program
pub ticket_metadata: UncheckedAccount<'info>,
#[account(
    mut,
    seeds = [
        b"metadata", 
        token_metadata_program.key().as_ref(),
        ticket_mint.key().as_ref(),
        b"edition"
    ],
    bump,
    seeds::program = token_metadata_program.key()
)]
/// CHECK: This account is checked by the metadata program
pub ticket_master_edition: UncheckedAccount<'info>,
#[account(
    mut,
    seeds = [
        b"metadata", 
        token_metadata_program.key().as_ref(),
        collection_mint.key().as_ref()
    ],
    bump,
    seeds::program = token_metadata_program.key()
)]
/// CHECK: This account is checked by the metadata program
pub collection_metadata: UncheckedAccount<'info>,
#[account(
    mut,
    seeds = [
        b"metadata", 
        token_metadata_program.key().as_ref(),
        collection_mint.key().as_ref(),
        b"edition"
    ],
    bump,
    seeds::program = token_metadata_program.key()
)]
/// CHECK: This account is checked by the metadata program
pub collection_master_edition: UncheckedAccount<'info>,
#[account(init,payer=payer,associated_token::mint=ticket_mint,associated_token::authority=payer,associated_token::token_program=token_program)]
pub destination:Account<'info,TokenAccount>,
   #[account(mut,seeds=[b"collectionmint".as_ref()],bump)]
   pub collection_mint:Account<'info,Mint>,
       pub system_program:Program<'info,System>,
       pub associated_token_program:Program<'info,AssociatedToken>,
       pub token_metadata_program: Program<'info, Metadata>,
    pub token_program:Program<'info,Token>,
    pub rent: Sysvar<'info, Rent>,
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
pub struct  CommitRandomness<'info>{
    #[account(mut)]
    pub  payer:Signer<'info>,
    #[account(mut,
        seeds=[b"token_lottery".as_ref()],
        bump=token_lottery.bump)]
        pub token_lottery:Account<'info,TokenLottery>,
/// CHECK:This account is checked by the SWitchboard smart contract
        pub randomness_account:UncheckedAccount<'info>

}
#[derive(Accounts)]

pub struct InitializeToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority =collection_mint,
        seeds = [b"collectionmint"],
        bump
    )]
    pub collection_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = collection_mint,
        associated_token::authority = signer
    )]
    pub collection_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            b"metadata", 
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: This account is checked by the metadata program
    pub metadata: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"metadata", 
            token_metadata_program.key().as_ref(),
            collection_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    /// CHECK: This account is checked by the metadata program
    pub master_edition: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
#[error_code]
pub enum  ErrorCode {
    #[msg("Lootery is not open")]
    LotteryNotOpen,
    #[msg("Not authorized")]
    Notauthorized,
    #[msg("Randomness alredy revealed")]
    RandomnessAlredyRevealed
}