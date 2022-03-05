import * as anchor from '@project-serum/anchor';
import { Connection,PublicKey,ConfirmOptions} from '@solana/web3.js';
const { SystemProgram } = anchor.web3;
import * as borsh from "@project-serum/borsh";
import { useAnchorWallet } from '@solana/wallet-adapter-react';
import { Program, Provider } from '@project-serum/anchor';

 const MAX_SESSIONS = 9;
 const MAX_WINNERS = 5;
 const MAXPLAYER : number = 15;

const PDATHEWHEEL_DATA_LAYOUT = borsh.struct([
  borsh.u8("is_initialized"),
  borsh.i64("time_initialization"),
  borsh.array(borsh.u8("session"),MAX_SESSIONS,"sessionmap"),
  borsh.map(borsh.u8("session"),borsh.publicKey("player"), "winners"),
  borsh.map(borsh.publicKey("player"), borsh.u8("session"), "pendingmap"),
]);


const PDAGAME_DATA_LAYOUT = borsh.struct([
  borsh.u8("is_initialized"),
  borsh.u8("is_lock"),
  borsh.publicKey("winner"),
  borsh.u8("sessionnumber"),
  borsh.u8("max_players"),
  borsh.u8("players_in_game"),
  borsh.i64("launchingdate"),
  borsh.map(borsh.publicKey("player"), borsh.u64("lamports"), "ledger"),
]);


describe('TheWheelOfficial3', () => {

  // Configure the client to use the local cluster.
 // const provider = anchor.Provider.env();
  
  const opts : ConfirmOptions = {  preflightCommitment: 'confirmed', }
  let endpoint = "https://api.devnet.solana.com";//"http://localhost:8899"
  const connection = new Connection(endpoint, "confirmed");
  const AnchorWallet = useAnchorWallet();
  const provider = new Provider( connection, AnchorWallet!, opts );
  anchor.setProvider(provider); 


  const idl = JSON.parse(
    require("fs").readFileSync("./target/idl/the_wheel_official3.json", "utf8")
  );
  
  // Address of the deployed program.
  const programId = new anchor.web3.PublicKey("DijskFGWfs99TPjkMFG1TtAqxBbLYxWd1Z7qnJMVgWpf");
  
  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId,provider);
  
  const  sessionnumber = 202;
  var uint8 = new Uint8Array(1);
  uint8[0] = sessionnumber;
  const  sessionbuffer : Buffer =  Buffer.from(uint8);

  

  let initthewheel_test : boolean = false;
  let initgame_test : boolean =false;
  let initplayer_test : boolean = false;
  let confirmdeposit_test: boolean = false;
  let play_test : boolean =false;
  let logzone: boolean = true;


  // publickeys to test
  const baseAccountPublic = new PublicKey("D6CXVPddn1KdeH9Bw19sE2hXJHQ7V6VZjfp7xbpzTAnW");
  const baseAccountPublic2 = new PublicKey("ANBQLzgo9UfqtqaeDSiaNrJVGNwK4DDxewMkYjRiqjLT");
  const baseAccountPublic3 = new PublicKey("42QhMHwjYDZYvySaqFNawxD8nbddfkJqaTkWertrqY5u");

  const TheWheelPDAAccount =new Promise<[PublicKey,number]>( async (r,e)  =>{
    let [thewheelAccount, thewheelBump ] =  await anchor.web3.PublicKey
    .findProgramAddress( [Buffer.from("thewheel"),program.programId.toBuffer()],program.programId );
    return r([thewheelAccount,thewheelBump]);
  });

  
 const GamePDAAccount = new Promise<PublicKey>( async (r,e) =>{
    let [GameAccount, ] =  await anchor.web3.PublicKey
    .findProgramAddress( [Buffer.from("thewheel"),program.programId.toBuffer(),sessionbuffer],program.programId );
    return r(GameAccount);
  });


  const PlayerPDAAccount  = new Promise<PublicKey>( async (r,e)  =>{
    let [PlayerAccount, ] =  await anchor.web3.PublicKey
    .findProgramAddress( [Buffer.from("thewheel"),program.programId.toBuffer(),sessionbuffer, provider.wallet.publicKey.toBuffer()],program.programId );
    return r(PlayerAccount);
  });



//
//  Initialize The Wheel by creating main PDA Account
//

  if (initthewheel_test){
    it('Is initialized!', async () => {

      TheWheelPDAAccount.then( async (value) => {

      const tx = await program.rpc.initialize(value[1],{     
        accounts: {
          creator: provider.wallet.publicKey ,
          thewheelaccount: value[0],
          systemProgram: SystemProgram.programId,
      },});

      console.log("Your transaction signature", tx);

    } );
    });
  }


//
//  Init Game 
//

  if (initgame_test){
    it('Init Game!',  () => {

      Promise.all([TheWheelPDAAccount,GamePDAAccount]).then( async (values) => {
      const launchingdate = new Date().getTime() + 1000;

      console.log("values[0][0]=",values[0][0].toString());

        const tx = await program.rpc.initgame(sessionnumber,new anchor.BN(launchingdate),MAXPLAYER,{    
          accounts: {
            creatorgame: provider.wallet.publicKey ,
            thewheelaccount: values[0][0],
            gameaccount : values[1],
            systemProgram: SystemProgram.programId,
        },});

        console.log("Your transaction signature", tx);
      } );
    });
  }


//
//  Init Player : player requests to participate to a game
//

if (initplayer_test){
  it('Init Player!', () => {

    Promise.all([TheWheelPDAAccount,GamePDAAccount,PlayerPDAAccount]).then( async (values) => {

      const tx = await program.rpc.initplayer(sessionnumber,{    
        accounts: {
          player: provider.wallet.publicKey ,
          thewheelaccount: values[0][0],
          gameaccount : values[1],
          playeraccount : values[2],
          systemProgram: SystemProgram.programId,
      },});

      console.log("Your transaction signature", tx);
    } );
  });
}


//
//  Confirm deposit : player has transfer money and ask confirmation
//



if (confirmdeposit_test){
  it('Confirm deposit!', () => {

    Promise.all([TheWheelPDAAccount,GamePDAAccount,PlayerPDAAccount]).then( async (values) => {

      const tx = await program.rpc.confirmdeposit(sessionnumber,{    
        accounts: {
          player: provider.wallet.publicKey ,
          thewheelaccount: values[0][0],
          gameaccount : values[1],
          playeraccount : values[2],
      },});
  
      console.log("Your transaction signature", tx);
    } );
  });
}

if (play_test){
  it('Play Game!', async () => {

    Promise.all([TheWheelPDAAccount,GamePDAAccount]).then( async (values) => {

      const tx = await program.rpc.play(sessionnumber,{    
        accounts: {
          thewheelaccount: values[0][0],
          gameaccount : values[1],
      },});

      console.log("Your transaction signature", tx);
    } );
  });
}

if (logzone){

  it('log test!', async () => {

    Promise.all([TheWheelPDAAccount,GamePDAAccount]).then( async (values) => {

    console.log("logzone provider.wallet.publicKey=",provider.wallet.publicKey.toString());
    console.log("logzone thewheelAccount=",values[0][0].toString());
    console.log("logzone GameAccount=",values[1].toString());

    const connection = new Connection("http://localhost:8899", "confirmed");

    const TheWheelAccountInfo = await connection.getAccountInfo(values[0][0]);
    const TheWheelAccountState = PDATHEWHEEL_DATA_LAYOUT.decode(
      TheWheelAccountInfo.data
    );

    const GameAccountInfo = await connection.getAccountInfo(values[1]);
   const GameAccountState = PDAGAME_DATA_LAYOUT.decode(
      GameAccountInfo.data
    );


   let pending : Map<PublicKey,number> = TheWheelAccountState.pendingmap;
    console.log("after pending",pending.size)
    for (const [key, value] of pending.entries()) {
        console.log(`session = ${key} = ${value}`);
    }

    let session : Map<number,number>= TheWheelAccountState.sessionmap;
    for (const [key, value] of session.entries()) {
      console.log(`session = ${key} = ${value}`);
    }

    let winnersmap : Map<number,PublicKey>= TheWheelAccountState.winners;
    for (const [key, value] of winnersmap.entries()) {
      console.log(`winners = ${key} = ${value}`);
    } 

    console.log("new Date().getTime()=",new Date().getTime())
    console.log("is_initialized=",GameAccountState.is_initialized)
    console.log("is_lock=",GameAccountState.is_lock)
    console.log("winner=",GameAccountState.winner.toString())
    console.log("sessionnumber=",GameAccountState.sessionnumber)
    console.log("max_players=",GameAccountState.max_players)
    console.log("players_in_game=",GameAccountState.players_in_game)
    let launchingdate_bn : anchor.BN = GameAccountState.launchingdate;
    console.log("launching_date=",(launchingdate_bn.toNumber() * 1000))
    console.log("diff time = ", (launchingdate_bn.toNumber() * 1000) -new Date().getTime() )


    //console.log(GameAccountState.);
    let mymap : Map<PublicKey,anchor.BN> = GameAccountState.ledger;
    for (const [key, value] of mymap.entries()) {
      if (0 <value.toNumber()){
        console.log(`ledger = ${key.toString()} = ${value.toNumber()}`);
      }
    }

 
  } );
});
}



});