use anchor_lang::prelude::*;
use crate::Play;
use crate::{error};
use solana_program::hash::*;
use std::ops::Div;
use crate::GameAccount;
use crate::TheWheelAccount;
use solana_program::borsh::try_from_slice_unchecked;



// Context : the wheel is turned
// Operations:
//  - check thewheel PDA account
//  - check game PDA account
//  - check lauching date < clock
//

pub fn play_in_rs(ctx: Context<Play>, session : u8) -> ProgramResult {

    let clock = Clock::get()?;
    msg!("clock.slot :{}",clock.slot.to_string());
    msg!("clock.unix_timestamp :{}",clock.unix_timestamp.to_string());
    let diff : i64 = clock.unix_timestamp - (clock.slot as i64);
    let hash1 : Hash = hash(diff.to_string().as_bytes());
    let hash_s : String = hash1.to_string() ; 
    let str_slice: &[u8] =  hash_s.as_bytes();
        
    let mut r : u64  = 1;

    for n in 1..str_slice.len() {
        if let Some(i) = str_slice.get(n) {
            r = r * *i as u64
        }
    }
    
    let mut myfloat : f64 = r as f64;

    while 10.0 < myfloat { 
        myfloat = myfloat.div(10.0);
        if  myfloat < 10.0 {
            msg!("r--= :{}",myfloat);
            while 1.0 < myfloat{
                myfloat -= 1.0;
            }
            msg!("r--= :{}",myfloat);
            break;
        }
        
    }

    msg!("r= :{}",myfloat);

    msg!("D0");
    let thewheelaccount = &mut ctx.accounts.thewheelaccount;
    msg!("D1");
    let gameaccount = &mut ctx.accounts.gameaccount;
    msg!("D2");
    let mut game_pda_state = try_from_slice_unchecked::<GameAccount>(&gameaccount.data.borrow()).unwrap();
   
    if clock.unix_timestamp < game_pda_state.launching_date {
        return Err(error::TheWheelError::GameStillValid.into())
    }
   
   
    msg!("D3");
    let mut thewheel_pda_state = try_from_slice_unchecked::<TheWheelAccount>(&thewheelaccount.data.borrow()).unwrap();
    msg!("D3b");
    let mut total : u64 = 0;
    msg!("D4");
    for (key, value) in game_pda_state.ledger.iter() {
        msg!("{}: {}", key, value);
        total += value;
    }

    msg!("total: {}", total);

    let indicator : f64 = myfloat * total as f64;
    let mut total : u64 = 0;
    let mut winner : Pubkey = Pubkey::new_from_array([0;32]);
    for (key, value) in game_pda_state.ledger.iter() {
        msg!("{}: {}", key, value);
        total += value;
        if indicator < total as f64{
            winner = *key;
            msg!("winner is : {}", winner);
            break;
        }
    }
    if winner != Pubkey::new_from_array([0;32]){
        thewheel_pda_state.winners.insert(session,winner.key().clone());
    }
    game_pda_state.is_lock = 1;
    game_pda_state.winner = winner;
    msg!("indicator: {}", indicator);

    thewheel_pda_state.pendingmap = thewheel_pda_state.pendingmap
                    .into_iter().filter(|&(_, v)| v == session)
                    .collect();

    game_pda_state.serialize(&mut &mut gameaccount.data.borrow_mut()[..])?;
    thewheel_pda_state.serialize(&mut &mut thewheelaccount.data.borrow_mut()[..])?;

    Ok(())
}