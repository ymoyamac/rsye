# AVL Binary Search Tree in Rust

A implementation of a Binary Search Tree (BST) with AVL balancing written in Rust, using raw pointers via `Option<NonNull<T>>` for manual memory management.

---

## What is a Binary Tree?

A **binary tree** is a hierarchical data structure where each node has at most two children, referred to as the **left child** and the **right child**. The topmost node is called the **root**, and nodes with no children are called **leaves**.

```
        8
       / \
      3   10
     / \    \
    1   6    14
       / \   /
      4   7 13
```

Binary trees are the foundation for many efficient data structures and algorithms, including search trees, heaps, and expression parsers.

---

## Binary Search Tree (BST)

A **Binary Search Tree** is a binary tree with an ordering invariant: for every node, all values in its left subtree are **less than** the node's value, and all values in its right subtree are **greater than** the node's value.

This property enables efficient operations:

| Operation | Average Case | Worst Case (unbalanced) |
|-----------|-------------|------------------------|
| Search    | O(log n)    | O(n)                   |
| Insert    | O(log n)    | O(n)                   |
| Delete    | O(log n)    | O(n)                   |

The worst case occurs when the tree degenerates into a linked list — for example, inserting elements in sorted order:

```
1
 \
  2
   \
    3
     \
      4   ← linear, not logarithmic
```

This is the problem AVL balancing solves.

---

## AVL Balancing

An **AVL tree** (named after Adelson-Velsky and Landis) is a self-balancing BST that maintains a **balance invariant**: for every node, the height difference between its left and right subtrees is at most 1.

```
Balance Factor = height(left subtree) − height(right subtree)
Valid values:  -1, 0, 1
```

Any insertion or deletion that violates this invariant triggers one or more **rotations** to restore balance.

### The 4 Rotations

**Left Rotation** — applied when the right subtree is too heavy to the right:
```
  x                y
   \              / \
    y    →       x   z
     \
      z
```

**Right Rotation** — applied when the left subtree is too heavy to the left:
```
      z          y
     /           / \
    y    →      x   z
   /
  x
```

**Left-Right Rotation** — double rotation for a left subtree heavy to the right.

**Right-Left Rotation** — double rotation for a right subtree heavy to the left.

After every insertion or deletion, the tree walks back up to the root recalculating heights and applying rotations where needed.

---

## Tree Traversals

Three standard ways to visit every node in the tree:

**Inorder** (left → root → right) — produces values in sorted ascending order for a BST.

**Preorder** (root → left → right) — useful for serializing or copying the tree structure.

**Postorder** (left → right → root) — useful for deletion and freeing memory bottom-up.

---

## Implementation Details

### Why `Option<NonNull<T>>`?

This implementation uses `Option<NonNull<T>>` instead of safe alternatives like `Box<T>` or `Rc<RefCell<T>>` for the following reasons:

- **Parent pointers** — each node holds a pointer back to its parent, which creates reference cycles. `Box<T>` cannot model cycles (it enforces single ownership), and `Rc<RefCell<T>>` handles cycles but with runtime overhead and ergonomic cost.
- **`NonNull<T>`** is a raw pointer wrapper that guarantees non-nullness, allowing `Option<NonNull<T>>` to be the same size as a raw pointer (null-pointer optimization).
- Manual memory management via `alloc` / `dealloc` gives full control over node lifecycle, matching how the structure would be implemented in C or C++.

### Node Structure

```
Node {
    value:   T,
    height:  u32,
    parent:  Option<NonNull<Node<T>>>,
    left:    Option<NonNull<Node<T>>>,
    right:   Option<NonNull<Node<T>>>,
    _marker: PhantomData<T>,
}
```

`PhantomData<T>` is a zero-sized marker that tells the compiler the tree is **covariant** over `T` — meaning an `AVLTree<Dog>` can be used where an `AVLTree<Animal>` is expected, which is the correct variance for a structure that only produces values of type `T`.

### Generics and Trait Bounds

The tree is generic over `T` with the bound `T: Ord`, which requires that values can be totally ordered — a necessary condition for BST insertion and search.

---

## Safety

This implementation uses `unsafe` Rust blocks for pointer dereferencing and memory allocation. The caller is responsible for:

- Not accessing the tree after it has been dropped.
- Not holding raw references across mutations.

The tree enforces memory safety at the structural level by carefully managing pointer consistency across all insertions, deletions, and rotations.

---

## Operations

- `insert(value)` — inserts a value maintaining BST order and AVL balance
- `search(value)` → `bool` — returns whether a value exists in the tree
- `delete(value)` — removes a value and rebalances if necessary
- `inorder()` — returns all values in sorted order
- `height()` — returns the height of the tree

---

## References

- Adelson-Velsky, G.; Landis, E. M. (1962). *An algorithm for the organization of information*
- [The Rustonomicon — Working with Unsafe](https://doc.rust-lang.org/nomicon/)
- [Rust Reference — Subtyping and Variance](https://doc.rust-lang.org/reference/subtyping.html)
