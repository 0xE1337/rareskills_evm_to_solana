import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sysvar } from "../target/types/sysvar";

describe("sysvar", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sysvar as Program<Sysvar>;

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
      })
      .rpc();

    console.log("Transaction hash:", tx);
  });
});