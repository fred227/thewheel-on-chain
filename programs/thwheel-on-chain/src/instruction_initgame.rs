use anchor_lang::prelude::*;
use solana_program::system_instruction::*;
use solana_program::program::invoke_signed;
use solana_program::borsh::try_from_slice_unchecked;
use crate::{error};
use crate::TheWheelAccount;
use crate::GameAccount;
use crate::InitGame;
use std::{collections::BTreeMap};
use crate::id;

use solana_program::rent::Rent;

use crate::MIN_PLAYERS_IN_GAME;
use crate::MAX_PLAYERS_IN_GAME;
use crate::SPACE_FOR_GAMEPDA;

//Context : creation of a new game. 
//Before the call : player has checked in TheWheel PDA public account if one session is available.
//Operations in Instruction :
//  - check thewheel PDA account
//  - check game PDA account
//  - check inputs
//  - create game pda account 
//  - initialize fields
//  - update the wheel pda account
pub fn initgame_in_rs(ctx: Context<InitGame>, session: u8, launchingdate_insec: i64, maxplayers:u8) -> ProgramResult {

    msg!("initgame_in_rs pre0");
    let creator_of_game = &mut ctx.accounts.creatorgame; // pubkey of creator
    msg!("initgame_in_rs pre1");
    let thewheelaccount = &mut ctx.accounts.thewheelaccount; // thewheel pda account
    msg!("initgame_in_rs pre2");
    let gameaccount = &mut ctx.accounts.gameaccount; // game pda account

    msg!("initgame_in_rs D0 -- check Thewheel PDA account");
    let (thewheel_account_inner , _) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref()],
        &id()
    );

    msg!("initgame_in_rs D1");
    if thewheel_account_inner != *thewheelaccount.key  {
        return Err(error::TheWheelError::InvalidTheWheelAccount.into())
    }

    msg!("initgame_in_rs D2 -- check game PDA account");
    let (game_account_inner , game_bump_inner) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref(),&[session]],
        &id()
    );

    msg!("initgame_in_rs D3");
    if game_account_inner != *gameaccount.key  {
            return Err(error::TheWheelError::InvalidGameAccount.into())
    }

    msg!("initgame_in_rs D4");
    let mut thewheel_pda_state = try_from_slice_unchecked::<TheWheelAccount>(&thewheelaccount.data.borrow())?;
    
    msg!("initgame_in_rs D5 -- check if session is available");
    if session == 0 {
        return Err(error::TheWheelError::InvalidInput.into())
    }

    msg!("initgame_in_rs D5");
    let mut sessionavailable : bool = false;
    let mut sessionkey : usize = 0;
    for x in 0..thewheel_pda_state.arrarysession.len() {
        if thewheel_pda_state.arrarysession[x] == session {
            return Err(error::TheWheelError::SessionNotValid.into())
        }
        if thewheel_pda_state.arrarysession[x] == 0 {
            sessionavailable = true;
            sessionkey = x;
        }
    }

    msg!("initgame_in_rs D6");
    if sessionavailable == false{
        return Err(error::TheWheelError::SessionNotAvailable.into())
    }

    msg!("initgame_in_rs D7 -- check launchind date");
    let clock = Clock::get()?;
    if launchingdate_insec-120 < clock.unix_timestamp {
        return Err(error::TheWheelError::InvalidInput.into())
    }

    msg!("initgame_in_rs D7b");
    if 60*60*24*30 + clock.unix_timestamp < launchingdate_insec {
        return Err(error::TheWheelError::LaunchingDateTooHigh.into())
    }

    msg!("initgame_in_rs D8 -- check maxplayer");
    if u16::from(maxplayers) < MIN_PLAYERS_IN_GAME {
        return Err(error::TheWheelError::InvalidInput.into())
    }

    msg!("initgame_in_rs D9 -- check minplayer");
    if MAX_PLAYERS_IN_GAME < u16::from(maxplayers) {
        return Err(error::TheWheelError::InvalidInput.into())
    }

    msg!("initgame_in_rs D10 -- compute rent");
    let rentgame : u64 = Rent::default().minimum_balance(SPACE_FOR_GAMEPDA.into()) ;

    msg!("initgame_in_rs D11 -- create Player PDA Account");
    let create_game_pda_ix = &create_account(
        creator_of_game.key,
        gameaccount.key,
        rentgame,
        SPACE_FOR_GAMEPDA.into(),
        &id()
    );

    msg!("initgame_in_rs D12 -- sign Player PDA Account");
    invoke_signed(
        create_game_pda_ix
    ,
        &[
            creator_of_game.to_account_info().clone(),
            gameaccount.clone(),
        ]
    ,
        &[
            &[
                b"thewheel",
                id().as_ref(),
                &[session],
                &[game_bump_inner],
            ],
        ]
    )?;
    
    
    msg!("initgame_in_rs D13 -- initialize Player PDA Account");
    //initialization of PDA game account
    let mut gameaccount_pda_state = try_from_slice_unchecked::<GameAccount>(&gameaccount.data.borrow()).unwrap();

    gameaccount_pda_state.is_initialized = 1;
    gameaccount_pda_state.is_lock = 0;

    gameaccount_pda_state.winner = Pubkey::new_from_array([0;32]);

    gameaccount_pda_state.sessionnumber = session;
    gameaccount_pda_state.max_players = maxplayers;
    gameaccount_pda_state.players_in_game = 1;
    gameaccount_pda_state.launching_date = launchingdate_insec;

    let empty_map: BTreeMap<Pubkey, u64> = BTreeMap::new();
    gameaccount_pda_state.ledger = empty_map;
    gameaccount_pda_state.ledger.insert(creator_of_game.key().clone(),rentgame);

    gameaccount_pda_state.serialize(&mut &mut gameaccount.data.borrow_mut()[..])?;

    msg!("initgame_in_rs D14 -- update TheWheel PDA Account");
    thewheel_pda_state.arrarysession[sessionkey] = session;
    thewheel_pda_state.serialize(&mut &mut thewheelaccount.data.borrow_mut()[..])?;

    msg!("initgame_in_rs D15 -- end");
    Ok(())

 }