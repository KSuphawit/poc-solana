use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, MintTo, Token, TokenAccount, Transfer},
};

declare_id!("2pRe5i7Py9mxuQGm9xuh6BuD4SN9K1adwYr5qzAmVSU2");

#[program]
pub mod poc_solana {
    use super::*;

    pub fn init_mint(_ctx: Context<InitializeMint>, _token_mint_bump: u8) -> ProgramResult { Ok(()) }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> ProgramResult {
        let (pda, pda_bump) = Pubkey::find_program_address(&["token".as_ref()], ctx.program_id);

        anchor_spl::token::mint_to(
            ctx.accounts.into_mint_to_context().with_signer(&[&["token".as_ref(), &[pda_bump]]]),
            amount,
        );

        Ok(())
    }

    pub fn deposit_token(ctx: Context<DepositToken>, amount: u64) -> ProgramResult {

        // let (pda, pda_bump) = Pubkey::find_program_address(&["token".as_ref()], ctx.program_id);

        anchor_spl::token::transfer(
            ctx.accounts.into_transfer_context(),
            amount
        );

        // anchor_spl::token::mint_to(
        //     ctx.accounts.into_mint_to_context().with_signer(&[&["token".as_ref(), &[token_mint_bump]]]),
        //     amount,
        // );

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

impl <'info> DepositToken<'info> {
    fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_deposit_token_assoc_token_acct.to_account_info().clone(),
            to: self.program_deposit_token_assoc_token_acct.to_account_info().clone(),
            authority: self.user.to_account_info().clone()
        };

        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }

    // fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
    //
    //     let cpi_accounts = MintTo {
    //         mint: self.return_token.to_account_info().clone(),
    //         to: self.user_return_token_assoc_token_acct.to_account_info().clone(),
    //         authority: self.return_token.to_account_info().clone(),
    //     };
    //
    //     CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    // }
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
pub struct MintToken<'info> {
    #[account(mut)]
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
pub struct DepositToken<'info> {
    pub deposit_token: Account<'info, Mint>,
    // #[account(mut)]
    // pub return_token: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = deposit_token,
        associated_token::authority = program
    )]
    pub program_deposit_token_assoc_token_acct: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_deposit_token_assoc_token_acct: Account<'info, TokenAccount>,
    // #[account(
    //     init_if_needed,
    //     payer = user,
    //     associated_token::mint = return_token,
    //     associated_token::authority = user
    // )]
    // pub user_return_token_assoc_token_acct:  Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}