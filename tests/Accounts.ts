import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const BusLine = new anchor.web3.PublicKey(
    "4NT7yfA2qWUMZ5rWHruMU43UxPfE4G7cAytKr3VbD7qr"
);
export const Card = new anchor.web3.PublicKey(
    "6UmYQXFuknjBZXS5VNMZs3W2TAssgD2aTkkTXBz38z9"
);