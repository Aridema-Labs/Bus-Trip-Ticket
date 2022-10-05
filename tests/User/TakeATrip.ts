import * as anchor from "@project-serum/anchor";
import { BusTripTicket } from "../../target/types/bus_trip_ticket";
import { BusLine, Card } from "../Accounts"
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("Take a Trip", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BusTripTicket as anchor.Program<BusTripTicket>

  it("Buying Ticket", async () => {
    let balance = await provider.connection.getBalance(provider.wallet.publicKey);
    const Account = await program.account.busAccount.fetch(BusLine);
    const tx = await program.methods.takeATrip(0)
    .accounts({
      bus: BusLine,
      from: Card,
      to: Account.authority,
      systemProgram: anchor.web3.SystemProgram.programId,
  })
    .rpc();
    console.log("----------------------------------------------")
    console.log("Your transaction signature", tx);
    console.log("----------------------------------------------")
    console.log("Bus: ", BusLine.toBase58())
    console.log("----------------------------------------------")
    console.log("Your Balance: ", (balance / LAMPORTS_PER_SOL).toString())
    console.log("----------------------------------------------")
  });
});
