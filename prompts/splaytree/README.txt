Nipkow's Isabelle Splay Tree Formalization — Reference for APAS-VERUS Chap37

Source: Archive of Formal Proofs (AFP)
  Splay_Tree entry: https://devel.isa-afp.org/entries/Splay_Tree.html
  Amortized_Complexity entry: https://devel.isa-afp.org/entries/Amortized_Complexity.html

Files in this directory:

  CORRECTNESS (from AFP Splay_Tree entry)
  ----------------------------------------
  Splay_Tree.thy.txt          Core splay operations (splay, insert, delete, splay_max)
                               and BST correctness proofs. inv = λ_. True.
  Splay_Map.thy.txt           Map implementation on splay trees (key-value pairs).
  Splay_Heap.thy.txt          Splay heaps (Okasaki): partition-based priority queue.

  AMORTIZED ANALYSIS (from AFP Amortized_Complexity entry)
  --------------------------------------------------------
  Splay_Tree_Analysis_Base.thy.txt   Potential function φ/Φ, timing functions T_splay etc.
  Splay_Tree_Analysis.thy.txt        Standard O(log n) amortized bounds.
  Splay_Tree_Analysis_Optimal.thy.txt  Optimal constants via Schoenmakers' technique.
  Splay_Heap_Analysis.thy.txt        Amortized analysis of splay heaps.

Key takeaways for BSTSplay verification:
  1. Correctness needs NO structural invariant (inv = True). BST property suffices.
  2. splay preserves inorder traversal (content), BST property, and tree size.
  3. splay brings the target (or nearest element) to the root.
  4. insert = splay + split root; delete = splay + join subtrees via splay_max.
  5. Amortized analysis uses potential Φ = Σ log₂(size1(subtree)) over all nodes.
  6. The key inequality: 1 + log(x) + log(y) ≤ 2*log(x+y) for x,y > 0.
