import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CloseProgram } from "../target/types/close_program";
import { assert } from "chai";

describe("close_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CloseProgram as Program<CloseProgram>;

  it("Is initialized!", async () => {
   let [thePda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);
    await program.methods.initialize().accounts({thePda: thePda}).rpc();
    await program.methods.delete().accounts({thePda: thePda}).rpc();

    let account = await program.account.thePda.fetchNullable(thePda);
    console.log(account)
  });
});