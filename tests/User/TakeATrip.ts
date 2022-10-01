import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SubeCrypto } from "../target/types/sube_crypto";
import { PublicKey} from '@solana/web3.js'

describe("sube-crypto", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SubeCrypto as Program<SubeCrypto>;

  it("Is initialized!", async () => {
    const [BusLine, _bump] = await PublicKey.findProgramAddress(
      [
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    const Account = await program.account.unsafehold.fetch(BusLine);
    const tx = await program.methods.takeATrip(6)
    .accounts(
      sube: BusLine,
      from: provider.wallet.publicKey,
      to: Account.authority,
      systemProgram: anchor.web3.SystemProgram.programId
    )
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
