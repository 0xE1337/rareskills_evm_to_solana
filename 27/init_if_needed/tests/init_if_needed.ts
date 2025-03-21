import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { InitIfNeeded } from "../target/types/init_if_needed";

describe("init_if_needed", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.InitIfNeeded as Program<InitIfNeeded>;

  it("Is initialized!", async () => {
    const [myPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync([], program.programId);
    await program.methods.increment().accounts({myPda: myPda}).rpc();
    await program.methods.increment().accounts({myPda: myPda}).rpc();
    await program.methods.increment().accounts({myPda: myPda}).rpc();

    let result = await program.account.myPda.fetch(myPda);
    console.log(`counter is ${result.counter}`);
  });
});