import * as anchor from "@project-serum/anchor";
import { BusTripTicket } from "../../target/types/bus_trip_ticket";
import { Card } from "../Accounts"

describe("Sending balance", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BusTripTicket as anchor.Program<BusTripTicket>

  it("Balance sent", async () => {
    const tx = await program.methods.chargeBalance(
        new anchor.BN(50000)
    )
    .accounts({
      from: provider.wallet.publicKey,
      to: Card,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
    console.log("----------------------------------------------")
    console.log("Your transaction signature", tx);
    console.log("----------------------------------------------")
  });
});