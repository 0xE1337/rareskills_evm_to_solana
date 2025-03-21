import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountTypes } from "../target/types/account_types";

describe("account_types", () => {  
	anchor.setProvider(anchor.AnchorProvider.env()); 
 
	const program = anchor.workspace.AccountTypes as Program<AccountTypes>;  

	it("Wrong owner with Account", async () => {    
		await program.methods.hello().rpc();  
	});
});