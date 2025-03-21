import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorFunctionTutorial } from "../target/types/anchor_function_tutorial";

describe("anchor-function-tutorial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorFunctionTutorial as Program<AnchorFunctionTutorial>;

  it("Should call function_a", async () => {
    const signer = anchor.web3.Keypair.generate();
    const anotherSigner = anchor.web3.Keypair.generate();
    
    const tx = await program.methods
      .functionA()
      .accounts({
        signer: signer.publicKey,
        anotherSigner: anotherSigner.publicKey,
      })
      .signers([signer, anotherSigner])
      .rpc();
    
    console.log("Your transaction signature", tx);
  });

  it("Should call function_b", async () => {
    const tx = await program.methods
      .functionB(new anchor.BN(42))
      .rpc();
    
    console.log("Your transaction signature", tx);
  });
});
