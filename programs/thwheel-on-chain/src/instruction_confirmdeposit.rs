use anchor_lang::prelude::*;
use solana_program::borsh::try_from_slice_unchecked;
use crate::{error};
use crate::TheWheelAccount;
use crate::GameAccount;
use crate::ConfirmDeposit;
use crate::id;

use crate::MIN_LAMPORTS_IN_PLAYER;

// Context : Once player has requested to play a game, his PubKey is add to pending list in TheWheel PDA public Account.
// Once player has transfer money to playeraccount, he can then confirm deposit
pub fn confirmdeposit_in_rs(ctx: Context<ConfirmDeposit>, session : u8) -> ProgramResult {

    msg!("confirmdeposit_in_rs pre0");
    let player = &mut ctx.accounts.player;
    msg!("confirmdeposit_in_rs pre1");
    let thewheelaccount = &mut ctx.accounts.thewheelaccount;
    msg!("confirmdeposit_in_rs pre2");
    let gameaccount = &mut ctx.accounts.gameaccount;
    msg!("confirmdeposit_in_rs pre3");
    let playeraccount = &mut ctx.accounts.playeraccount;

    msg!("confirmdeposit_in_rs D0 -- check Thewheel PDA account");
    //check if The wheel PDA sent by player is the real one
    let (thewheel_account_inner , _) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref()],
        &id()
    );

    msg!("confirmdeposit_in_rs D1");
    if thewheel_account_inner != *thewheelaccount.key  {
        return Err(error::TheWheelError::InvalidTheWheelAccount.into())
    }
    
    msg!("confirmdeposit_in_rs D2 -- check Game PDA account");
    let (game_account_inner , _) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref(), &[session]],
        &id()
    );

    msg!("confirmdeposit_in_rs D3");
    if game_account_inner != *gameaccount.key  {
        return Err(error::TheWheelError::InvalidGameAccount.into())
    }

    msg!("confirmdeposit_in_rs D4 -- check Player PDA account");
    let (player_account_inner , _) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref(),&[session],player.key.as_ref()],
        &id()
    );

    msg!("confirmdeposit_in_rs D5");
    if player_account_inner != *playeraccount.key  {
        return Err(error::TheWheelError::InvalidPlayerAccount.into())
    }

    msg!("confirmdeposit_in_rs D6 -- check player is on pending list");
    let mut thewheel_pda_state = try_from_slice_unchecked::<TheWheelAccount>(&thewheelaccount.data.borrow()).unwrap();
   
    msg!("confirmdeposit_in_rs D7");
    if !thewheel_pda_state.pendingmap.contains_key(&player.key()) {
        return Err(error::TheWheelError::ErrorOnPendingList.into())
    }else{
        if let Some((_,b)) = thewheel_pda_state.pendingmap.get_key_value(&player.key()) {
            if *b != session {
                return Err(error::TheWheelError::ErrorOnPendingList.into())
                
            }
        }
    }

    msg!("confirmdeposit_in_rs D8 -- check lamport on Player PDA Account");
    let lamportstodeposit = playeraccount.lamports();

    msg!("confirmdeposit_in_rs D9");
    if  lamportstodeposit < MIN_LAMPORTS_IN_PLAYER.into() {
        return Err(error::TheWheelError::InsufficientDeposit.into())
    }

    msg!("confirmdeposit_in_rs D10 -- make transfer");
    let dest_lamports = gameaccount.lamports();
    **gameaccount.lamports.borrow_mut() = dest_lamports
        .checked_add(playeraccount.lamports())
        .unwrap();
    **playeraccount.lamports.borrow_mut() = 0;

    msg!("confirmdeposit_in_rs D11");
    let mut source_data = playeraccount.data.borrow_mut();
    source_data.fill(0);
    
    msg!("confirmdeposit_in_rs D12 -- remove player from pending list");
    thewheel_pda_state.pendingmap.remove(&player.key());

    msg!("confirmdeposit_in_rs D13 -- free thewheelaccount");
    thewheel_pda_state.serialize(&mut &mut thewheelaccount.data.borrow_mut()[..])?;

    msg!("confirmdeposit_in_rs D14 -- update Game PDA Account");
    let mut game_pda_state = try_from_slice_unchecked::<GameAccount>(&gameaccount.data.borrow()).unwrap();

    msg!("confirmdeposit_in_rs D15");
    game_pda_state.ledger.entry(player.key()).and_modify(|e| { *e += lamportstodeposit })
    .or_insert(lamportstodeposit);

    msg!("confirmdeposit_in_rs D17 -- free gameaccount");
    game_pda_state.serialize(&mut &mut gameaccount.data.borrow_mut()[..])?;

    msg!("confirmdeposit_in_rs D18 -- end");
    Ok(())
 }