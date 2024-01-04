const fs = require('fs');
const anchor = require('@coral-xyz/anchor');
const web3 = require('@solana/web3.js');
const os = require('os');
const path = require('path');
//import type { TokenPresale } from "../target/types/token_presale";
//import type { TokenPresale } from "../target/types/token_presale";


async function main() {
    const connection = new web3.Connection(web3.clusterApiUrl('devnet'), 'confirmed');

    const walletPath = path.join(os.homedir(), '.config/solana/id.json');
    const secretKeyString = fs.readFileSync(walletPath, 'utf8');
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const keypair = web3.Keypair.fromSecretKey(secretKey);
    const wallet = new anchor.Wallet(keypair);

    console.log("wallet.publicKey " + wallet.publicKey);
    const balanceInLamports = await connection.getBalance(wallet.publicKey);
    const balanceInSOL = balanceInLamports / web3.LAMPORTS_PER_SOL;

    console.log(`Wallet Address: ${wallet.publicKey.toString()}`);
    console.log(`Balance: ${balanceInSOL} SOL`);

    const provider = new anchor.AnchorProvider(connection, wallet, { preflightCommitment: 'confirmed' });
    anchor.setProvider(provider);

    //const idl = JSON.parse(fs.readFileSync('../target/idl/token_presale.json', 'utf8'));
    const idl = JSON.parse(fs.readFileSync('../target/idl/test_program.json', 'utf8'));
    const programId = new anchor.web3.PublicKey('4XszoJS6Sb4FyRhqqaspyWhk5zzLNZo7ix4ADJCJVdKW');
    const program = new anchor.Program(idl, programId);

    console.log("program " + program);
    console.log("Listing all available RPC functions:");

    // Assuming 'program' is your initialized Anchor program
    const rpcFunctions = Object.keys(program.rpc);

    // Iterate and log each function name
    rpcFunctions.forEach(funcName => {
        console.log(funcName);
    });

    await program.rpc.create({
        accounts: {
            baseAccount: wallet.publicKey,
            user: wallet.publicKey,
            systemProgram: programId,
        },
        signers: [keypair],
    });

    // /* Fetch the account and check the value of count */
    // const account = await program.account.baseAccount.fetch(wallet.publicKey);
    // console.log('Count 0: ', account.count.toString())
    // assert.ok(account.count.toString() == 0);
    // _baseAccount = baseAccount;


}

main().then(() => console.log('Success')).catch(err => console.error(err));

