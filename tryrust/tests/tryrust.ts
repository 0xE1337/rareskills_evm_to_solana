import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tryrust } from "../target/types/tryrust";

describe("tryrust", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Tryrust as Program<Tryrust>;

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize("Alice", new anchor.BN(20)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Age checker", async () => {
    const tx = await program.methods.ageChecker(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  });

});
