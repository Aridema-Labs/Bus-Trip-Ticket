import * as anchor from "@project-serum/anchor";
import { BusTripTicket } from "../../target/types/bus_trip_ticket";
import { PublicKey } from '@solana/web3.js'

describe("Register a Bus Line", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BusTripTicket as anchor.Program<BusTripTicket>

  it("Is initialized!", async () => {
    const [BusLine, _bump] = await PublicKey.findProgramAddress(
      [
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    const tx = await program.methods.initializeBusLine(
      new anchor.BN(1785714),
      new anchor.BN(2000000),
      new anchor.BN(2100000),
      new anchor.BN(2200000),
      new anchor.BN(2300000)
    )
    .accounts({
      bus: BusLine,
      signer: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
    const Account = await program.account.busAccount.fetch(BusLine);
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