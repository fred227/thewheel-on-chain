import { Keypair } from "@solana/web3.js"
import { Connection, LAMPORTS_PER_SOL } from "@solana/web3.js"

const anchor = require("@project-serum/anchor");


module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  let programAuthorityKeypair = new Keypair()

  this.connection = new Connection("https://api.devnet.solana.com", "confirmed")

  const signature = await this.connection.requestAirdrop(
      programAuthorityKeypair.publicKey,
      LAMPORTS_PER_SOL * 5
  )
  await this.connection.confirmTransaction(signature)
  const fs = require('fs');
  const path = require('path');

  const programAuthorityKeyfileName = `deploy/programauthority-keypair.json`
  const programAuthorityKeypairFile = path.resolve(
      `${__dirname}."\\.${programAuthorityKeyfileName}`
  )
  
  // ... [snip]
  
  fs.writeFileSync(
      programAuthorityKeypairFile,
      `[${Buffer.from(programAuthorityKeypair.secretKey.toString())}]`
  )

  

}
