Skewb
=====

Code for drawing, manipulating, and finding the shortest solution for a skewb.

Terminology
-----------

To **turn** the skewb around a corner means actually rearranging the pieces. To
**rotate** a skewb is just to look at it from a different angle without changing
the state.

The skewb has eight corner pieces. If you disassemble a real skewb, you will
find that four of these are **fixed** to the inner mechanism while the other
four are **floating** and are not attached to anything.

A **normalized** skewb has been rotated so that it is in the canonical position.
The **canonical position** is when the yellow-orange-green corner piece is in
the upper back left position and the yellow-red-blue corner piece is in the
upper front right position. This rotation is always possible since these are
both fixed corners. Normalization makes comparing skewbs for equality up to
rotation easier.

Future work
-----------
- The skewb has a small enough state space that it would be feasible to cache
  the shortest solution for every state.
- When converting a `Skewb` to a `NormalizedSkewb`, automatically rotate the
  `Skewb` to the canonical position. Currently, normalization fails if the skewb
  is not already in the canonical position.
- Remove the `even_rotation` field from `Skewb` because it is redundant. We can
  compute it from the corner positions.
