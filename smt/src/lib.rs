pub mod poseidon;
pub mod smt;

#[cfg(test)]
mod test {
    use crate::poseidon::{FieldHasher, Poseidon};
    use crate::smt::SparseMerkleTree;
    use halo2_proofs::arithmetic::{Field, FieldExt};
    use halo2_proofs::halo2curves::pasta::Fp;
    use rand::rngs::OsRng;

    //helper to change leaves array to BTreeMap and then create SMT
    fn create_merkle_tree<F: FieldExt, H: FieldHasher<F, 2>, const N: usize>(
        hasher: H,
        leaves: &[F],
        default_leaf: &[u8; 64],
    ) -> SparseMerkleTree<F, H, N> {
        SparseMerkleTree::<F, H, N>::new(leaves, &hasher, default_leaf).unwrap()
    }

    #[test]
    fn smt_proof() {
        let poseidon = Poseidon::<Fp, 2>::new();
        let default_leaf = [0u8; 64];
        let rng = OsRng;
        let leaves = [Fp::random(rng), Fp::random(rng), Fp::random(rng)];
        const HEIGHT: usize = 3;
        let smt = create_merkle_tree::<Fp, Poseidon<Fp, 2>, HEIGHT>(
            poseidon.clone(),
            &leaves,
            &default_leaf,
        );

        let proof = smt.generate_membership_proof(0);

        let res = proof
            .check_membership(&smt.root(), &leaves[0], &poseidon)
            .unwrap();
        assert!(res);
    }
}
