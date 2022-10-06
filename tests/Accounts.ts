import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const BusLine = new anchor.web3.PublicKey(
    "44jmj26wqMZJjk2rxatUk6djb5orVTMEHnJNAc1WjWL3"
);
export const Card = new anchor.web3.PublicKey(
    "4kszrEHLChhzWJe43jHsGd66enDJTsVfjcwqasXXMD9B"
);