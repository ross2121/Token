import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Token } from "../target/types/token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";

// Metaplex Token Metadata Program ID
const METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

describe("Token Lottery Program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.token as Program<Token>;

  // Helper function to get PDAs
  const getTokenLotteryPDA = async () => {
    return await PublicKey.findProgramAddress(
      [Buffer.from("token_lottery")],
      program.programId
    );
  };

  const getCollectionMintPDA = async () => {
    return await PublicKey.findProgramAddress(
      [Buffer.from("collectionmint")],
      program.programId
    );
  };

  const getTicketMintPDA = async (ticketNum: number) => {
    return await PublicKey.findProgramAddress(
      [Buffer.from(ticketNum.toString())],
      program.programId
    );
  };

  const getMetadataPDA = async (mint: PublicKey) => {
    return await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      METADATA_PROGRAM_ID
    );
  };

  const getMasterEditionPDA = async (mint: PublicKey) => {
    return await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from("edition"),
      ],
      METADATA_PROGRAM_ID
    );
  };

  describe("Initialize Token Lottery", () => {
    it("should initialize the token lottery with correct parameters", async () => {
      const [tokenLotteryPDA] = await getTokenLotteryPDA();
      
      const startTime = new anchor.BN(0);
      const endTime = new anchor.BN(12313);
      const ticketPrice = new anchor.BN(232);

      const tx = await program.methods.initialize(
        startTime,
        endTime,
        ticketPrice
      ).accountsStrict({
        signer: wallet.publicKey,
        tokenLottery: tokenLotteryPDA,
        systemProgram: SystemProgram.programId,
      }).instruction();

      const blockhash = await provider.connection.getLatestBlockhash();
      const tx3 = new anchor.web3.Transaction({
        blockhash: blockhash.blockhash,
        feePayer: wallet.publicKey,
        lastValidBlockHeight: blockhash.lastValidBlockHeight
      }).add(tx);

      const signature = await anchor.web3.sendAndConfirmTransaction(
        provider.connection,
        tx3,
        [wallet.payer]
      );
      console.log("Token lottery initialized:", signature);

 
      const lotteryAccount = await program.account.tokenLottery.fetch(tokenLotteryPDA);
      expect(lotteryAccount.startTime.toString()).to.equal(startTime.toString());
      expect(lotteryAccount.endTime.toString()).to.equal(endTime.toString());
      expect(lotteryAccount.ticketPrice.toString()).to.equal(ticketPrice.toString());
      expect(lotteryAccount.authority.toString()).to.equal(wallet.publicKey.toString());
    });
  });

  describe("Initialize Lottery Collection", () => {
    it("should initialize the lottery collection with metadata", async () => {
      const [collectionMintPDA] = await getCollectionMintPDA();
      const [metadataPDA] = await getMetadataPDA(collectionMintPDA);
      const [masterEditionPDA] = await getMasterEditionPDA(collectionMintPDA);
      
      const collectionToken = await anchor.utils.token.associatedAddress({
        mint: collectionMintPDA,
        owner: wallet.publicKey
      });

      const initLottery = await program.methods.initializeLottery().accountsStrict({
        signer: wallet.publicKey,
        collectionMint: collectionMintPDA,
        collectionToken: collectionToken,
        metadata: metadataPDA,
        masterEdition: masterEditionPDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenMetadataProgram: METADATA_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      }).instruction();

      const blockhash = await provider.connection.getLatestBlockhash();
      const tx = new anchor.web3.Transaction({
        blockhash: blockhash.blockhash,
        feePayer: wallet.publicKey,
        lastValidBlockHeight: blockhash.lastValidBlockHeight
      }).add(initLottery);

      const signature = await anchor.web3.sendAndConfirmTransaction(
        provider.connection,
        tx,
        [wallet.payer]
      );
      console.log("Lottery collection initialized:", signature);
      const tokenAccount = await provider.connection.getTokenAccountBalance(collectionToken);
      expect(tokenAccount.value.amount).to.equal("1");
    });
  });

  describe("Buy Lottery Ticket", () => {
    it("should allow buying a lottery ticket and mint NFT", async () => {
      const [tokenLotteryPDA] = await getTokenLotteryPDA();
      const [collectionMintPDA] = await getCollectionMintPDA();
  
      const lotteryAccount = await program.account.tokenLottery.fetch(tokenLotteryPDA);
      const ticketNum = lotteryAccount.totalTickets.toNumber();
      
      const [ticketMintPDA] = await getTicketMintPDA(ticketNum);
      const [ticketMetadataPDA] = await getMetadataPDA(ticketMintPDA);
      const [ticketMasterEditionPDA] = await getMasterEditionPDA(ticketMintPDA);
      const [collectionMetadataPDA] = await getMetadataPDA(collectionMintPDA);
      const [collectionMasterEditionPDA] = await getMasterEditionPDA(collectionMintPDA);
      
      const destination = await anchor.utils.token.associatedAddress({
        mint: ticketMintPDA,
        owner: wallet.publicKey
      });

      const buyTicket = await program.methods.buyTicket().accountsStrict({
        payer: wallet.publicKey,
        tokenLottery: tokenLotteryPDA,
        ticketMint: ticketMintPDA,
        ticketMetadata: ticketMetadataPDA,
        ticketMasterEdition: ticketMasterEditionPDA,
        destination: destination,
        collectionMetadata: collectionMetadataPDA,
        collectionMasterEdition: collectionMasterEditionPDA,
        collectionMint: collectionMintPDA,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        tokenMetadataProgram: METADATA_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }).instruction();

      const blockhash = await provider.connection.getLatestBlockhash();
      const tx = new anchor.web3.Transaction({
        blockhash: blockhash.blockhash,
        feePayer: wallet.publicKey,
        lastValidBlockHeight: blockhash.lastValidBlockHeight
      }).add(buyTicket);

      const signature = await anchor.web3.sendAndConfirmTransaction(
        provider.connection,
        tx,
        [wallet.payer]
      );
      console.log("Ticket bought:", signature);
      const ticketAccount = await provider.connection.getTokenAccountBalance(destination);
      expect(ticketAccount.value.amount).to.equal("1");

      const updatedLotteryAccount = await program.account.tokenLottery.fetch(tokenLotteryPDA);
      expect(updatedLotteryAccount.totalTickets.toNumber()).to.equal(ticketNum + 1);
    });
  });
});  
