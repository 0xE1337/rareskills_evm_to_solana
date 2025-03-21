import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ExampleMap } from "../target/types/example_map";

describe("example_map", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ExampleMap as Program<ExampleMap>;

  it("Initialize and set value", async () => {
    const key1 = new anchor.BN(42);
    const key2 = new anchor.BN(43);
    const value = new anchor.BN(1337);

    const seeds = [key1.toArrayLike(Buffer, "le", 8), key2.toArrayLike(Buffer, "le", 8)];
    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId,
    )[0];

    await program.methods.initialize(key1, key2).accounts({val: valueAccount}).rpc();
    await program.methods.set(key1, key2, value).accounts({val: valueAccount}).rpc();

    let result = await program.account.val.fetch(valueAccount);
    console.log(`the value ${result.value} was stored in ${valueAccount.toBase58()}`);

  });
});