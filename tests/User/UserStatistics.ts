import * as anchor from "@project-serum/anchor";
import { BusTripTicket } from "../../target/types/bus_trip_ticket";
import { PublicKey, LAMPORTS_PER_SOL } from '@solana/web3.js'

describe("User statistics", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BusTripTicket as anchor.Program<BusTripTicket>

  it("Solana Blockchain APIs", async () => {
    const [User, _bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("Enable"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    let balance = await provider.connection.getBalance(User);
    const Account = await program.account.enableUserCard.fetch(User);
    console.log("----------------------------------------------")
    console.log("PDA: ", User.toBase58())
    console.log("----------------------------------------------")
    console.log("Authority:", Account.authority.toBase58())
    console.log("----------------------------------------------")
    console.log("Bump:", Account.bumpOriginal.toString())
    console.log("----------------------------------------------")
    console.log("Balance:", ((balance - 1176240) / LAMPORTS_PER_SOL).toString())
    console.log("----------------------------------------------")
  });
});