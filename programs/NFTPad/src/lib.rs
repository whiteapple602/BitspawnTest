use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod basic_0 {
    use super::*;

    pub fn create_augment(_ctx: Context<NewAugmentNFT>, owner: Pubkey,addr: Pubkey, name: String, capacity: u16, character: &Vec<Pubkey>, account_bump: u8) -> ProgramResult {
        let user = &ctx.accounts.user;
        let list = &mut ctx.accounts.list;

        if list.possible_characters.len() >= capacity as usize {
            return Err(NFTPadError::ListFull.into());
        }

        list.creator = user.to_account_info().key;
        list.owner = owner;
        list.published_addr = addr;
        list.name = name;
        list.capacity = capacity;
        list.possible_characters = *character.clone();

        // Move the bounty to the account. We account for the rent amount that Anchor's init
        // already transferred into the account.
        let account_lamports = **list.to_account_info().lamports.borrow();
        let transfer_amount = bounty
            .checked_sub(account_lamports)
            .ok_or(NFTPadError::BountyTooSmall)?;

        if transfer_amount > 0 {
            invoke(
                &transfer(
                    user.to_account_info().key,
                    list.to_account_info().key,
                    transfer_amount,
                ),
                &[
                    user.to_account_info(),
                    list.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        Ok(())
    }
}

#[error]
pub enum NFTPadError {
    #[msg("Augment is not attached to Character")]
    NotAttached,
    #[msg("This list is full")]
    ListFull,
    #[msg("Bounty must be enough to mark account rent-exempt")]
    BountyTooSmall,
}

fn name_seed(name: &str) -> &[u8] {
    let b = name.as_bytes();
    if b.len() > 32 {
        &b[0..32]
    } else {
        b
    }
}
#[derive(Accounts)]
#[instruction(name: String, capacity: u16, character: &Vec<Pubkey>, bump: u8)]
pub struct NewAugmentNFT<'info> {
    #[account(init,
        payer=user,
        space=AugmentNFT::space(&name, capacity),
        seeds=[
            b"todolist",
            user.to_account_info().key.as_ref(),
            name_seed(&name)
        ],
        bump=list_bump)]
    pub list: Account<'info, AugmentNFT>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
pub struct CharacterNFT {
    pub creator: Pubkey,
    pub owner: Pubkey,
    pub attached_augments: Vec<Pubkey>,
}

#[account]
pub struct AugmentNFT {
    pub creator: Pubkey,
    pub owner: Pubkey,
    pub published_addr: Pubkey,
    pub name: String,
    pub capacity: u16,
    pub possible_characters: Vec<Pubkey>,
}

impl AugmentNFT {
    fn space(name: &str, capacity: u16) -> usize {
        // discriminator + onwerpubkey + name string + capacity
        8 + 32 + (4 + name.len()) + 2 +
        // + vec of items
        (4 + (capacity as usize) * std::mem::size_of::<Pubkey>())
    }
}