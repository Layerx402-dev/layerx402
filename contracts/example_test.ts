import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("Layerx402 Smart Contracts", () => {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;

  describe("Payment Escrow", () => {
    let escrowAccount: PublicKey;
    let recipient: Keypair;
    const amount = new anchor.BN(1_000_000_000); // 1 SOL

    before(async () => {
      recipient = Keypair.generate();
    });

    it("Initializes an escrow", async () => {
      const program = anchor.workspace.PaymentEscrow as Program;

      const expiryTimestamp = Math.floor(Date.now() / 1000) + 3600; // 1 hour

      [escrowAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from("escrow"), payer.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .initializeEscrow(amount, recipient.publicKey, new anchor.BN(expiryTimestamp))
        .accounts({
          escrow: escrowAccount,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const escrowData = await program.account.escrow.fetch(escrowAccount);
      assert.ok(escrowData.payer.equals(payer.publicKey));
      assert.ok(escrowData.recipient.equals(recipient.publicKey));
      assert.ok(escrowData.amount.eq(amount));
    });

    it("Funds the escrow", async () => {
      const program = anchor.workspace.PaymentEscrow as Program;

      const balanceBefore = await provider.connection.getBalance(escrowAccount);

      await program.methods
        .fundEscrow()
        .accounts({
          escrow: escrowAccount,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const balanceAfter = await provider.connection.getBalance(escrowAccount);
      assert.ok(balanceAfter > balanceBefore);
    });

    it("Releases the escrow with payment proof", async () => {
      const program = anchor.workspace.PaymentEscrow as Program;

      const recipientBalanceBefore = await provider.connection.getBalance(
        recipient.publicKey
      );

      await program.methods
        .releaseEscrow("BASE64_PAYMENT_PROOF_EXAMPLE")
        .accounts({
          escrow: escrowAccount,
          recipient: recipient.publicKey,
          authority: payer.publicKey,
        })
        .rpc();

      const recipientBalanceAfter = await provider.connection.getBalance(
        recipient.publicKey
      );

      assert.ok(recipientBalanceAfter > recipientBalanceBefore);

      const escrowData = await program.account.escrow.fetch(escrowAccount);
      assert.equal(escrowData.status.released, true);
    });
  });

  describe("Payment Verification", () => {
    let verifierAccount: PublicKey;
    let paymentAccount: PublicKey;
    const paymentProof = "TEST_PAYMENT_PROOF_" + Date.now();

    it("Initializes the verifier", async () => {
      const program = anchor.workspace.PaymentVerification as Program;

      [verifierAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from("verifier")],
        program.programId
      );

      await program.methods
        .initializeVerifier()
        .accounts({
          verifier: verifierAccount,
          authority: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const verifierData = await program.account.verifier.fetch(verifierAccount);
      assert.ok(verifierData.authority.equals(payer.publicKey));
      assert.ok(verifierData.totalVerifications.eq(new anchor.BN(0)));
    });

    it("Verifies a payment", async () => {
      const program = anchor.workspace.PaymentVerification as Program;

      const recipient = Keypair.generate().publicKey;
      const amount = new anchor.BN(500_000_000); // 0.5 SOL

      [paymentAccount] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("payment"),
          payer.publicKey.toBuffer(),
          Buffer.from(paymentProof),
        ],
        program.programId
      );

      await program.methods
        .verifyPayment(
          paymentProof,
          amount,
          recipient,
          "solana",
          "5wHu7J9VqYZN8xN9xN9xN9xN9xN9xN9xN9xN9xN9xN9x"
        )
        .accounts({
          payment: paymentAccount,
          verifier: verifierAccount,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const paymentData = await program.account.payment.fetch(paymentAccount);
      assert.ok(paymentData.amount.eq(amount));
      assert.ok(paymentData.recipient.equals(recipient));
      assert.equal(paymentData.network, "solana");
      assert.equal(paymentData.status.verified, true);

      const verifierData = await program.account.verifier.fetch(verifierAccount);
      assert.ok(verifierData.totalVerifications.eq(new anchor.BN(1)));
    });

    it("Settles a payment", async () => {
      const program = anchor.workspace.PaymentVerification as Program;

      await program.methods
        .settlePayment()
        .accounts({
          payment: paymentAccount,
          verifier: verifierAccount,
          authority: payer.publicKey,
        })
        .rpc();

      const paymentData = await program.account.payment.fetch(paymentAccount);
      assert.equal(paymentData.status.settled, true);
      assert.ok(paymentData.settledAt !== null);
    });
  });

  describe("Payment Settlement", () => {
    let poolAccount: PublicKey;
    let settlementAccount: PublicKey;
    const paymentId = "PAYMENT_" + Date.now();

    it("Initializes the settlement pool", async () => {
      const program = anchor.workspace.PaymentSettlement as Program;

      [poolAccount] = PublicKey.findProgramAddressSync(
        [Buffer.from("pool")],
        program.programId
      );

      const feePercentage = 100; // 1%

      await program.methods
        .initializePool(feePercentage)
        .accounts({
          pool: poolAccount,
          authority: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const poolData = await program.account.settlementPool.fetch(poolAccount);
      assert.ok(poolData.authority.equals(payer.publicKey));
      assert.equal(poolData.feePercentage, feePercentage);
      assert.ok(poolData.totalSettled.eq(new anchor.BN(0)));
    });

    it("Creates a settlement", async () => {
      const program = anchor.workspace.PaymentSettlement as Program;

      const recipient = Keypair.generate().publicKey;
      const amount = new anchor.BN(1_000_000_000); // 1 SOL

      [settlementAccount] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("settlement"),
          payer.publicKey.toBuffer(),
          Buffer.from(paymentId),
        ],
        program.programId
      );

      await program.methods
        .createSettlement(amount, recipient, paymentId)
        .accounts({
          settlement: settlementAccount,
          pool: poolAccount,
          payer: payer.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const settlementData = await program.account.settlement.fetch(
        settlementAccount
      );
      assert.ok(settlementData.grossAmount.eq(amount));
      assert.equal(settlementData.paymentId, paymentId);
      assert.equal(settlementData.status.pending, true);

      // Check fee calculation (1%)
      const expectedFee = amount.muln(1).divn(100);
      assert.ok(settlementData.feeAmount.eq(expectedFee));
    });

    it("Processes a settlement", async () => {
      const program = anchor.workspace.PaymentSettlement as Program;

      const settlementData = await program.account.settlement.fetch(
        settlementAccount
      );
      const recipient = settlementData.recipient;

      const recipientBalanceBefore = await provider.connection.getBalance(
        recipient
      );

      await program.methods
        .processSettlement()
        .accounts({
          settlement: settlementAccount,
          pool: poolAccount,
          payer: payer.publicKey,
          recipient: recipient,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const recipientBalanceAfter = await provider.connection.getBalance(
        recipient
      );

      assert.ok(recipientBalanceAfter > recipientBalanceBefore);

      const updatedSettlement = await program.account.settlement.fetch(
        settlementAccount
      );
      assert.equal(updatedSettlement.status.completed, true);

      const poolData = await program.account.settlementPool.fetch(poolAccount);
      assert.ok(poolData.totalSettled.gt(new anchor.BN(0)));
      assert.ok(poolData.totalFeesCollected.gt(new anchor.BN(0)));
    });

    it("Gets pool statistics", async () => {
      const program = anchor.workspace.PaymentSettlement as Program;

      await program.methods
        .getPoolStats()
        .accounts({
          pool: poolAccount,
        })
        .rpc();

      const poolData = await program.account.settlementPool.fetch(poolAccount);
      console.log("Pool Statistics:");
      console.log("  Total Settled:", poolData.totalSettled.toString());
      console.log("  Total Fees:", poolData.totalFeesCollected.toString());
      console.log("  Fee %:", poolData.feePercentage / 100);
    });
  });
});
