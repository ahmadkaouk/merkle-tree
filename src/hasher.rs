/// A trait for hashing data.
///
/// The `Hasher` trait is used to hash data. It is implemented for the `MerkleTree` struct.
///
/// # Example
///
/// ```
/// use merkle_tree::Hasher;
///
/// struct MyHasher;
///
/// impl Hasher for MyHasher {
///    type Hash = Vec<u8>;
///   type Data = Vec<u8>;
///
///    fn hash(data: impl Into<Self::Data>) -> Self::Hash {
///       vec![]
///   }
/// }
/// ```
///
pub trait Hasher {
    type Hash: AsRef<[u8]>;
    type Data: AsRef<[u8]>;

    fn hash(data: impl Into<Self::Data>) -> Self::Hash;
}