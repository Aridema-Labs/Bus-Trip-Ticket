import * as anchor from "@project-serum/anchor";
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
export const wallet = provider.wallet as anchor.Wallet;

export const BusLine = new anchor.web3.PublicKey(
    "69zuTqeMAXcePnVhVRZzcFPKFKws24zmx8xVUPrZZFFM"
);