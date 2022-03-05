use anchor_lang::prelude::*;
use solana_program::borsh::try_from_slice_unchecked;
use crate::{error};
use crate::InitPlayer;
use crate::TheWheelAccount;
use crate::GameAccount;
use solana_program::program::invoke_signed;
use solana_program::system_instruction::create_account;
use crate::id;


// Context : Player has requested to participate a game identified by a sessions number.
//              It is now necessary to open a Player PDA Account
// Operations:
//  - check thewheel PDA account
//  - check player PDA account
//  - check player is not already in pending list
//  - create player PDA account
//  - add player to pending list
pub fn initplayer_in_rs(ctx: Context<InitPlayer>, session: u8) -> ProgramResult {

    msg!("initplayer_in_rs pre0");
    let player = &mut ctx.accounts.player;
    msg!("initplayer_in_rs pre1");
    let thewheelaccount = &mut ctx.accounts.thewheelaccount;
    msg!("initplayer_in_rs pre2");
    let playeraccount = &mut ctx.accounts.playeraccount;
    msg!("initplayer_in_rs pre3");
    let gameaccount = &mut ctx.accounts.gameaccount;

    msg!("initplayer_in_rs D0 -- check thewheel PDA Account");
    let (thewheel_account_inner , _) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref()],
        &id()
    );

    msg!("initplayer_in_rs D1");
    if thewheel_account_inner != *thewheelaccount.key  {
        return Err(error::TheWheelError::InvalidTheWheelAccount.into())
    }

    msg!("initplayer_in_rs D2 -- check player PDA Account");
    let (player_account_inner , player_bump_inner) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref(),&[session],player.key.as_ref()],
        &id()
    );

    msg!("initplayer_in_rs D3");
    if player_account_inner != *playeraccount.key  {
        return Err(error::TheWheelError::InvalidPlayerAccount.into())
    }

    msg!("initplayer_in_rs D4 -- check data in thewheel PDA Account");
    let mut thewheel_pda_state = try_from_slice_unchecked::<TheWheelAccount>(&thewheelaccount.data.borrow())?;

    msg!("initplayer_in_rs D5 -- count players in pendinglist for session");
    let mut playersinsession : u8 = 0;
    for (_, value) in &thewheel_pda_state.pendingmap {
        if *value == session{
            playersinsession = playersinsession + 1;
        }
    }

    msg!("initplayer_in_rs D6 -- player on plending list");
    if thewheel_pda_state.pendingmap.contains_key(&player.key()) {
        return Err(error::TheWheelError::PlayerInPendingList.into())
    }

    msg!("initplayer_in_rs D7 -- check max player is not reached");
    let mut game_pda_state = try_from_slice_unchecked::<GameAccount>(&gameaccount.data.borrow())?;

    msg!("initplayer_in_rs D8");
    if !game_pda_state.ledger.contains_key(&player.key()) {
        if game_pda_state.max_players <= game_pda_state.players_in_game + playersinsession {
            return Err(error::TheWheelError::MaxPlayerLimitReach.into())
        }
    }

    msg!("initplayer_in_rs D9 -- create player PDA Account");
    let create_player_pda_ix = &create_account(
        player.key,
        playeraccount.key,
        500000, // a minimum
        1,
        &id()
    );

    msg!("initplayer_in_rs D10 -- sign player PDA Account");
    invoke_signed(
        create_player_pda_ix
    ,
        &[
            player.clone().to_account_info(),
            playeraccount.clone(),
        ]
    ,
        &[
            &[
                b"thewheel",
                id().as_ref(),
                &[session],
                &player.key().as_ref(),
                &[player_bump_inner],
            ],
        ]
    )?;



    msg!("initplayer_in_rs D11 -- free thewheel PDA Account");
    thewheel_pda_state.pendingmap.insert(player.key().clone(),session);

    msg!("initplayer_in_rs D12 -- free thewheel PDA Account");
    thewheel_pda_state.serialize(&mut &mut thewheelaccount.data.borrow_mut()[..])?;

    msg!("initplayer_in_rs D13 -- update game PDA Account");
    game_pda_state.players_in_game = game_pda_state.players_in_game + 1;

    msg!("initplayer_in_rs D14 -- free game PDA Account");
    game_pda_state.serialize(&mut &mut gameaccount.data.borrow_mut()[..])?;

    msg!("initplayer_in_rs D15 -- end");
    Ok(())

 }
