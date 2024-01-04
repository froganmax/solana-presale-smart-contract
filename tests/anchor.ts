import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as token from '@solana/spl-token';
const SOLANA = require('@solana/web3.js');
const { Connection, PublicKey, LAMPORTS_PER_SOL, clusterApiUrl } = SOLANA;
import * as anchor from "@coral-xyz/anchor";
import type { TokenPresale } from "../target/types/token_presale";

const WALLET_SECRET_KEY = JSON.parse(require("fs").readFileSync("/Users/gic_owner/.config/solana/id.json", "utf8")).slice(0,32);;
console.log("This is the Wallet Secret", WALLET_SECRET_KEY);
const ADMIN_WALLET_ADDRESS_STRING = web3.Keypair.fromSeed(new Uint8Array(WALLET_SECRET_KEY)).publicKey.toString();
const ADMIN_WALLET_ADDRESS_PUB_KEY = new PublicKey(ADMIN_WALLET_ADDRESS_STRING);
console.log("This is the wallet", ADMIN_WALLET_ADDRESS_STRING);
const SOLANA_CONNECTION = new Connection("http://127.0.0.1:8899");

describe("Airdrop SOL", () => {
  // Configure the client to use the local cluster
  const AIRDROP_AMOUNT = 1 * LAMPORTS_PER_SOL; // 1 SOL 

  it("Airdroping 1 SOL", async () => {
     // Airdrop 2 SOL to the admin account
    console.log(`Requesting airdrop for ${ADMIN_WALLET_ADDRESS_STRING}`)
    const signature = await SOLANA_CONNECTION.requestAirdrop(
      ADMIN_WALLET_ADDRESS_PUB_KEY,
      AIRDROP_AMOUNT
    );
    const { blockhash, lastValidBlockHeight } = await SOLANA_CONNECTION.getLatestBlockhash();
    await SOLANA_CONNECTION.confirmTransaction({
        blockhash,
        lastValidBlockHeight,
        signature
    },'finalized');
    console.log(`Airdrop complete with Tx: https://explorer.solana.com/tx/${signature}?cluster=devnet`)
    assert.ok(signature);
  })
});

describe("Create Random Token", () => {
  const program = anchor.workspace.TokenPresale as anchor.Program<TokenPresale>;
  const tokenVault = new web3.Keypair();
  const adminAccount = new web3.Keypair();
  const escrowAccount = new web3.Keypair();
  const presaleAccount = new web3.Keypair(); 

  it("Create token", async () => {
    // Create a new SPL token
    const newToken = await token.createMint(
      anchor.getProvider().connection,
      ADMIN_WALLET_ADDRESS_PUB_KEY,
      ADMIN_WALLET_ADDRESS_PUB_KEY.publicKey,
      null,
      9,
    );

    // Mint 100 tokens to the admin account
    const tokenAccount = await token.createAccount(
      anchor.getProvider().connection,
      ADMIN_WALLET_ADDRESS_PUB_KEY,
      ADMIN_WALLET_ADDRESS_PUB_KEY.publicKey,
      null,
    );

    // const ata = await getAssociatedTokenAddress(tokenAccount, receiveAddress);

    await token.mintTo(
      anchor.getProvider().connection,
      ADMIN_WALLET_ADDRESS_PUB_KEY,
      ADMIN_WALLET_ADDRESS_PUB_KEY.publicKey,
      tokenAccount, 
      newToken,
      100,
    );

    const txHash = await program.methods
    .initialize()
    .accounts({
      admin: ADMIN_WALLET_ADDRESS_PUB_KEY.publicKey,
      tokenMint: newToken,
      tokenVault: tokenVault.publicKey,
      adminAccount: adminAccount.publicKey,
      escrowAccount: escrowAccount.publicKey,
      presaleAccount: presaleAccount.publicKey,
      systemProgram: web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenProgram: token.TOKEN_PROGRAM_ID,
    })
    .signers([ADMIN_WALLET_ADDRESS_PUB_KEY])
    .rpc();

    // Confirm transaction
    const confirmation = await anchor.getProvider().connection.confirmTransaction(txHash);
    console.log(`Transaction ${confirmation.value.err ? 'failed' : 'succeeded'}`);

    const fetchedAdminAccount = await program.account.adminAccount.fetch(
      adminAccount.publicKey
    );
    assert.ok(fetchedAdminAccount);

  //   async function getTokenAccounts(wallet: string, solanaConnection: web3.Connection) {
  //     const filters:web3.GetProgramAccountsFilter[] = [
  //         {
  //           dataSize: 165,    //size of account (bytes)
  //         },
  //         {
  //           memcmp: {
  //             offset: 32,     //location of our query in the account (bytes)
  //             bytes: wallet,  //our search criteria, a base58 encoded string
  //           },            
  //         }];
  //     const accounts = await solanaConnection.getParsedProgramAccounts(
  //         token.TOKEN_PROGRAM_ID, //new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
  //         {filters: filters}
  //     );
  //     console.log(`Found ${accounts.length} token account(s) for wallet ${wallet}.`);
  //     accounts.forEach((account, i) => {
  //         //Parse the account data
  //         const parsedAccountInfo:any = account.account.data;
  //         const mintAddress:string = parsedAccountInfo["parsed"]["info"]["mint"];
  //         const tokenBalance: number = parsedAccountInfo["parsed"]["info"]["tokenAmount"]["uiAmount"];
  //         //Log results
  //         console.log(`Token Account No. ${i + 1}: ${account.pubkey.toString()}`);
  //         console.log(`--Token Mint: ${mintAddress}`);
  //         console.log(`--Token Balance: ${tokenBalance}`);
  //     });
  // }
  // getTokenAccounts(admin,solanaConnection);

    // Log the completion of the initialization
    console.log('Initialization completed successfully');

    // Log the connection to the devnet
    console.log(`Connected to ${anchor.getProvider().connection.rpcEndpoint}`);
  })
  
  it("Initialize", async () => {

  });
});