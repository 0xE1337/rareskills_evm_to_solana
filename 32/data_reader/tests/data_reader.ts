import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DataReader } from "../target/types/data_reader";

describe("data-reader", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .DataReader as Program<DataReader>;

  it("Is initialized!", async () => {
    // CHANGE THIS TO THE ADDRESS OF THE PDA OF
    // DATA ACCOUNT HOLDER
    const otherStorageAddress =
      "8bxHxSAfw3CRFrog3DjFytW7u4KJSc7DdGjALEE5xUWf";

    const pub_key_other_storage = new anchor.web3.PublicKey(
      otherStorageAddress
    );

    const tx = await program.methods
      .readOtherData()
      .accounts({ otherData: pub_key_other_storage })
      .rpc();
  });
});