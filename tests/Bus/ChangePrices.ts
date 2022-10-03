import * as anchor from "@project-serum/anchor";
import { BusTripTicket } from "../../target/types/bus_trip_ticket";
import { BusLine } from "../Accounts"

describe("Take a Trip", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BusTripTicket as anchor.Program<BusTripTicket>

  it("Buying Ticket", async () => {
    const Account = await program.account.busAccount.fetch(BusLine);
    const tx = await program.methods.changePrices(
      new anchor.BN(1785714),
      new anchor.BN(2000000),
      new anchor.BN(2100000),
      new anchor.BN(2200000),
      new anchor.BN(2300000)
    )
    .accounts({
      bus: BusLine,
      signer: provider.publicKey,
  })
    .rpc();
    console.log("----------------------------------------------")
    console.log("Your transaction signature", tx);
    console.log("----------------------------------------------")
    console.log("PDA: ", BusLine.toBase58())
    console.log("----------------------------------------------")
    console.log("Authority:", Account.authority.toBase58())
    console.log("----------------------------------------------")
    console.log("Bump:", Account.bumpOriginal.toString())
    console.log("----------------------------------------------")
    console.log("0 to 3 km ticket price:", Account.toThreeKm.toString())
    console.log("----------------------------------------------")
    console.log("3 to 6 km ticket price:", Account.toSixKm.toString())
    console.log("----------------------------------------------")
    console.log("6 to 12 km ticket price:", Account.toTwelveKm.toString())
    console.log("----------------------------------------------")
    console.log("12 to 27 km ticket price:", Account.toTwentySevenKm.toString())
    console.log("----------------------------------------------")
    console.log("more 27 km ticket price:", Account.moreTwentySevenKm.toString())
    console.log("----------------------------------------------")
  });
});