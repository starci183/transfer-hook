import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
    ExtensionType,
    TOKEN_2022_PROGRAM_ID,
    createInitializeMintInstruction,
    createInitializeTransferHookInstruction,
    getMintLen,
    getOrCreateAssociatedTokenAccount,
    mintTo,
    Account
} from "@solana/spl-token";
import { TransferHookDispatcher } from "../target/types/transfer_hook_dispatcher";
import { Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { TransferHook } from "../target/types/transfer_hook";

describe("transfer-hook-dispatcher", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.TransferHookDispatcher as Program<TransferHookDispatcher>;
    const hookDispatcherProgram = anchor.workspace.TransferHook as Program<TransferHook>;
    const payer = provider.wallet;
    const mint = Keypair.generate();
    const admin = Keypair.generate();
    const from = Keypair.generate();
    const to = Keypair.generate();
    let sourceAta: Account;
    console.log(`Payer: ${payer.payer.publicKey.toBase58()}`);
    console.log(`Mint: ${mint.publicKey.toBase58()}`);
    console.log(`Admin: ${admin.publicKey.toBase58()}`);
    console.log(`From: ${from.publicKey.toBase58()}`);
    console.log(`To: ${to.publicKey.toBase58()}`);
    before(async () => {
        // create mint
        const extensions = [ExtensionType.TransferHook];
        const mintLen = getMintLen(extensions);
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
                admin.publicKey,
                program.programId,
                TOKEN_2022_PROGRAM_ID
            ),
            createInitializeMintInstruction(
                mint.publicKey,
                8,
                admin.publicKey,
                null,
                TOKEN_2022_PROGRAM_ID
            )
        );
        await provider.sendAndConfirm(
            transaction,
            [mint]
        );

        // create source ATA
        sourceAta = await getOrCreateAssociatedTokenAccount(
            provider.connection,
            payer.payer,
            mint.publicKey,
            from.publicKey,
            false,
            undefined,
            undefined,
            TOKEN_2022_PROGRAM_ID
        );
        await mintTo(
            provider.connection,
            payer.payer,
            mint.publicKey,
            sourceAta.address,
            admin.publicKey,
            100_000_000_000,
            [
                admin
            ],
            undefined,
            TOKEN_2022_PROGRAM_ID
        );
    });

    it("should initialize global dispatcher config", async () => {
        // console.log(payer.publicKey.toBase58());
        // console.log(payer.payer.publicKey.toBase58());
        await program.methods
          .initializeGlobalDispatcherConfig(
            admin.publicKey
          )
          .accounts({
            payer: payer.publicKey,
          })
          .signers([payer.payer])
          .rpc();

        const [globalDispatcherConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("global-dispatcher-config")],
            program.programId
        );
        const dispatcherAccount = await program.account.globalDispatcherConfigAccount.fetch(
            globalDispatcherConfigPDA
        );
        assert.equal(
            dispatcherAccount.authority.toBase58(),
            admin.publicKey.toBase58(),
        );
    });

    it("should add allowed hook program", async () => {
        const [globalDispatcherConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("global-dispatcher-config")],
            program.programId
        );
        await program.methods
            .addAllowedHookProgram(
                hookDispatcherProgram.programId
            )
            .accounts({
                globalDispatcherConfigAccount: globalDispatcherConfigPDA,
                authority: admin.publicKey,
            })
            .signers([admin])
            .rpc();
    });

    it("should initialize extra account meta list", async () => {
        await program.methods
            .initializeExtraAccountMetaList()
            .accounts({
                mint: mint.publicKey,
                payer: payer.publicKey,
                tokenProgram: TOKEN_2022_PROGRAM_ID,
            })
            .signers([payer.payer])
            .rpc();
        const [dispatcherPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("dispatcher"), mint.publicKey.toBuffer()],
            program.programId
        );

        assert.isTrue(
            await program.account.dispatcherAccount.fetchNullable(dispatcherPDA) !== null,
            "Dispatcher account should be initialized"
        );
    });

    it("should add hook program to extra account meta list", async () => {
        const [globalDispatcherConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("global-dispatcher-config")],
            program.programId
        );
        const [dispatcherPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("dispatcher"), mint.publicKey.toBuffer()],
            program.programId
        );

        await program.methods
            .registerHookProgram(
                hookDispatcherProgram.programId
            )
            .accounts({
                dispatcherAccount: dispatcherPDA,
                globalDispatcherConfigAccount: globalDispatcherConfigPDA,
            })
            .signers([payer.payer])
            .rpc();

        const dispatcherAccount = await program.account.dispatcherAccount.fetch(
            dispatcherPDA
        );

        assert.equal(
            dispatcherAccount.hookPrograms[0].toBase58(),
            hookDispatcherProgram.programId.toBase58()
        );
    });
});