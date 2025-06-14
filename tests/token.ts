import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token } from "../target/types/token";

describe("token", () => {
  // Configure the client to use the local cluster.
  const provider=anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());
 const wallet=provider.wallet as anchor.Wallet;

  const program = anchor.workspace.token as Program<Token>;

  async function buyticket() {
    const blockhash=await provider.connection.getLatestBlockhash();
    const initlottery=await program.methods.initializeLottery().accounts({
       
    }).instruction(); 
    const initlotterty=new anchor.web3.Transaction(
      {
        blockhash:blockhash.blockhash,
        feePayer:provider.wallet.publicKey,
        lastValidBlockHeight:blockhash.lastValidBlockHeight  
      }  
    ).add(initlottery);
    const initsign=await anchor.web3.sendAndConfirmTransaction(provider.connection,initlotterty,[wallet.payer]);
    console.log("dasd",wallet.payer.publicKey.toBase58())
    console.log("dasd",initsign);
  }

  it("Is initialized!", async () => {
    // Add your test here.
    const tx =await program.methods.initialize(
      new anchor.BN(0),
      new anchor.BN(12313),
      new anchor.BN(232)
    ).instruction();
    const blockhash=await provider.connection.getLatestBlockhash();
    const tx3=new anchor.web3.Transaction(
      {
        blockhash:blockhash.blockhash,
        feePayer:provider.wallet.publicKey,
        lastValidBlockHeight:blockhash.lastValidBlockHeight  
      }  
    ).add(tx);
    const sign=await anchor.web3.sendAndConfirmTransaction(provider.connection,tx3,[wallet.payer]);
    const initlottery=await program.methods.initializeLottery().accounts({
      
    }).instruction();
    const initlotterty=new anchor.web3.Transaction(
      {
        blockhash:blockhash.blockhash,
        feePayer:provider.wallet.publicKey,
        lastValidBlockHeight:blockhash.lastValidBlockHeight  
      }  
    ).add(initlottery);
    const initsign=await anchor.web3.sendAndConfirmTransaction(provider.connection,initlotterty,[wallet.payer]);
    console.log("dasd",wallet.payer.publicKey.toBase58())
    console.log("dasd",initsign);
    console.log("Your transaction signature", sign);
    await buyticket();
  });
});  
