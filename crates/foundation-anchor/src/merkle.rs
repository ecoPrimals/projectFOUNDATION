use crate::AnchorError;

/// A BLAKE3 Merkle tree built from certificate hashes.
///
/// Leaf nodes are the raw 32-byte certificate hashes. Internal nodes
/// are `BLAKE3(left || right)`. If the leaf count is odd, the last
/// leaf is promoted (not duplicated — no hash-with-self).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MerkleTree {
    /// The 32-byte root hash.
    root: [u8; 32],
    /// Leaf hashes (certificate hashes in insertion order).
    leaves: Vec<[u8; 32]>,
    /// Total number of internal nodes (for verification).
    node_count: usize,
}

impl MerkleTree {
    /// Build a Merkle tree from a slice of certificate hashes.
    ///
    /// # Errors
    ///
    /// Returns [`AnchorError::EmptyBatch`] if the slice is empty.
    pub fn from_certificates(hashes: &[[u8; 32]]) -> Result<Self, AnchorError> {
        if hashes.is_empty() {
            return Err(AnchorError::EmptyBatch);
        }

        if hashes.len() == 1 {
            return Ok(Self {
                root: hashes[0],
                leaves: hashes.to_vec(),
                node_count: 1,
            });
        }

        let mut current_level: Vec<[u8; 32]> = hashes.to_vec();
        let mut node_count = hashes.len();

        while current_level.len() > 1 {
            let mut next_level = Vec::with_capacity(current_level.len().div_ceil(2));

            let mut i = 0;
            while i < current_level.len() {
                if i + 1 < current_level.len() {
                    let mut combined = [0u8; 64];
                    combined[..32].copy_from_slice(&current_level[i]);
                    combined[32..].copy_from_slice(&current_level[i + 1]);
                    next_level.push(*blake3::hash(&combined).as_bytes());
                    node_count += 1;
                } else {
                    next_level.push(current_level[i]);
                }
                i += 2;
            }

            current_level = next_level;
        }

        Ok(Self {
            root: current_level[0],
            leaves: hashes.to_vec(),
            node_count,
        })
    }

    /// The 32-byte Merkle root.
    #[must_use]
    pub const fn root(&self) -> &[u8; 32] {
        &self.root
    }

    /// The leaf hashes in insertion order.
    #[must_use]
    pub fn leaves(&self) -> &[[u8; 32]] {
        &self.leaves
    }

    /// Number of leaves.
    #[must_use]
    pub fn leaf_count(&self) -> usize {
        self.leaves.len()
    }

    /// Total node count (leaves + internal).
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }

    /// Verify that rebuilding the tree from leaves produces the same root.
    #[must_use]
    pub fn verify(&self) -> bool {
        match Self::from_certificates(&self.leaves) {
            Ok(rebuilt) => rebuilt.root == self.root,
            Err(_) => false,
        }
    }

    /// Hex-encode the root for display / chain submission.
    #[must_use]
    pub fn root_hex(&self) -> String {
        use std::fmt::Write;
        self.root
            .iter()
            .fold(String::with_capacity(64), |mut s, b| {
                let _ = write!(s, "{b:02x}");
                s
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash_str(s: &str) -> [u8; 32] {
        *blake3::hash(s.as_bytes()).as_bytes()
    }

    #[test]
    fn single_leaf_is_identity() {
        let h = hash_str("cert-001");
        let tree = MerkleTree::from_certificates(&[h]).unwrap();
        assert_eq!(tree.root(), &h);
        assert_eq!(tree.leaf_count(), 1);
        assert!(tree.verify());
    }

    #[test]
    fn two_leaves_produce_deterministic_root() {
        let a = hash_str("cert-001");
        let b = hash_str("cert-002");
        let tree = MerkleTree::from_certificates(&[a, b]).unwrap();
        assert_ne!(tree.root(), &a);
        assert_ne!(tree.root(), &b);
        assert_eq!(tree.leaf_count(), 2);
        assert!(tree.verify());
    }

    #[test]
    fn order_matters() {
        let a = hash_str("cert-001");
        let b = hash_str("cert-002");
        let tree_ab = MerkleTree::from_certificates(&[a, b]).unwrap();
        let tree_ba = MerkleTree::from_certificates(&[b, a]).unwrap();
        assert_ne!(tree_ab.root(), tree_ba.root());
    }

    #[test]
    fn odd_leaf_count_handled() {
        let leaves: Vec<[u8; 32]> = (0..5).map(|i| hash_str(&format!("cert-{i:03}"))).collect();
        let tree = MerkleTree::from_certificates(&leaves).unwrap();
        assert_eq!(tree.leaf_count(), 5);
        assert!(tree.verify());
    }

    #[test]
    fn large_batch_verifies() {
        let leaves: Vec<[u8; 32]> =
            (0..1000).map(|i| hash_str(&format!("cert-{i:04}"))).collect();
        let tree = MerkleTree::from_certificates(&leaves).unwrap();
        assert_eq!(tree.leaf_count(), 1000);
        assert!(tree.verify());
    }

    #[test]
    fn empty_batch_rejected() {
        let result = MerkleTree::from_certificates(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn root_hex_is_64_chars() {
        let h = hash_str("cert-001");
        let tree = MerkleTree::from_certificates(&[h]).unwrap();
        assert_eq!(tree.root_hex().len(), 64);
    }

    #[test]
    fn rebuild_produces_same_root() {
        let leaves: Vec<[u8; 32]> =
            (0..17).map(|i| hash_str(&format!("cert-{i:03}"))).collect();
        let tree1 = MerkleTree::from_certificates(&leaves).unwrap();
        let tree2 = MerkleTree::from_certificates(&leaves).unwrap();
        assert_eq!(tree1.root(), tree2.root());
    }

    #[test]
    fn power_of_two_leaf_count() {
        let leaves: Vec<[u8; 32]> = (0..8).map(|i| hash_str(&format!("cert-{i:03}"))).collect();
        let tree = MerkleTree::from_certificates(&leaves).unwrap();
        assert_eq!(tree.leaf_count(), 8);
        assert!(tree.verify());
    }
}
