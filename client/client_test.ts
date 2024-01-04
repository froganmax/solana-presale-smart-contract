const fs = require('fs');
const anchor = require('@coral-xyz/anchor');
const web3 = require('@solana/web3.js');
//import type { TokenPresale } from "../target/types/token_presale";
//import type { TokenPresale } from "../target/types/token_presale";


async function main() {
    const connection = new web3.Connection(web3.clusterApiUrl('devnet'), 'confirmed');
    const wallet = new anchor.Wallet(web3.Keypair.generate());
    const provider = new anchor.AnchorProvider(connection, wallet, { preflightCommitment: 'confirmed' });
    anchor.setProvider(provider);

    //const idl = JSON.parse(fs.readFileSync('../target/idl/token_presale.json', 'utf8'));
    const idl = JSON.parse(fs.readFileSync('../target/idl/test_program.json', 'utf8'));
    const programId = new anchor.web3.PublicKey('4XszoJS6Sb4FyRhqqaspyWhk5zzLNZo7ix4ADJCJVdKW');
    const program = new anchor.Program(idl, programId);
    console.log("program " + program);
    //await program.rpc.initialize();

    await program.rpc.create({
        accounts: {
            baseAccount: wallet.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: programId,
        },
        signers: [wallet],
    });

    /* Fetch the account and check the value of count */
    const account = await program.account.baseAccount.fetch(wallet.publicKey);
    console.log('Count 0: ', account.count.toString())
    // assert.ok(account.count.toString() == 0);
    // _baseAccount = baseAccount;


}

main().then(() => console.log('Success')).catch(err => console.error(err));

