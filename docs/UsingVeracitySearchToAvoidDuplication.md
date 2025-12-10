# Using veracity-search to Avoid Duplicating vstd


Opus-4.5 says:
When verusifying MathSeq, I initially marked `subseq` and `subseq_copy` as
`external_body`, assuming vstd lacked specs for slice operations. Only after the user
prompted me to search properly did I discover vstd already provides exactly what I needed:
`slice_subrange` (returns `&[T]` with `ensures out@ == slice@.subrange(i, j)`) and
`slice_to_vec` (returns `Vec<T>` with `ensures out@ == slice@`). The lesson: before
writing `external_body`, use type-based search patterns like `veracity-search -v 'fn
.*slice.*'` or `veracity-search -v 'fn _ ( : &[T] )'` to find existing vstd functions.

Text-based grep misses the structure. When I searched for "subrange" I found
`Seq::subrange` (spec-level) but missed the exec-level `slice_subrange`. Type-based
patterns like `fn _ -> Vec` or `fn _ types Seq` search by signature shape, not string
matching. For my use case, `veracity-search -v 'fn .*to_vec.*'` instantly found
`slice_to_vec` in vstd/slice.rs. The adjacent `slice_subrange` solved my other
hole. Always search vstd before assuming you need `external_body`—the function you need
may already exist with the spec you want.

