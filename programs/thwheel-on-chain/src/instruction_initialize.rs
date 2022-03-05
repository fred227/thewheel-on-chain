use anchor_lang::prelude::*;
use solana_program::system_instruction::*;
use solana_program::program::invoke_signed;
use solana_program::borsh::try_from_slice_unchecked;
use crate::Initialize;
use std::collections::BTreeMap;
use crate::TheWheelAccount;
use crate::id;
use crate::{error};

use crate::MAX_SESSIONS_IN_THEWHEEL;
use crate::SPACE_FOR_THEWHEELPDA;

//Context : The Wheel need a main public PDA account. Details of this Account is given in lib.rs
//No check is perform in this instruction as signer has nothing to win to fake input.
pub fn initialize_in_rs(ctx: Context<Initialize>, thewheelaccount_bump: u8 ) -> ProgramResult {

    msg!("initialize_in_rs pre0");
    let creator = &mut ctx.accounts.creator;
    msg!("initialize_in_rs pre1");
    let thewheelaccount_pda = &mut ctx.accounts.thewheelaccount;

    msg!("initialize_in_rs D0 -- check thewhellPDA");
    let (thewheel_account_inner , bump_inner) = Pubkey::find_program_address(
        &[b"thewheel".as_ref(), id().as_ref()],
        &id()
    );

    msg!("thewheel_account_inner={}",thewheel_account_inner.key().to_string());
    msg!("bump_inner={}",bump_inner.to_string());
    msg!("thewheelaccount_pda={}",thewheelaccount_pda.key().to_string());
    msg!("thewheelaccount_bump={}",thewheelaccount_bump.to_string());

    msg!("initialize_in_rs D0b -- compute rent");
    if thewheel_account_inner != *thewheelaccount_pda.key  {
        return Err(error::TheWheelError::InvalidTheWheelAccount.into())
    }

    msg!("initialize_in_rs D0c -- compute rent");
    let rentthewheel : u64 = Rent::default().minimum_balance(SPACE_FOR_THEWHEELPDA.into()) ;

    msg!("initialize_in_rs D1 -- create thewheel PDA Account");
    let create_thewheel_pda_ix = &create_account(
        creator.key,
        thewheelaccount_pda.key,
        rentthewheel,
        SPACE_FOR_THEWHEELPDA.into(),
        &id()
    );

    msg!("initialize_in_rs D2 -- sign creation");
    invoke_signed(
        create_thewheel_pda_ix
        ,
            &[
                creator.clone().to_account_info(),
                thewheelaccount_pda.clone(),
            ]
        ,
            &[
                &[
                    b"thewheel",
                    id().as_ref(),
                    &[thewheelaccount_bump],
                ],
            ]
    )?;

    msg!("initialize_in_rs D3 -- initialize TheWheel PDA Account");
    let mut thewheelaccount_pda_state = try_from_slice_unchecked::<TheWheelAccount>(&thewheelaccount_pda.data.borrow()).unwrap();

    thewheelaccount_pda_state.is_initialized = 1;

    let clock = Clock::get()?;
    thewheelaccount_pda_state.time_initialization = clock.unix_timestamp;

    let myarray: [u8; MAX_SESSIONS_IN_THEWHEEL as usize] = [0; MAX_SESSIONS_IN_THEWHEEL as usize];
    thewheelaccount_pda_state.arrarysession = myarray;

    let winners_map: BTreeMap<u8,Pubkey> = BTreeMap::new();
    thewheelaccount_pda_state.winners = winners_map;

    let empty_map: BTreeMap<Pubkey, u8> = BTreeMap::new();
    thewheelaccount_pda_state.pendingmap = empty_map;

    msg!("initialize_in_rs D4 -- free TheWheel PDA Account");
    thewheelaccount_pda_state.serialize(&mut &mut thewheelaccount_pda.data.borrow_mut()[..])?;

    msg!("initialize_in_rs D5 -- end");
    Ok(())
}