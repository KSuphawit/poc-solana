use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, MintTo, Token, TokenAccount},
};

declare_id!("2pRe5i7Py9mxuQGm9xuh6BuD4SN9K1adwYr5qzAmVSU2");

#[program]
pub mod poc_solana {
    use super::*;

    pub fn init_mint(_ctx: Context<InitializeMint>, _token_mint_bump: u8) -> ProgramResult { Ok(()) }

    pub fn init_user_associated_token_acc(_ctx: Context<InitializeUserAssociatedTokenAcc>) -> ProgramResult { Ok(()) }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64, token_mint_bump: u8) -> ProgramResult {

        anchor_spl::token::mint_to(
            ctx.accounts.into_mint_to_context().with_signer(&[&["token".as_ref(), &[token_mint_bump]]]),
            amount,
        );

        Ok(())
    }
}

impl<'info> MintToken<'info> {
    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.token_mint.to_account_info().clone(),
            to: self.user_assoc_token_acct.to_account_info().clone(),
            authority: self.token_mint.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

#[derive(Accounts)]
#[instruction(token_mint_bump: u8)]
pub struct InitializeMint<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        seeds = ["token".as_ref()],
        bump = token_mint_bump,
        mint::decimals = 9,
        mint::authority = token_mint
    )]
    pub token_mint: Account<'info, Mint>,
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeUserAssociatedTokenAcc<'info> {
    pub token_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_mint,
        associated_token::authority = payer
    )]
    pub user_assoc_token_acct: Account<'info, TokenAccount>,
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = payer
    )]
    pub user_assoc_token_acct: Account<'info, TokenAccount>,
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
