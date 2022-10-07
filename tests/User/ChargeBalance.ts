import * as anchor from "@project-serum/anchor";
import { BusTripTicket } from "../../target/types/bus_trip_ticket";
import { Card } from "../Accounts";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("Sending balance", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BusTripTicket as anchor.Program<BusTripTicket>

  it("Balance sent", async () => {
    const tx = await program.methods.chargeBalance(
        new anchor.BN(2300000)
    )
    .accounts({
      from: provider.wallet.publicKey,
      to: Card,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
    let balance = await provider.connection.getBalance(Card);
    console.log("----------------------------------------------")
    console.log("Your transaction signature", tx);
    console.log("----------------------------------------------")
    console.log("Balance:", (balance / LAMPORTS_PER_SOL).toString())
    console.log("----------------------------------------------")
  });
});