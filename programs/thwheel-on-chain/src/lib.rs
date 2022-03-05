use anchor_lang::prelude::*;
use std::{collections::BTreeMap};
use borsh::{BorshSerialize, BorshDeserialize};
mod error;
mod instruction_initialize;
mod instruction_initgame;
mod instruction_initplayer;
mod instruction_confirmdeposit;
mod instruction_play;
mod instruction_getpaid;
mod instruction_reopengame;

declare_id!("39D9W9evHuroXaBg7P48Z3ovJsyYvr4LUN7P8V9oGJ1Y");

const MIN_PLAYERS_IN_GAME: u16 = 2;
const MAX_PLAYERS_IN_GAME: u16 = 15;
const MAX_SESSIONS_IN_THEWHEEL: u16 = 9;
const MIN_LAMPORTS_IN_PLAYER: u64 = 10000000;
const LIMIT_TIME_FOR_DEPOSIT_INSEC: i64 =  60*60*2;// ;60*60*24; // 1 hours
const NEW_TIMEGAME_INSEC: i64 = 60*60*7;// ;60*60*7; // 1 week

#[program]
pub mod the_wheel_official3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, thewheelaccount_bump: u8 ) -> ProgramResult {
        crate::instruction_initialize::initialize_in_rs(ctx, thewheelaccount_bump)
    }

    pub fn initgame(ctx: Context<InitGame>, session: u8, launchingdate: i64, maxplayers:u8) -> ProgramResult {
        crate::instruction_initgame::initgame_in_rs(ctx, session, launchingdate, maxplayers)
    }

    pub fn initplayer(ctx: Context<InitPlayer>, session: u8) -> ProgramResult {
        crate::instruction_initplayer::initplayer_in_rs(ctx, session)
    }

    pub fn confirmdeposit(ctx: Context<ConfirmDeposit>, session: u8) -> ProgramResult {
        crate::instruction_confirmdeposit::confirmdeposit_in_rs(ctx, session)
    }

    pub fn play(ctx: Context<Play>, session: u8) -> ProgramResult {
        crate::instruction_play::play_in_rs(ctx, session)
    }

    pub fn getpaid(ctx: Context<GetPaid>, session: u8) -> ProgramResult {
        crate::instruction_getpaid::getpaid_in_rs(ctx, session)
    }

    pub fn reopengame(ctx: Context<Play>, session: u8 ) -> ProgramResult {
        crate::instruction_reopengame::reopengame_in_rs(ctx,session)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub creator: Signer<'info>,
    #[account(mut)]
    pub thewheelaccount: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct InitGame<'info> {
    pub creatorgame: Signer<'info>,
    #[account(mut)]
    pub thewheelaccount: AccountInfo<'info>,
    #[account(mut)]
    pub gameaccount: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct GetPaid<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub thewheelaccount: AccountInfo<'info>,
    #[account(mut)]
    pub gameaccount: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct InitPlayer<'info> {
    pub player: Signer<'info>,
    #[account(mut)]
    pub thewheelaccount: AccountInfo<'info>,
    #[account(mut)]
    pub playeraccount: AccountInfo<'info>,
    #[account(mut)]
    pub gameaccount: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ConfirmDeposit<'info> {
    pub player: Signer<'info>,
    #[account(mut)]
    pub thewheelaccount: AccountInfo<'info>,
    #[account(mut)]
    pub gameaccount: AccountInfo<'info>,
    #[account(mut)]
    pub playeraccount: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub thewheelaccount: AccountInfo<'info>,
    #[account(mut)]
    pub gameaccount: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    pub from: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
}

const SPACE_FOR_THEWHEELPDA : u16 = 1 + 8
+ ( 1 * MAX_SESSIONS_IN_THEWHEEL ) // pub arrarysession: [u8; 9]
 + ( 4 + ( 1 + 32 ) * MAX_SESSIONS_IN_THEWHEEL ) //  pub winners: BTreeMap<u8, Pubkey>, 
  + ( 4 + ( 32 + 1 ) * MAX_PLAYERS_IN_GAME * MAX_SESSIONS_IN_THEWHEEL ); // pub pendingmap: BTreeMap<Pubkey, u8>
// space = 1 + 8 + ( 1 * 9 ) + ( 4 + ( 1 + 32 ) * 15 ) + ( 4 + ( 32 + 1 ) * 15 * 9 ) = 4976 limit
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TheWheelAccount { // PDA = "thewheel"+id()
    pub is_initialized: u8,
    pub time_initialization: i64,
    pub arrarysession: [u8; 9], // games available max = 9
    pub winners: BTreeMap<u8, Pubkey>, 
    pub pendingmap: BTreeMap<Pubkey, u8> // pending list : Pubkey session - value
}

const SPACE_FOR_GAMEPDA : u16 = 1 + 1 + 32 + 1 + 1 + 1 +  8 + ( 4 + ( 32 + 8 ) * MAX_PLAYERS_IN_GAME );
// space = 1 + 1 + 32 + 1 + 1 + 1 +  8 + ( 4 + ( 32 + 8 ) * 15 ) = 649
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct GameAccount { // PDA = "thewheel"+id()+session
    pub is_initialized: u8,
    pub is_lock: u8,  // lock when winner is knew
    pub winner: Pubkey,  // winner pubkey
    pub sessionnumber: u8,
    pub max_players: u8,
    pub players_in_game: u8,
    pub launching_date: i64, // launching date when turn wheel
    pub ledger: BTreeMap<Pubkey, u64> // Pubkey player - lamports deposit max player = 30
}

// space =1
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct PlayerAccount { // PDA = "thewheel"+id()+pubkey player
    pub is_initialized: u8, 
}
