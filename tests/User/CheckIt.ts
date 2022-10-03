import * as anchor from "@project-serum/anchor";
import { BusTripTicket } from "../../target/types/bus_trip_ticket";
import { BusLine } from "../Accounts"

describe("Register a Bus Line", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BusTripTicket as anchor.Program<BusTripTicket>

  it("Is initialized!", async () => {
    const Account = await program.account.busAccount.fetch(BusLine);
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