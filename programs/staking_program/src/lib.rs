use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("ABxTX1onDRNRfQHadz2mE49e8pN84hrSfSx6cggaxiJk");

#[program]
pub mod disks_staking_program {
    use super::*;

    pub fn initialize_staking_pool(
        ctx: Context<InitializeStakingPool>,
        reward_per_week: u16,
    ) -> Result<()> {
        initialize::initialize_staking_pool(ctx, reward_per_week)
    }

    pub fn stake_nft(ctx: Context<StakeNft>) -> Result<()> {
        stake::stake_nft(ctx)
    }

    pub fn stake_pnft(ctx: Context<StakePnft>) -> Result<()> {
        stake_pnft::stake_pnft(ctx)
    }

    pub fn unstake_nft(ctx: Context<WithdrawNft>) -> Result<()> {
        unstake::unstake_nft(ctx)
    }

    pub fn deposit_credits(ctx: Context<DepositCredits>, amount: u64) -> Result<()> {
        // Transfer reward tokens into the vault.
        deposit_reward::handle(ctx, amount)
    }

    pub fn withdraw_credits(ctx: Context<WithdrawCredits>) -> Result<()> {
        withdraw_reward::handle(ctx)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        claim_reward::claim_reward(ctx)
    }

    pub fn change_reward_mint(ctx: Context<ChangeRewardMint>, reward_mint: Pubkey) -> Result<()> {
        update_token_mint::handle(ctx, reward_mint)
    }

    pub fn change_pool_setting(
        ctx: Context<ChangePoolSetting>,
        reward_per_week: u16,
        paused: bool,
    ) -> Result<()> {
        update_config::handle(ctx, reward_per_week, paused)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>, new_admin: Pubkey) -> Result<()> {
        transfer_ownership::handle(ctx, new_admin)
    }
}
