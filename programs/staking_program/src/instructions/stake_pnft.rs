use crate::{constants::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
use mpl_token_metadata::instructions::TransferV1CpiBuilder;

use std::mem::size_of;

#[derive(Accounts)]
// #[instruction(global_bump: u8, staked_nft_bump: u8)]
pub struct StakePnft<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [RS_PREFIX.as_bytes()],
        bump,
        constraint = pool_account.is_initialized == true,
        constraint = pool_account.paused == false,
    )]
    pub pool_account: Account<'info, PoolConfig>,

    #[account(mut)]
    pub user_nft_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = owner,
        seeds = [RS_STAKE_SEED.as_ref(), nft_mint.key().as_ref()],
        bump,
        token::mint = nft_mint,
        token::authority = pool_account,
    )]
    pub dest_nft_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = owner,
        seeds = [RS_STAKEINFO_SEED.as_ref(), nft_mint.key().as_ref()],
        bump,
        space = 8 + size_of::<StakeInfo>(),
    )]
    pub nft_stake_info_account: Account<'info, StakeInfo>,

    pub nft_mint: Account<'info, Mint>,

    /// CHECK: Checking in program
    pub token_program: Program<'info, Token>,
    /// CHECK: Checking in program
    pub update_authority: AccountInfo<'info>,
    /// CHECK: Checking in program
    pub sysvar_instructions: AccountInfo<'info>,
    /// CHECK: Checking in program
    pub token_metadata_program: AccountInfo<'info>,
    /// CHECK: Checking in program
    pub spl_ata_program: AccountInfo<'info>,
    /// CHECK: Checking in program
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn stake_pnft(ctx: Context<StakePnft>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    // set stake info
    let staking_info = &mut ctx.accounts.nft_stake_info_account;
    staking_info.nft_addr = ctx.accounts.nft_mint.key();
    staking_info.owner = ctx.accounts.owner.key();
    staking_info.stake_time = timestamp;
    staking_info.last_update_time = timestamp;
    // staking_info.class_id = class_id;

    // set global info
    ctx.accounts.pool_account.staked_nft += 1;
  
    TransferV1CpiBuilder::new(&ctx.accounts.token_metadata_program)
        .token(&ctx.accounts.user_nft_token_account.to_account_info())
        .token_owner(&ctx.accounts.owner)
        .destination_token(&ctx.accounts.dest_nft_token_account.to_account_info())
        .destination_owner(&ctx.accounts.owner)
        .mint(&ctx.accounts.nft_mint.to_account_info())
        //.metadata(&ctx.accounts.metadata)
        .authority(&ctx.accounts.update_authority)
        .payer(&ctx.accounts.owner)
        .system_program(&ctx.accounts.system_program)
        .sysvar_instructions(&ctx.accounts.sysvar_instructions)
        .spl_token_program(&ctx.accounts.token_program)
        .spl_ata_program(&ctx.accounts.spl_ata_program)
        .amount(1)
        .invoke()?;

    Ok(())
}

