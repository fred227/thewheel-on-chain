
use anchor_lang::prelude::*;
use crate::Play;
use crate::{error};
use crate::id;
use solana_program::borsh::try_from_slice_unchecked;
use std::{collections::BTreeMap};

use crate::TheWheelAccount;
use crate::GameAccount;

use crate::LIMIT_TIME_FOR_DEPOSIT_INSEC;
use crate::NEW_TIMEGAME_INSEC;

// Close any Account by transfering all lamports to TheWheel PDA public Account
 pub fn reopengame_in_rs(ctx: Context<Play>, session : u8) -> ProgramResult {
    
    msg!("reopengame_in_rs pre0");
    let thewheelaccount = &mut ctx.accounts.thewheelaccount;
    msg!("reopengame_in_rs pre1");
    let gameaccount = &mut ctx.accounts.gameaccount;

    msg!("reopengame_in_rs D0 -- check thewheel PDA Account");
    let (thewheel_account_inner , _) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref()],
        &id()
    );

    msg!("reopengame_in_rs D1");
    if thewheel_account_inner != *thewheelaccount.key  {
        return Err(error::TheWheelError::InvalidTheWheelAccount.into())
    }

    msg!("reopengame_in_rs D2 -- check player PDA Account");
    let (game_account_inner , _) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref(), &[session]],
        &id()
    );

    msg!("reopengame_in_rs D3");
    if game_account_inner != *gameaccount.key  {
        return Err(error::TheWheelError::InvalidGameAccount.into())
    }

    msg!("reopengame_in_rs D4 -- update game PDA account");
    let mut game_pda_state = try_from_slice_unchecked::<GameAccount>(&gameaccount.data.borrow()).unwrap();
    
    let clock = Clock::get()?;

    msg!("reopengame_in_rs D5 -- check if game is still valid");
    if   clock.unix_timestamp < game_pda_state.launching_date + LIMIT_TIME_FOR_DEPOSIT_INSEC { // + LIMIT_TIME_FOR_DEPOSIT {
        msg!("clock.unix_timestamp = {}",clock.unix_timestamp);
        msg!("game_pda_state.launching_date = {}",game_pda_state.launching_date);
        msg!("LIMIT_TIME_FOR_DEPOSIT_INSEC = {}",LIMIT_TIME_FOR_DEPOSIT_INSEC);
        msg!("game_pda_state.launching_date + LIMIT_TIME_FOR_DEPOSIT_INSEC - clock.unix_timestamp ={}",game_pda_state.launching_date + LIMIT_TIME_FOR_DEPOSIT_INSEC-clock.unix_timestamp);
        return Err(error::TheWheelError::GameStillValid.into())
    }

    msg!("reopengame_in_rs D6 -- reinitialization");
    game_pda_state.is_lock=0;
    game_pda_state.winner = Pubkey::new_from_array([0;32]);
    game_pda_state.players_in_game = 0;
    game_pda_state.launching_date = clock.unix_timestamp + NEW_TIMEGAME_INSEC;

    let empty_map_gameledger: BTreeMap<Pubkey, u64> = BTreeMap::new();
    game_pda_state.ledger = empty_map_gameledger;

    msg!("reopengame_in_rs D7 -- remove winner");
    let mut thewheel_pda_state = try_from_slice_unchecked::<TheWheelAccount>(&thewheelaccount.data.borrow()).unwrap();
        thewheel_pda_state.winners.remove(&session);
    thewheel_pda_state.serialize(&mut &mut thewheelaccount.data.borrow_mut()[..])?;
 
    msg!("reopengame_in_rs D8 -- free gameaccount");
    game_pda_state.serialize(&mut &mut gameaccount.data.borrow_mut()[..])?;

    msg!("reopengame_in_rs D9 -- end");
    Ok(())

 }
