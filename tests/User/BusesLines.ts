import * as anchor from "@project-serum/anchor";
import { BusTripTicket } from "../../target/types/bus_trip_ticket";
import { BusLine } from "../Accounts"
const Id = new anchor.web3.PublicKey(
    "3SW4hFCYq1SP9U2Qq8BYFKFSpzTvsxrbHmXqKLKQ4jdw"
);

describe("Buses Lines", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BusTripTicket as anchor.Program<BusTripTicket>

  it("Is initialized!", async () => {
    const line = await program.provider.connection.getProgramAccounts(Id)
    const Account = await program.account.busAccount.fetch(BusLine);
    console.log("----------------------------------------------")
    console.log("Buses Lines: ", line)
    console.log("----------------------------------------------")
  });
});