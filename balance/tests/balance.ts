import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Balance } from "../target/types/balance";

describe("balance", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Balance as Program<Balance>;
  let pubkey = new anchor.web3.PublicKey("5NhLjdFKocoRMqic9sqAe5TxLagJCoCBunzg51ioMYot");

  it("Tests the balance", async () => {
    const tx = await program.methods.readBalance().accounts({ acct: pubkey }).rpc();
  });
});