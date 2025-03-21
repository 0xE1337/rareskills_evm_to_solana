import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sender } from "../target/types/sender";

describe("sender", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sender as Program<Sender>;

  it("Is called by the owner", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        signerAccount: program.provider.publicKey,
      })
      .rpc();

    console.log("Transaction hash:", tx);
  });
});