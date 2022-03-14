# thewheel-on-chain

before using the on-chain program, it is first important to initialize TheWheel PDA Account.
to do so user must run `anchor test` command with the following configuration in the thewheel-on-chain.ts file :
```
  let initthewheel_test : boolean = true;
  let initgame_test : boolean =false;
  let initplayer_test : boolean = false;
  let confirmdeposit_test: boolean = false;
  let play_test : boolean =false;
  let logzone: boolean = false;
  ```
