import { describe, it, expect } from '@jest/globals';

/**
 * Utility functions for payment processing
 */

// Payment validation utilities
export const validatePaymentProof = (proof: string): boolean => {
  if (!proof || typeof proof !== 'string') return false;
  if (proof.length < 10) return false;
  // Basic base64 check
  return /^[A-Za-z0-9+/=]+$/.test(proof);
};

export const validateWalletAddress = (address: string, network: string): boolean => {
  if (!address || typeof address !== 'string') return false;

  switch (network) {
    case 'solana':
      // Solana addresses are 32-44 characters, base58
      return /^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(address);
    case 'ethereum':
      // Ethereum addresses are 42 characters with 0x prefix
      return /^0x[a-fA-F0-9]{40}$/.test(address);
    default:
      return false;
  }
};

export const calculateFee = (amount: number, feePercentage: number): number => {
  if (amount <= 0 || feePercentage < 0) return 0;
  return Math.floor((amount * feePercentage) / 10000);
};

export const formatAmount = (lamports: number): string => {
  const sol = lamports / 1_000_000_000;
  return `${sol.toFixed(9)} SOL`;
};

export const parseAmount = (solString: string): number => {
  const sol = parseFloat(solString.replace(/[^0-9.]/g, ''));
  return Math.floor(sol * 1_000_000_000);
};

export const isPaymentExpired = (timestamp: number, expirySeconds: number): boolean => {
  const now = Math.floor(Date.now() / 1000);
  return now > timestamp + expirySeconds;
};

export const generatePaymentHash = (proof: string, amount: number, recipient: string): string => {
  // Simple hash simulation (in production, use proper cryptographic hash)
  const data = `${proof}:${amount}:${recipient}`;
  return Buffer.from(data).toString('base64').slice(0, 32);
};

describe('Payment Utility Functions', () => {
  describe('validatePaymentProof', () => {
    it('should validate correct payment proof', () => {
      const validProof = 'QmFzZTY0RW5jb2RlZFBheW1lbnRQcm9vZg==';
      expect(validatePaymentProof(validProof)).toBe(true);
    });

    it('should reject empty proof', () => {
      expect(validatePaymentProof('')).toBe(false);
    });

    it('should reject short proof', () => {
      expect(validatePaymentProof('abc')).toBe(false);
    });

    it('should reject non-base64 proof', () => {
      expect(validatePaymentProof('invalid@#$%')).toBe(false);
    });

    it('should reject null or undefined', () => {
      expect(validatePaymentProof(null as any)).toBe(false);
      expect(validatePaymentProof(undefined as any)).toBe(false);
    });
  });

  describe('validateWalletAddress', () => {
    it('should validate Solana address', () => {
      const solanaAddress = 'GZ7X9FDJ8K4L5M6N7P8Q9R0S1T2U3V4W5X6Y7Z8A';
      expect(validateWalletAddress(solanaAddress, 'solana')).toBe(true);
    });

    it('should validate Ethereum address', () => {
      const ethAddress = '0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb';
      expect(validateWalletAddress(ethAddress, 'ethereum')).toBe(true);
    });

    it('should reject invalid Solana address', () => {
      expect(validateWalletAddress('invalid', 'solana')).toBe(false);
    });

    it('should reject invalid Ethereum address', () => {
      expect(validateWalletAddress('0xinvalid', 'ethereum')).toBe(false);
    });

    it('should reject empty address', () => {
      expect(validateWalletAddress('', 'solana')).toBe(false);
    });

    it('should reject unknown network', () => {
      expect(validateWalletAddress('someaddress', 'unknown')).toBe(false);
    });
  });

  describe('calculateFee', () => {
    it('should calculate 1% fee correctly', () => {
      const amount = 1_000_000;
      const fee = calculateFee(amount, 100); // 100 basis points = 1%
      expect(fee).toBe(10_000);
    });

    it('should calculate 0.5% fee correctly', () => {
      const amount = 1_000_000;
      const fee = calculateFee(amount, 50); // 50 basis points = 0.5%
      expect(fee).toBe(5_000);
    });

    it('should handle zero amount', () => {
      expect(calculateFee(0, 100)).toBe(0);
    });

    it('should handle negative amount', () => {
      expect(calculateFee(-1000, 100)).toBe(0);
    });

    it('should handle zero fee percentage', () => {
      expect(calculateFee(1_000_000, 0)).toBe(0);
    });

    it('should round down fractional fees', () => {
      const amount = 1_000_001;
      const fee = calculateFee(amount, 100);
      expect(fee).toBe(10_000); // Rounded down
    });
  });

  describe('formatAmount', () => {
    it('should format 1 SOL correctly', () => {
      const lamports = 1_000_000_000;
      expect(formatAmount(lamports)).toBe('1.000000000 SOL');
    });

    it('should format 0.5 SOL correctly', () => {
      const lamports = 500_000_000;
      expect(formatAmount(lamports)).toBe('0.500000000 SOL');
    });

    it('should format small amounts correctly', () => {
      const lamports = 1;
      expect(formatAmount(lamports)).toBe('0.000000001 SOL');
    });

    it('should format zero correctly', () => {
      expect(formatAmount(0)).toBe('0.000000000 SOL');
    });

    it('should format large amounts correctly', () => {
      const lamports = 1_000_000_000_000;
      expect(formatAmount(lamports)).toBe('1000.000000000 SOL');
    });
  });

  describe('parseAmount', () => {
    it('should parse "1 SOL" correctly', () => {
      const lamports = parseAmount('1 SOL');
      expect(lamports).toBe(1_000_000_000);
    });

    it('should parse "0.5 SOL" correctly', () => {
      const lamports = parseAmount('0.5 SOL');
      expect(lamports).toBe(500_000_000);
    });

    it('should parse plain number', () => {
      const lamports = parseAmount('2.5');
      expect(lamports).toBe(2_500_000_000);
    });

    it('should handle zero', () => {
      const lamports = parseAmount('0');
      expect(lamports).toBe(0);
    });

    it('should strip non-numeric characters', () => {
      const lamports = parseAmount('$1.50 SOL');
      expect(lamports).toBe(1_500_000_000);
    });
  });

  describe('isPaymentExpired', () => {
    it('should return false for non-expired payment', () => {
      const timestamp = Math.floor(Date.now() / 1000);
      const expirySeconds = 3600; // 1 hour
      expect(isPaymentExpired(timestamp, expirySeconds)).toBe(false);
    });

    it('should return true for expired payment', () => {
      const timestamp = Math.floor(Date.now() / 1000) - 7200; // 2 hours ago
      const expirySeconds = 3600; // 1 hour expiry
      expect(isPaymentExpired(timestamp, expirySeconds)).toBe(true);
    });

    it('should handle immediate expiry', () => {
      const timestamp = Math.floor(Date.now() / 1000);
      const expirySeconds = 0;
      expect(isPaymentExpired(timestamp, expirySeconds)).toBe(true);
    });

    it('should handle long expiry', () => {
      const timestamp = Math.floor(Date.now() / 1000);
      const expirySeconds = 86400 * 365; // 1 year
      expect(isPaymentExpired(timestamp, expirySeconds)).toBe(false);
    });
  });

  describe('generatePaymentHash', () => {
    it('should generate consistent hash for same inputs', () => {
      const proof = 'PROOF123';
      const amount = 1000000;
      const recipient = 'RECIPIENT_ADDRESS';

      const hash1 = generatePaymentHash(proof, amount, recipient);
      const hash2 = generatePaymentHash(proof, amount, recipient);

      expect(hash1).toBe(hash2);
    });

    it('should generate different hashes for different inputs', () => {
      const hash1 = generatePaymentHash('PROOF1', 1000000, 'RECIPIENT1');
      const hash2 = generatePaymentHash('PROOF2', 1000000, 'RECIPIENT1');

      expect(hash1).not.toBe(hash2);
    });

    it('should generate hash of correct length', () => {
      const hash = generatePaymentHash('PROOF', 1000000, 'RECIPIENT');
      expect(hash.length).toBe(32);
    });

    it('should handle empty inputs', () => {
      const hash = generatePaymentHash('', 0, '');
      expect(hash).toBeDefined();
      expect(hash.length).toBe(32);
    });
  });
});

describe('Payment Processing Edge Cases', () => {
  it('should handle very large amounts', () => {
    const largeAmount = Number.MAX_SAFE_INTEGER;
    const fee = calculateFee(largeAmount, 100);
    expect(fee).toBeGreaterThan(0);
    expect(fee).toBeLessThan(largeAmount);
  });

  it('should handle precision in amount conversion', () => {
    const solAmount = '1.123456789';
    const lamports = parseAmount(solAmount);
    const formatted = formatAmount(lamports);
    expect(formatted).toContain('1.123456789');
  });

  it('should validate international characters in addresses', () => {
    const invalidAddress = 'GZ7X9FDJ8K4L5M6Nöäü';
    expect(validateWalletAddress(invalidAddress, 'solana')).toBe(false);
  });

  it('should handle boundary conditions for expiry', () => {
    const now = Math.floor(Date.now() / 1000);

    // Exactly at expiry
    expect(isPaymentExpired(now - 3600, 3600)).toBe(false);

    // One second past expiry
    expect(isPaymentExpired(now - 3601, 3600)).toBe(true);
  });
});
