import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Stablecoin } from "../target/types/stablecoin";
import { assert, expect } from "chai";

describe("stablecoin", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.stablecoin as Program<Stablecoin>;
  const wallet = program.provider.wallet!;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializeConfig()
      .accounts({
        authority: wallet.publicKey,
      })
      .signers([wallet.payer])
      .rpc();

    console.log("Your transaction signature", tx);

    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );
    const configAccount = await program.account.config.fetch(pda, "confirmed");

    console.log("etogoe", configAccount);
  });

  it("can update the conf", async () => {
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );
    const configAccount = await program.account.config.fetch(pda, "confirmed");

    console.log("etogoe2", configAccount);

    await program.methods
      .updateConfig(60, null, null)
      .accounts({
        config: pda,
      })
      .signers([wallet.payer])
      .rpc({ commitment: "confirmed" });

    const updatedConfigAccount = await program.account.config.fetch(
      pda,
      "confirmed"
    );

    console.log("updated conf", updatedConfigAccount);

    expect(updatedConfigAccount.minHealthFactor).to.not.equal(
      configAccount.minHealthFactor
    );
  });
});
