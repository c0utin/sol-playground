import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import { Votingdapp } from "../target/types/votingdapp";
import { BankrunProvider, startAnchor } from "anchor-bankrun";

const IDL = require("../target/idl/votingdapp.json");
const VOTING_ADDRESS = new PublicKey(
  "coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF",
);

describe("votingdapp", () => {
  it("Initialize Poll", async () => {
    const context = await startAnchor(
      "",
      [{ name: "votingdapp", programId: VOTING_ADDRESS }],
      [],
    );
    const provider = new BankrunProvider(context);

    const votingProgram = new Program<Votingdapp>(IDL, provider);

    await votingProgram.methods
      .initializePoll(
        new anchor.BN(1),
        "What is your favorite rapper?",
        new anchor.BN(0),
        new anchor.BN(1839448991),
      )
      .rpc();
  });
});
