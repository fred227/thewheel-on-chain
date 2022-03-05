use anchor_lang::prelude::*;
use crate::GameAccount;
use crate::TheWheelAccount;
use solana_program::borsh::try_from_slice_unchecked;
use crate::GetPaid;
use crate::id;
use crate::{error};

// Context : winner ask to get money
// Operations:
//  - check thewheel PDA account
//  - check game PDA account
//  - check lauching date < clock
//  - check game PDA account lock
//  - check player == winner
//  - transfer money

pub fn getpaid_in_rs(ctx: Context<GetPaid>, session: u8) -> ProgramResult {

    msg!("getpaid_in_rs pre0");
    let player = &mut ctx.accounts.player;
    msg!("getpaid_in_rs pre1");
    let thewheelaccount = &mut ctx.accounts.thewheelaccount;
    msg!("getpaid_in_rs pre2");
    let gameaccount = &mut ctx.accounts.gameaccount;
    msg!("getpaid_in_rs pre3");
    let _system_program = &mut ctx.accounts.system_program;

    msg!("getpaid_in_rs D0");
    //check if The wheel PDA sent by player is the real one
    let (thewheel_account_inner , _) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref()],
        &id()
    );
    msg!("getpaid_in_rs D1");
    if thewheel_account_inner != *thewheelaccount.key  {
        return Err(error::TheWheelError::InvalidTheWheelAccount.into())
    }
    msg!("getpaid_in_rs D2");
    //check if game PDA sent by player is the real one
    let (game_account_inner , _) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref(), &[session]],
        &id()
    );
    msg!("getpaid_in_rs D3");
    if game_account_inner != *gameaccount.key  {
        return Err(error::TheWheelError::InvalidGameAccount.into())
    }
    msg!("getpaid_in_rs D4");
    let mut thewheel_pda_state = try_from_slice_unchecked::<TheWheelAccount>(&thewheelaccount.data.borrow()).unwrap();
    
    msg!("getpaid_in_rs D5");
    let game_pda_state = try_from_slice_unchecked::<GameAccount>(&gameaccount.data.borrow()).unwrap();

    msg!("getpaid_in_rs D6");
    if game_pda_state.is_lock!=1 {
        return Err(error::TheWheelError::ErrorOnPendingList.into())
    }
    
    msg!("getpaid_in_rs D7");
    if game_pda_state.winner.key() !=  *player.key{
        return Err(error::TheWheelError::ErrorOnPendingList.into())
    }

    msg!("getpaid_in_rs D8");
    game_pda_state.serialize(&mut &mut gameaccount.data.borrow_mut()[..])?;

    msg!("getpaid_in_rs D9 - transfert lamports game PDA Account to player");
    let to_account_lamports = player.lamports();
    **player.lamports.borrow_mut() = to_account_lamports
        .checked_add(gameaccount.lamports())
        .unwrap();
    **gameaccount.lamports.borrow_mut() = 0;
    let mut from_account_data = gameaccount.data.borrow_mut();
    from_account_data.fill(0);

    msg!("getpaid_in_rs D10 - erase session in thewheel_pda_state.arrarysession");
    for x in 0..thewheel_pda_state.arrarysession.len() {
        if thewheel_pda_state.arrarysession[x] == session {
            thewheel_pda_state.arrarysession[x] = 0;
        }
    }

    msg!("getpaid_in_rs D11 - erase winner in thewheel_pda_state.winners");
    thewheel_pda_state.winners.remove(&session);

    msg!("getpaid_in_rs D12");
    thewheel_pda_state.serialize(&mut &mut thewheelaccount.data.borrow_mut()[..])?;

    msg!("getpaid_in_rs D10 - end");
    Ok(())
}