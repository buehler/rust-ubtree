# Rust UBTree

This is an implementation of the UBTree (Unlimited Branching Tree) in Rust. It is based on the paper [A New Method to Index and Query Sets](https://www.ijcai.org/Proceedings/99-1/Papers/067.pdf) by Jorg HOffmann and Jana Koehler from the University in Freiburg.

I implemented this structure to understand the underlying data structure that powers [KLEE](https://klee-se.org/) (a symbolic execution engine) and to - somewhat - benchmark the datastructure. The structure creates very good results without any big optimization (and used on `i32` or `u32` types).

The data stucture supports the purposed operations in the paper:

- `insert`; insert a new set into the tree
- `exists` (known in the paper as `lookup_first`); check if any set exists in the tree, such that the set is a subset of the query; meaning $`\exists s \in S \text{ in } T \text{ with } s \subseteq q`$
- `subsets` (known in the paper as `lookup_subs`); find all sets in the tree, such that the set is a subset of the query; meaning $`\forall s \in S \text{ in } T \text{ with } s \subseteq q`$
- `supersets` (known in the paper as `lookup_sups`); find all sets in the tree, such that the set is a superset of the query; meaning $`\forall s \in S \text{ in } T \text{ with } s \supseteq q`$

> Note: s is a set, S is the set of sets (all sets in the tree), T is the tree, and q is the query set.

The implementation makes use of binary search and simple heap vectors in rust.

### Insert

Inserting into the tree works as follows:

- If the inserted set is empty, return
- Sort the inserting set
- Reference the root nodes children
- Get the last element of the inserting set (for reference)
- Iterate through the inserting set and perform:
  - Search the current array of nodes for the current element
  - If a node with the given element is found, check if the element was the last in the inserting set. If it was, set the nodes `end of path`, if note, reference the found nodes children and continue
  - If no node with the given element is found, create a new node and insert it at the corresponding binary tree position and check the end of path status like above. Then reference the (obviously empty) children of the new node and continue

### Exists

Checking if a set exists in the tree that is a subset of the query set works as follows:

- If the query set is empty, return `false`
- Sort the query set
- Reference the root nodes children
- For each element in the query set perform:
  - Check if the current children contain a node with the current element
  - If there is a node and the node is an "end of path", return `true`, otherwise reference its children and continue
  - If there are no nodes with current elements and no end of path was found, return `false`

### Subsets

Finding all sets in the tree that are a subset of the query set works basically like searching for any existing set that is a subset of the query. The only change is that all reachable nodes are evaluated and not only the first one that is an end of path. All nodes that are encountered that are an end of path are added to the result set.

### Supersets

Finding all sets in the tree that are a superset of the query set works as follows:

- If the query set is empty, return all sets in the tree
- Sort the query set
- Reference the root nodes children
- Iterate through the query set and perform for each element:
  - Find a node in the currently referenced children that contains the current element
  - If there is one, reference its children and continue
- The result of the iteration is a reduced subtree that contains all sets that are supersets of the query set
- Calculate and return all end of pathes in the reduced subtree
