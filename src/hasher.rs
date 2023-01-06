/// ['Hasher'] is a trait that defines a hash function for a Merkle tree.
///
/// # Examples
/// ```
/// use merkle_tree::Hasher;
/// use sha2::{Sha256, Digest};
///
/// struct MyHasher;
///
/// impl Hasher for MyHasher {
///
///    type Hash = String;
///
///    fn hash(data: impl Into<Vec<u8>>) -> Self::Hash {
///        let mut hasher = Sha256::new();
///        hasher.update(data.into());
///        hasher.finalize().to_vec()
///    }
/// }
///
/// let hash = MyHasher::hash("hello world".as_bytes());
/// assert_eq!(hash, [185, 77, 39, 185, 147, 77, 62, 8, 165, 46, 82, 215, 218, 125, 171, 250, 196, 132, 239, 227, 122, 83, 128, 238, 144, 136, 247, 172, 226, 239, 205, 233]);
/// ```
pub trait Hasher {
    /// The type of the hash. This is the type that the [`hash`] function returns.
    type Hash: Clone + Into<Vec<u8>> + From<Vec<u8>> + AsRef<[u8]>;

    /// Hash the given data.
    fn hash(data: impl Into<Vec<u8>>) -> Self::Hash;
}
