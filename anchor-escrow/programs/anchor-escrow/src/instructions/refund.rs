use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked}};


use crate::state::Escrow;

#[derive(Accounts)]

pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        close = maker,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
   pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Refund<'info> {
    pub fn refund(&mut self,) -> Result<()> {
       let cpi_program = self.token_program.to_account_info();
       let cpi_accounts = TransferChecked {
           from: self.vault.to_account_info(),
           to: self.maker_mint_a_ata.to_account_info(),
           authority: self.escrow.to_account_info(),
           mint: self.mint_a.to_account_info(),    
    };

    let seeds: &[&[&[u8]]] = &[
            &[
                b"escrow",
                self.maker.key.as_ref(),
                &self.escrow.seed.to_le_bytes(),
                &[self.escrow.bump],
            ],
        ];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_a.decimals)?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker_mint_a_ata.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let seeds: &[&[&[u8]]] = &[
            &[
                b"escrow",
                self.maker.key.as_ref(),
                &self.escrow.seed.to_le_bytes(),
                &[self.escrow.bump],
            ],
        ];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
        close_account(cpi_ctx)?;
        Ok(())
    }
}