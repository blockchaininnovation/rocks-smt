# ZkRollup Rationale

## Abstract

- Rollup Tree Circuit
    - Merkle Tree Chip
        - [Inclusion](https://github.com/blockchaininnovation/rocks-smt/blob/8e7b828c813ba6f689bbe5fda0cf20a3ef97c02f/chiplet/src/smt_chip.rs#L184) ✅
        - [Update](https://github.com/blockchaininnovation/rocks-smt/blob/8e7b828c813ba6f689bbe5fda0cf20a3ef97c02f/chiplet/src/smt_chip.rs#L151) ✅
    - Field Chip
        - [Range Proof](https://docs.axiom.xyz/zero-knowledge-proofs/getting-started-with-halo2#range.rs) ✅
        - [Comparison](https://axiom-crypto.github.io/halo2-lib/halo2_base/gates/range/trait.RangeInstructions.html#tymethod.check_less_than) ✅
    - Hash Chip
        - [Hashs](https://github.com/blockchaininnovation/rocks-smt/blob/8e7b828c813ba6f689bbe5fda0cf20a3ef97c02f/chiplet/src/poseidon_chip.rs#L68) ✅
    - Eddsa Chip
        - [Hash Chip](https://github.com/blockchaininnovation/rocks-smt/blob/8e7b828c813ba6f689bbe5fda0cf20a3ef97c02f/chiplet/src/poseidon_chip.rs#L68) ✅
        - Verify Signature 🚧
    - Transaction Tree Circuit
        - Merkle Tree Chip

## Chip

ecdsa example
https://github.com/privacy-scaling-explorations/halo2wrong/blob/master/ecdsa/src/ecdsa.rs

Load Merkle tree, field and eddsa, hash chip.

## Rollup Circuit

**public io**: Transaction Merkle Tree Root, Transactions and Current State Root, Next State Root

### Sequence

Operator does

1. Receive User's Transaction
2. Compose
    - Transaction Merkle Tree Root
    - Transactions
    - Current State Root
    - Next State Root
3. Setup Circuit by Loading `Merkle Tree`, `Field` and `Eddsa`, `Hash` Chip
4. Check Existance of Relevant Accounts (Merkle Tree Chip)
5. Check Validity of Transfer Amount and Balance (Field Chip)
6. Check Signature (Eddsa Chip)
7. Check Validity of Sender Nonce
8. Check Exsistance in Transaction Merkle Tree (Transaction Merkle Circuit)
9. Compute Intermediate State Root

https://github.com/privacy-scaling-explorations/zkevm-circuits/blob/main/zkevm-circuits/src/state_circuit.rs#L50
