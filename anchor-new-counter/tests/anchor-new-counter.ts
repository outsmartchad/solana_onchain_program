import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect, assert } from "chai";
import { AnchorNewCounter } from "../target/types/anchor_new_counter";

describe("anchor-new-counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider)

  const program = anchor.workspace.anchorNewCounter as Program<AnchorNewCounter>;

  const counter = anchor.web3.Keypair.generate();
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({counter: counter.publicKey})
      .signers([counter])
      .rpc()

    const counter_account = await program.account.counter.fetch(counter.publicKey);
    console.log(counter_account.count)
    expect(counter_account.count.toNumber()).to.equal(0);
  });

  it("counter incremented!", async () => {
    // Add your test here.
    const tx1 = await program.methods
      .increment()
      .accounts({counter: counter.publicKey})
      .rpc();

    const counter_account = await program.account.counter.fetch(counter.publicKey);
    console.log(counter_account.count)
    expect(counter_account.count.toNumber()).to.equal(1);
  });


  it("counter decremented!", async () => {
    // Add your test here.
    const tx = await program.methods
      .decrement()
      .accounts({counter: counter.publicKey})
      .rpc();

    const counter_account = await program.account.counter.fetch(counter.publicKey);
    console.log(counter_account.count)
    expect(counter_account.count.toNumber()).to.equal(0);
  });

  it("counter decremented but previous count is 0", async () => {
    // Add your test here.
    const counter_account = await program.account.counter.fetch(counter.publicKey);
    console.log(counter_account.count)
    try{
      await program.methods
        .decrement()
        .accounts({counter: counter.publicKey})
        .rpc();
      //assert.fail("Expected counter at zero but someone try to decrement");
    }catch(error){
      //console.log(error)
      expect(error.error.errorCode.code).to.equal("CounterAtZero")
    }
  });

});
