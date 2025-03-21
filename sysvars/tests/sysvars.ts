import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sysvars } from "../target/types/sysvars";

describe("sysvars", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sysvars as Program<Sysvars>;

  const StakeHistory_PublicKey = new anchor.web3.PublicKey(
    "SysvarStakeHistory1111111111111111111111111"
  );

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initialize(3)
      .accounts({
        stakeHistory: StakeHistory_PublicKey,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY, 
        instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY, 
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});