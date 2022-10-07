import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const BusLine = new anchor.web3.PublicKey(
    "AiPzHFkW4ysfvrNJ68w8NPFGmh6NJAxw2RnBY5PxGp8"
);
export const Card = new anchor.web3.PublicKey(
    "G4wVoLwEQiAXPNit2RPVoWisUQcJPFWvJcAHdXCFU3wN"
);