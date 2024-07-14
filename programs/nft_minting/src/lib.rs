use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Token, MintTo};

declare_id!("5ADurQGboE85A3k58sMkQZtC7AtCDvg18S4g35Yun5QR");

#[program]
pub mod nft_minting {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNft>, metadata: String) -> Result<()> {
         let mint_key = *ctx.accounts.mint.to_account_info().key;
        
        // Borrow mint and token_account as mutable
        let mint = &mut ctx.accounts.mint;
        let token_account = &mut ctx.accounts.token_account;

        token::mint_to(
            ctx.accounts.into_mint_to_context(),
            1,
        )?;

        // Borrow metadata as mutable and set the metadata
        let nft_metadata = &mut ctx.accounts.metadata;
        nft_metadata.mint = mint_key;
        nft_metadata.data = metadata;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(init, payer = user, mint::decimals = 0, mint::authority = user)]
    pub mint: Account<'info, Mint>,
    #[account(init, payer = user, token::mint = mint, token::authority = user)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(init, payer = user, space = 8 + 32 + 64)]
    pub metadata: Account<'info, NftMetadata>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct NftMetadata {
    pub mint: Pubkey,
    pub data: String,
}

impl<'info> MintNft<'info> {
    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info().clone(),
            to: self.token_account.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}
