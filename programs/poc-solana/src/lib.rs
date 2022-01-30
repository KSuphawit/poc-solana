use std::convert::TryFrom;

use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, MintTo, Token, TokenAccount},
};

declare_id!("2pRe5i7Py9mxuQGm9xuh6BuD4SN9K1adwYr5qzAmVSU2");

#[program]
pub mod poc_solana {
    use super::*;

    pub fn mint_token(ctx: Context<MintToken>, bump_seed: u8) -> ProgramResult {
        let u32_token_decimal = u32::try_from(ctx.accounts.token.decimals).unwrap();

        let pow_decimal = u64::try_from(i32::pow(10, u32_token_decimal)).unwrap();

        let amount = 5;

        let computed_token_amount = amount * pow_decimal;

        anchor_spl::token::mint_to(
            ctx.accounts.into_mint_to_context().with_signer(&[&[&[], &[bump_seed]]]),
            computed_token_amount,
        );

        Ok(())
    }
}

impl<'info> MintToken<'info> {
    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.token.to_account_info().clone(),
            to: self.destination.to_account_info().clone(),
            authority: self.token_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub token: Account<'info, Mint>,
    pub token_authority: AccountInfo<'info>,
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
