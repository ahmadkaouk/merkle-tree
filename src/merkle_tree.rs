use crate::Hasher;

/// A Merkle tree.
///
/// `levels` is a vector of vectors. The first vector contains the root of the tree, the second
/// vector contains the root's children, and so on. The last vector contains the leaves.
///
pub struct MerkleTree<T: Hasher> {
    levels: Vec<Vec<T::Hash>>,
}

impl<T: Hasher> MerkleTree<T> {
    /// Create a new Merkle tree with a given height.
    ///
    /// `height` is the height of the tree. The height of a tree is the number of levels in the
    /// tree, not counting the leaves. For example, a tree with height 2 has 3 levels: the root,
    /// the root's children, and the leaves. A tree with height 1 has only a root.
    ///
    /// The leaves of the tree are empty hashes. The other levels are filled in from the bottom up.
    ///
    /// # Examples
    /// ```
    /// use merkle_tree::MerkleTree;
    /// let tree: MerkleTree<T> = MerkleTree::new(2);
    /// assert_eq!(tree.leaf(0), &vec![]);
    /// assert_eq!(tree.leaf(1), &vec![]);
    /// assert_eq!(tree.leaf(2), &vec![]);
    /// assert_eq!(tree.leaf(3), &vec![]);
    /// ```
    ///
    pub fn new(height: usize) -> Self {
        let mut levels = Vec::new();

        // The last level contains the leaves, which are empty hashes.
        levels.push(vec![T::hash(vec![]); 2 << height]);

        // The other levels are filled in from the bottom up.
        for i in (0..height).rev() {
            let mut level: Vec<T::Hash> = Vec::new();
            for j in 0..(2 << i) {
                let mut data = Vec::new();
                data.extend(levels.last().unwrap()[j * 2].clone().into().iter());
                data.extend(levels.last().unwrap()[j * 2 + 1].clone().into().iter());
                level.push(data.into());
            }
            levels.push(level);
        }

        levels.reverse();
        MerkleTree { levels }
    }

    /// Get the root of the tree.
    pub fn root(&self) -> &T::Hash {
        &self.levels[0][0]
    }

    /// Get the hash of a leaf.
    /// `index` is the index of the leaf.
    /// The leaves are numbered from left to right, starting at 0. For example, if the tree has
    /// height 2, the leaves are numbered 0, 1, 2, and 3.
    pub fn leaf(&self, index: usize) -> &T::Hash {
        &self.levels[self.levels.len() - 1][index]
    }

    /// Insert a new value and recalculate the tree.
    /// `data` is the data to be inserted.
    /// The data is hashed and inserted into the first empty leaf. If the last level is full,
    /// the tree is resized. After the data is inserted, the tree is recalculated.
    pub fn insert(&mut self, data: impl Into<Vec<u8>>) {
        // Find the first empty leaf and insert the data. If the last level is full, resize the
        // tree.
        if let Some((mut index, _x)) = self
            .levels
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_i, x)| x.as_ref().is_empty())
        {
            self.levels.last_mut().unwrap()[index] = T::hash(data);

            // Recompute the branch of the tree that contains the new leaf from the bottom up.
            for i in (0..self.levels.len() - 1).rev() {
                index /= 2;
                let mut hash = Vec::new();
                hash.extend(self.levels[i + 1][index * 2].clone().into().iter());
                hash.extend(self.levels[i + 1][index * 2 + 1].clone().into().iter());
                self.levels[i][index] = T::hash(hash);
            }
        } else {
            self.reseize_and_insert(data);
        }
    }

    /// Resize the tree and insert a new value.
    /// `data` is the data to be inserted.
    ///
    /// The tree is resized by adding a new level to the top of the tree and doubling the number
    /// of leaves. The new leaf is inserted into the first empty leaf. The tree is then recalculated
    /// from the bottom up.
    fn reseize_and_insert(&mut self, data: impl Into<Vec<u8>>) {
        // Add a new level to the tree on the top
        self.levels.insert(0, vec![]);
        self.levels.last_mut().unwrap().push(T::hash(data));

        let len = self.levels.last().unwrap().len();
        self.levels
            .last_mut()
            .unwrap()
            .resize(2 * len, T::Hash::from(vec![]));

        // The starting index of the nodes that need to be recomputed.
        let mut index = len / 2;
        // Recompute the new branches of the tree from the bottom up.
        for i in (0..self.levels.len() - 1).rev() {
            for j in index..(2 << i) {
                let mut hash = Vec::new();
                hash.extend(self.levels[i + 1][j * 2].clone().into().iter());
                hash.extend(self.levels[i + 1][j * 2 + 1].clone().into().iter());
                self.levels[i][j] = T::hash(hash);
            }
            index /= 2;
        }
    }
}
