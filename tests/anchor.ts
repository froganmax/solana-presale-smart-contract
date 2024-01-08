import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as token from "@solana/spl-token";
const SOLANA = require("@solana/web3.js");
const { Connection, PublicKey, LAMPORTS_PER_SOL, clusterApiUrl } = SOLANA;
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import * as anchor from "@coral-xyz/anchor";
import type { TokenPresale } from "../target/types/token_presale";
import walletLists from "./wallets.json";

const SOL_VAULT_SEED = "presale-escrow-vault";
const ADMIN_MANAGE_SEED = "admin-role";
const USER_ACCOUNT_SEED = "user-role";
const PRESALE_INFO_SEED = "presale-info";
const TOKEN_VAULT_SEED = "token-vault";

const wallet = NodeWallet.local();

console.log("This is the Wallet Secret", wallet);
const ADMIN_WALLET_ADDRESS_STRING = wallet.publicKey.toString();
const ADMIN_WALLET_ADDRESS_PUB_KEY = new PublicKey(ADMIN_WALLET_ADDRESS_STRING);
console.log("This is the wallet", ADMIN_WALLET_ADDRESS_STRING);
const SOLANA_CONNECTION = new Connection("http://127.0.0.1:8899");

let newToken: web3.PublicKey;
let tokenAccount: web3.PublicKey;
let tokenVault: web3.PublicKey;
let adminAccount: web3.PublicKey;
let escrowAccount: web3.PublicKey;
let presaleAccount: web3.PublicKey;
let userAccount: web3.PublicKey;
let userWallet = web3.Keypair.generate();

describe("Airdrop SOL", () => {
  // Configure the client to use the local cluster
  const AIRDROP_AMOUNT = 1 * LAMPORTS_PER_SOL; // 1 SOL

  it("Airdroping 1 SOL to admin wallet", async () => {
    // Airdrop 2 SOL to the admin account
    console.log(
      `Requesting airdrop for admin wallet - ${ADMIN_WALLET_ADDRESS_STRING}`
    );
    const signature = await SOLANA_CONNECTION.requestAirdrop(
      ADMIN_WALLET_ADDRESS_PUB_KEY,
      AIRDROP_AMOUNT
    );
    const { blockhash, lastValidBlockHeight } =
      await SOLANA_CONNECTION.getLatestBlockhash();
    await SOLANA_CONNECTION.confirmTransaction(
      {
        blockhash,
        lastValidBlockHeight,
        signature,
      },
      "finalized"
    );
    console.log(
      `Airdrop complete with Tx: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
    assert.ok(signature);
  });

  it("Airdroping 1 SOL to user wallet", async () => {
    // Airdrop 2 SOL to the admin account
    console.log(
      `Requesting airdrop for user wallet - ${userWallet.publicKey.toString()}`
    );
    const signature = await SOLANA_CONNECTION.requestAirdrop(
      userWallet.publicKey,
      AIRDROP_AMOUNT
    );
    const { blockhash, lastValidBlockHeight } =
      await SOLANA_CONNECTION.getLatestBlockhash();
    await SOLANA_CONNECTION.confirmTransaction(
      {
        blockhash,
        lastValidBlockHeight,
        signature,
      },
      "finalized"
    );
    console.log(
      `Airdrop complete with Tx: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
    assert.ok(signature);
  });
});

describe("Create Random Token", () => {
  const program = anchor.workspace.TokenPresale as anchor.Program<TokenPresale>;

  it("Initialize", async () => {
    // Create a new SPL token
    newToken = await token.createMint(
      anchor.getProvider().connection,
      wallet.payer,
      ADMIN_WALLET_ADDRESS_PUB_KEY,
      null,
      9
    );
    console.log("Temp token address", newToken.toString());

    // Mint 100 tokens to the admin account
    tokenAccount = await token.createAccount(
      anchor.getProvider().connection,
      wallet.payer,
      newToken,
      ADMIN_WALLET_ADDRESS_PUB_KEY,
      null,
      null,
      token.TOKEN_PROGRAM_ID
    );
    console.log("Associated token address for admin", tokenAccount.toString());

    // const ata = await getAssociatedTokenAddress(tokenAccount, receiveAddress);

    await token.mintTo(
      anchor.getProvider().connection,
      wallet.payer,
      newToken,
      tokenAccount,
      wallet.payer,
      100,
      [],
      null,
      token.TOKEN_PROGRAM_ID
    );
    console.log("Minted tokens successfully");

    [tokenVault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(TOKEN_VAULT_SEED), newToken.toBuffer()],
      program.programId
    );
    [adminAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(ADMIN_MANAGE_SEED), ADMIN_WALLET_ADDRESS_PUB_KEY.toBuffer()],
      program.programId
    );
    [escrowAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(SOL_VAULT_SEED)],
      program.programId
    );
    [presaleAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(PRESALE_INFO_SEED)],
      program.programId
    );

    const txHash = await program.methods
      .initialize()
      .accounts({
        admin: ADMIN_WALLET_ADDRESS_PUB_KEY,
        tokenMint: newToken,
        tokenVault: tokenVault,
        adminAccount: adminAccount,
        escrowAccount: escrowAccount,
        presaleAccount: presaleAccount,
        systemProgram: web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: token.TOKEN_PROGRAM_ID,
      })
      .signers([])
      .rpc();

    // Confirm transaction
    const confirmation = await anchor
      .getProvider()
      .connection.confirmTransaction(txHash);
    console.log(
      `Transaction ${confirmation.value.err ? "failed" : "succeeded"}`
    );

    const fetchedAdminAccount =
      await program.account.adminAccount.fetch(adminAccount);
    console.log(fetchedAdminAccount);
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
    console.log("Initialization completed successfully");

    // Log the connection to the devnet
    console.log(`Connected to ${anchor.getProvider().connection.rpcEndpoint}`);
  });

  it("Add thousands of whilelist wallets", async () => {
    for (let i = 0; i < walletLists.length; i++) {
      [userAccount] = web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from(USER_ACCOUNT_SEED),
          new web3.PublicKey(walletLists[i]).toBuffer(),
        ],
        program.programId
      );
      const txHash = await program.methods
        .addWhitelist()
        .accounts({
          admin: ADMIN_WALLET_ADDRESS_PUB_KEY,
          adminAccount: adminAccount,
          presaleAccount: presaleAccount,
          userAccount: userAccount,
          authority: new web3.PublicKey(walletLists[i]),
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([])
        .rpc();

      // Confirm transaction
      const confirmation = await anchor
        .getProvider()
        .connection.confirmTransaction(txHash);
      console.log(
        `Transaction ${confirmation.value.err ? "failed" : "succeeded"}`
      );

      const fetchedPresaleAccount =
        await program.account.presaleAccount.fetch(presaleAccount);
      console.log(fetchedPresaleAccount);
      assert.ok(fetchedPresaleAccount);

      const fetchedUserAccount =
        await program.account.userAccount.fetch(userAccount);
      console.log(fetchedUserAccount);
      assert.ok(fetchedUserAccount);

      // Log the completion of the initialization
      console.log("Adding whitelist completed successfully");

      // Log the connection to the devnet
      console.log(
        `Connected to ${anchor.getProvider().connection.rpcEndpoint}`
      );
    }
  });

  it("Add Whilelist for one wallet - we can test buying token feature using this wallet.", async () => {
    [userAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(USER_ACCOUNT_SEED), userWallet.publicKey.toBuffer()],
      program.programId
    );
    const txHash = await program.methods
      .addWhitelist()
      .accounts({
        admin: ADMIN_WALLET_ADDRESS_PUB_KEY,
        adminAccount: adminAccount,
        presaleAccount: presaleAccount,
        userAccount: userAccount,
        authority: userWallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([])
      .rpc();

    // Confirm transaction
    const confirmation = await anchor
      .getProvider()
      .connection.confirmTransaction(txHash);
    console.log(
      `Transaction ${confirmation.value.err ? "failed" : "succeeded"}`
    );

    const fetchedPresaleAccount =
      await program.account.presaleAccount.fetch(presaleAccount);
    console.log(fetchedPresaleAccount);
    assert.ok(fetchedPresaleAccount);

    const fetchedUserAccount =
      await program.account.userAccount.fetch(userAccount);
    console.log(fetchedUserAccount);
    assert.ok(fetchedUserAccount);

    // Log the completion of the initialization
    console.log("Adding whitelist completed successfully");

    // Log the connection to the devnet
    console.log(`Connected to ${anchor.getProvider().connection.rpcEndpoint}`);
  });

  it("Update presale info", async () => {
    [userAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(USER_ACCOUNT_SEED), userWallet.publicKey.toBuffer()],
      program.programId
    );
    const txHash = await program.methods
      .updatePresalePeriod(new BN(100), new BN(10000), new BN(99999999999999))
      .accounts({
        admin: ADMIN_WALLET_ADDRESS_PUB_KEY,
        adminAccount: adminAccount,
        presaleAccount: presaleAccount,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([])
      .rpc();

    // Confirm transaction
    const confirmation = await anchor
      .getProvider()
      .connection.confirmTransaction(txHash);
    console.log(
      `Transaction ${confirmation.value.err ? "failed" : "succeeded"}`
    );

    const fetchedPresaleAccount =
      await program.account.presaleAccount.fetch(presaleAccount);
    console.log(fetchedPresaleAccount);
    assert.ok(fetchedPresaleAccount);

    const fetchedUserAccount =
      await program.account.userAccount.fetch(userAccount);
    console.log(fetchedUserAccount);
    assert.ok(fetchedUserAccount);

    // Log the completion of the initialization
    console.log("Updating presale info completed successfully");

    // Log the connection to the devnet
    console.log(`Connected to ${anchor.getProvider().connection.rpcEndpoint}`);
  });

  it("Buy tokens", async () => {
    [userAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(USER_ACCOUNT_SEED), userWallet.publicKey.toBuffer()],
      program.programId
    );
    const txHash = await program.methods
      .buyToken(new BN(10))
      .accounts({
        escrowAccount: escrowAccount,
        presaleAccount: presaleAccount,
        userAccount: userAccount,
        authority: userWallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([userWallet])
      .rpc();

    // Confirm transaction
    const confirmation = await anchor
      .getProvider()
      .connection.confirmTransaction(txHash);
    console.log(
      `Transaction ${confirmation.value.err ? "failed" : "succeeded"}`
    );

    const fetchedPresaleAccount =
      await program.account.presaleAccount.fetch(presaleAccount);
    console.log(fetchedPresaleAccount);
    assert.ok(fetchedPresaleAccount);

    const fetchedUserAccount =
      await program.account.userAccount.fetch(userAccount);
    console.log(fetchedUserAccount);
    assert.ok(fetchedUserAccount);

    // Log the completion of the initialization
    console.log("Buying token completed successfully");

    // Log the connection to the devnet
    console.log(`Connected to ${anchor.getProvider().connection.rpcEndpoint}`);
  });

  it("Remove Whilelist", async () => {
    [userAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(USER_ACCOUNT_SEED), userWallet.publicKey.toBuffer()],
      program.programId
    );
    const txHash = await program.methods
      .removeWhitelist()
      .accounts({
        admin: ADMIN_WALLET_ADDRESS_PUB_KEY,
        adminAccount: adminAccount,
        presaleAccount: presaleAccount,
        userAccount: userAccount,
        authority: userWallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([])
      .rpc();

    // Confirm transaction
    const confirmation = await anchor
      .getProvider()
      .connection.confirmTransaction(txHash);
    console.log(
      `Transaction ${confirmation.value.err ? "failed" : "succeeded"}`
    );

    const fetchedPresaleAccount =
      await program.account.presaleAccount.fetch(presaleAccount);
    console.log(fetchedPresaleAccount);
    assert.ok(fetchedPresaleAccount);

    const fetchedUserAccount =
      await program.account.userAccount.fetch(userAccount);
    console.log(fetchedUserAccount);
    assert.ok(fetchedUserAccount);

    // Log the completion of the initialization
    console.log("Removing whitelist completed successfully");

    // Log the connection to the devnet
    console.log(`Connected to ${anchor.getProvider().connection.rpcEndpoint}`);
  });
});
