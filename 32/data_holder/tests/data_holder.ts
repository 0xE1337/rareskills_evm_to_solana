import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DataHolder } from "../target/types/data_holder";
describe("data-holder", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .DataHolder as Program<DataHolder>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [storage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    await program.methods
      .initialize()
      .accounts({ storage: storage })
      .rpc();

    let storageStruct = await program.account.storage.fetch(
      storage
    );

    console.log(
      "The value of x is: ",
      storageStruct.x.toString()
    );

    console.log("Storage account address: ", storage.toBase58());
  });
});