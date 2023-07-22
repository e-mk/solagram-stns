use anchor_lang::prelude::*;

declare_id!("986HMwKkWJRdF15ywXYaMJX1sqV14psxoon7hZ43Mtm");

#[program]
mod solagram_db {
    use super::*;

    pub fn add_user_watched_accounts(
        ctx: Context<AddUserWatchedAccount>,
        tg_user_id: String,
        watched_accounts: String,
    ) -> Result<()> {
        msg!("Watched Accounts Created");
        msg!("tg_user_id: {}", tg_user_id);
        msg!("watched_accounts: {}", watched_accounts);

        if watched_accounts.is_empty() {
            msg!("watched_accounts string is empty");
            return err!(ErrorCode::InvalidWatchedAccounts);
        }

        if tg_user_id.is_empty() {
            msg!("tg_user_id is empty");
            return err!(ErrorCode::InvalidTgUserId);
        }

        let user_watch_accounts = &mut ctx.accounts.user_watch_accounts;
        user_watch_accounts.user = ctx.accounts.initializer.key();
        user_watch_accounts.tg_user_id = tg_user_id;
        user_watch_accounts.watched_accounts = watched_accounts;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(tg_user_id:String, watched_accounts:String)]
pub struct AddUserWatchedAccount<'info> {
    #[account(
        init,
        seeds = [tg_user_id.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 32 + 4 + tg_user_id.len() + 4 + watched_accounts.len()
    )]
    pub user_watch_accounts: Account<'info, UserWatchedAccounts>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserWatchedAccounts {
    pub user: Pubkey,             // 32
    pub tg_user_id: String,       // 4 + len()
    pub watched_accounts: String, // 4 + len()
}

#[error_code]
pub enum ErrorCode {
    #[msg("Empty watched_accounts string")]
    InvalidWatchedAccounts,

    #[msg("Empty tg_user_id string")]
    InvalidTgUserId,
}
