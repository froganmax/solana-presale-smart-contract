const fs = require("fs");
const anchor = require("@coral-xyz/anchor");
const web3 = require("@solana/web3.js");
//import type { TokenPresale } from "../target/types/token_presale";
//import type { TokenPresale } from "../target/types/token_presale";

async function main() {
  const connection = new web3.Connection(
    web3.clusterApiUrl("devnet"),
    "confirmed"
  );
  const wallet = new anchor.Wallet(web3.Keypair.generate());
  const provider = new anchor.AnchorProvider(connection, wallet, {
    preflightCommitment: "confirmed",
  });
  anchor.setProvider(provider);

  //const idl = JSON.parse(fs.readFileSync('../target/idl/token_presale.json', 'utf8'));
  const idl = JSON.parse(
    fs.readFileSync("../target/idl/test_program.json", "utf8")
  );
  const programId = new anchor.web3.PublicKey(
    "CzvDqDn7oCVwh9fHYDRCVhXqFDtqDxbNfJrk3mYYKKHn"
  );
  const program = new anchor.Program(idl, programId);
  console.log("program " + program);
  //await program.rpc.initialize();
}

main()
  .then(() => console.log("Success"))
  .catch((err) => console.error(err));

// import * as web3 from "@solana/web3.js";
// import * as anchor from "@coral-xyz/anchor";
// import type { TokenPresale } from "../target/types/token_presale";

// // Configure the client to use the local cluster
// anchor.setProvider(anchor.AnchorProvider.env());

// const program = anchor.workspace.TokenPresale as anchor.Program<TokenPresale>;

// // Client
//console.log("My address:", program.provider.publicKey.toString());
//const balance = await program.provider.connection.getBalance(program.provider.publicKey);
// console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);
