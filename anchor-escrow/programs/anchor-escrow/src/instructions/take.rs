use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, close_account, CloseAccount }
};
use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
    )]
    pub taker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
    )]
    pub maker_mint_b_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        close = taker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
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


impl<'info> Take<'info> {
    pub fn transfer_to_maker(&mut self,) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let receive_amount = self.escrow.receive;
        let cpi_accounts = TransferChecked {
            from: self.taker_mint_b_ata.to_account_info(),
            to: self.maker_mint_b_ata.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_b.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, receive_amount, self.mint_b.decimals)?;
        Ok(())
    }

    pub fn transfer_to_taker(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let deposit_amount = self.vault.amount;
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_mint_a_ata.to_account_info(),
            mint: self.mint_a.to_account_info(),
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
        transfer_checked(cpi_ctx, deposit_amount, self.mint_a.decimals)?;
        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_account = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
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
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_account, seeds);
        close_account(cpi_ctx)?;
        Ok(())
    }
}