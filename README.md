# Merkle Tree
This library provides a simple implementation of a Merkle tree in Rust. It is based on the [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree) data structure, which is a binary tree in which each leaf node is labelled with the hash of a data block and each non-leaf node is labelled with the cryptographic hash of the labels of its child nodes.

The tree has a fixed height and nodes can be accessed directly using an index, which allows for faster lookups and modifications of the tree data.

## Merkle Tree Representation
There are two main ways to represent a Merkle tree: as an array or as a linked list. This library uses an array-based representation, which is more efficient when the number of items in the tree is known in advance.


## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
merkle-tree = "0.1.0"
```

## Example
```rust
use merkle_tree::MerkleTree;


it is best to use an array-based representation for a Merkle tree when the number of items in the tree is known in advance and is relatively small.
This is because arrays have a fixed size and can be accessed directly using an index, which allows for faster lookups and modifications of the tree data.