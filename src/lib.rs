use anchor_lang::prelude::*;

pub mod constant;
pub mod states;
use crate::{constant::*, states::*};

declare_id!("A81QxcQJrfZprjbNkvC9a9gSi2huQu45bSDsECTX2aJE");

#[program]
pub mod med_marketplace {
    use super::*;

    pub fn initialize_user(
        ctx: Context<InitializeUser>,
        name: String,
        email: String,
        phone: String,
        address: String,
        role: String,
    ) -> Result<()> {
        // Initialize user profile with default data

        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.name = name;
        user_profile.email = email;
        user_profile.phone = phone;
        user_profile.address = address;
        user_profile.role = role;
        user_profile.product_count = 0;
        user_profile.last_product_idx = 0;

        Ok(())
    }

    pub fn add_product(
        ctx: Context<AddProduct>,
        name: String,
        description: String,
        price: String,
        image_url: String,
        quantity: u64,
    ) -> Result<()> {
        let product_account = &mut ctx.accounts.product_account;
        let user_profile = &mut ctx.accounts.user_profile;

        // Fill contents with argument
        product_account.authority = ctx.accounts.authority.key();
        product_account.idx = user_profile.last_product_idx;
        product_account.name = name;
        product_account.description = description;
        product_account.price = price;
        product_account.image_url = image_url;
        product_account.quantity = quantity;

        // Increase airbnb idx for PDA
        user_profile.last_product_idx = user_profile.last_product_idx.checked_add(1).unwrap();

        // Increase total airbnb count
        user_profile.product_count = user_profile.product_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn update_product(
        ctx: Context<UpdateProduct>,
        _idx: u8,
        price: String,
        quantity: u64,
    ) -> Result<()> {
        let product_account = &mut ctx.accounts.product_account;

        // Mark todo
        product_account.price = price;
        product_account.quantity = quantity;
        Ok(())
    }

    pub fn remove_product(ctx: Context<RemoveProduct>, _idx: u8) -> Result<()> {
        // Decreate total airbnb count
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.product_count = user_profile.product_count.checked_sub(1).unwrap();

        // No need to decrease last airbnb idx

        // Todo PDA already closed in context

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddProduct<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [PRODUCT_TAG, authority.key().as_ref(), &[user_profile.last_product_idx]],
        bump,
        payer = authority,
        space = 4657 + 8,
    )]
    pub product_account: Box<Account<'info, ProductAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(idx: u8)]
pub struct UpdateProduct<'info> {
    #[account(
        mut,
        seeds = [PRODUCT_TAG, authority.key().as_ref(), &[idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub product_account: Box<Account<'info, ProductAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(idx: u8)]
pub struct RemoveProduct<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        close = authority,
        seeds = [PRODUCT_TAG, authority.key().as_ref(), &[idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub product_account: Box<Account<'info, ProductAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
