import * as anchor from "@project-serum/anchor";
import { BusTripTicket } from "../../target/types/bus_trip_ticket";
import { PublicKey } from '@solana/web3.js'

describe("Register a Bus Line", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BusTripTicket as anchor.Program<BusTripTicket>

  it("Is initialized!", async () => {
    const [Card, _bump] = await PublicKey.findProgramAddress(
        [
          provider.wallet.publicKey.toBuffer(),
        ],
        program.programId
      )
    const tx = await program.methods.enableCard()
    .accounts({
      signer: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
    console.log("----------------------------------------------")
    console.log("Your transaction signature", tx);
    console.log("----------------------------------------------")
    console.log("Card: ", Card.toBase58())
    console.log("----------------------------------------------")
  });
});