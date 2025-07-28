import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferHook } from "../target/types/transfer_hook"
import {
  ExtensionType,
  TOKEN_2022_PROGRAM_ID,
  createInitializeMintInstruction,
  createInitializeTransferHookInstruction,
  createTransferCheckedInstruction,
  createTransferCheckedWithTransferHookInstruction,
  getMint,
  getMintLen,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
  transferChecked,
  transferCheckedWithTransferHook,
} from "@solana/spl-token";
import { assert } from "chai";
import { PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("transfer-hook", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.TransferHook as Program<TransferHook>;

  const payer = provider.wallet;
  const senderUser = payer.payer; // Payer is the sender
  const user1 = anchor.web3.Keypair.generate(); // whitelisted
  const user2 = anchor.web3.Keypair.generate(); // not whitelisted

  let mint: anchor.web3.Keypair;
  let sourceATA: any;
  let whitelistPDA: anchor.web3.PublicKey;

  before(async () => {
    mint = anchor.web3.Keypair.generate();
    const extensions = [ExtensionType.TransferHook];
    const mintLen = getMintLen(extensions);
    console.log(program.programId.toBase58());
    const lamports =
      await provider.connection.getMinimumBalanceForRentExemption(mintLen);
    const transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: mint.publicKey,
        space: mintLen,
        lamports,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeTransferHookInstruction(
        mint.publicKey,
        payer.publicKey,
        program.programId,
        TOKEN_2022_PROGRAM_ID
      ),
      createInitializeMintInstruction(
        mint.publicKey,
        8,
        payer.publicKey,
        null,
        TOKEN_2022_PROGRAM_ID
      )
    );
    await provider.sendAndConfirm(transaction, [mint]);

    sourceATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint.publicKey,
      payer.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    await mintTo(
      provider.connection,
      payer.payer,
      mint.publicKey,
      sourceATA.address,
      payer.publicKey,
      100_000_000_000,
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const [extraMetaListPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("extra-account-metas"), mint.publicKey.toBuffer()],
      program.programId
    );
    const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      program.programId
    );
    await program.methods
      .initializeExtraAccountMetaList()
      .accounts({
        payer: payer.publicKey,
        mint: mint.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
  });

  it("✅ should transfer to whitelisted user", async () => {

    const [extraAccountMetaListPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("extra-account-metas"), mint.publicKey.toBuffer()],
      program.programId
    );
    const [counterPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter")],
      program.programId
    );
    const destinationATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer.payer,
      mint.publicKey,
      user1.publicKey,
      true,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const ix = await createTransferCheckedWithTransferHookInstruction(
      provider.connection,
      sourceATA.address,
      mint.publicKey,
      destinationATA.address,
      payer.publicKey,
      BigInt(100_000_000), // 1 token with 8 decimals
      8,
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID, 
    )
    ix.keys.push(
      { pubkey: extraAccountMetaListPDA, isSigner: false, isWritable: true },
      { pubkey: counterPDA, isSigner: false, isWritable: true },
    )
    const tx = new Transaction().add(ix);
    const sig = await sendAndConfirmTransaction(
      provider.connection,
      tx,
      [payer.payer],
      { commitment: "confirmed" }
    );
    console.log("Transfer transaction signature:", sig);

    const account = await provider.connection.getTokenAccountBalance(destinationATA.address);
    assert.equal(account.value.uiAmount, 1, "User1 should receive tokens");
    // check count
    const counter = await program.account.counterAccount.fetch(
      counterPDA
    );
    assert.equal(counter.counter, 1, "Counter should be incremented");
  });
});
