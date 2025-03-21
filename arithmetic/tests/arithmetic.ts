import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Arithmetic } from "../target/types/arithmetic";

describe("arithmetic", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Arithmetic as Program<Arithmetic>;

  it("Is initialized!", async () => {
    const tx = await program.methods
        .initialize(new anchor.BN(777), new anchor.BN(888), "hello")
        .rpc();
    console.log("Your transaction signature", tx);
});
});
