import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OptionsFutures } from "../target/types/options_futures";
import * as fs from "fs";
import { assert } from "chai";

describe("options-futures", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OptionsFutures as Program<OptionsFutures>;
  console.log("Program ID", program.programId.toBase58());
  console.log("Signer", program.provider.publicKey.toBase58());

  const secretKey = JSON.parse(
    fs.readFileSync("/home/hackerboy/.config/solana/id.json", "utf-8")
  );

  const signer1 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(secretKey));
  const signer2 = anchor.web3.Keypair.generate();
  const myAccount = anchor.web3.Keypair.generate();

  const sendLamportsTxn = new anchor.web3.Transaction();
  sendLamportsTxn.add(
    anchor.web3.SystemProgram.transfer({
      fromPubkey: program.provider.publicKey,
      toPubkey: signer2.publicKey,
      lamports: 10000000000,
    })
  );

  it("Is initialized!", async () => {
    const sendLamportsTxnId = await anchor.web3.sendAndConfirmTransaction(
      anchor.getProvider().connection,
      sendLamportsTxn,
      [signer1]
    );
    console.log("sendLamportsTxnId", sendLamportsTxnId);

    const tx = await program.methods
      .initialize({})
      .accounts({
        myAccount: myAccount.publicKey,
        signer1: signer1.publicKey,
        signer2: signer2.publicKey,
      })
      .signers([myAccount, signer2])
      .rpc();
    console.log("Your transaction signature", tx);
  });
  it("Is multisig initialized!", async () => {
    const tx = await program.methods
      .initializeMultisig({
        threshold: new anchor.BN(2),
        signers: [signer1.publicKey, signer2.publicKey, myAccount.publicKey],
      })
      .accounts({
        payer: program.provider.publicKey,
        signer1: myAccount.publicKey,
        signer2: signer2.publicKey,
      })
      .signers([myAccount, signer2])
      .rpc();
    console.log("Your transaction signature", tx);
  });
  it("Is proposed", async () => {
    try {
      const underlyingAssest = anchor.web3.Keypair.generate();
      const tx = await program.methods
        .proposeTrade({
          underlyingAsset: underlyingAssest.publicKey,
          strikePrice: new anchor.BN(100),
          expirationUnixTimestamp: new anchor.BN(1000000000),
          quantity: new anchor.BN(100),
          premiumOrPrice: new anchor.BN(100),
          tradeType: { call: {} },
        })
        .accounts({
          proposer: program.provider.publicKey,
          signer1: myAccount.publicKey,
          signer2: signer2.publicKey,
        })
        .signers([myAccount, signer2])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("Error", error);
    }
  });
  it("Updates", async () => {
    const tx = await program.methods
      .update({ data: new anchor.BN(45) })
      .accounts({
        myAccount: myAccount.publicKey,
        authority: signer2.publicKey,
      })
      .signers([signer2])
      .rpc();
    console.log("Updates", tx);
    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    assert.ok(account.data.eq(new anchor.BN(45)));
  });
});
