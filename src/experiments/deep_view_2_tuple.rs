//  Experiment: deep_view on 2-tuples
//
//  HYPOTHESIS: vstd only declares DeepView for 1-tuples, so (K, Vec<V>)
//  does not implement DeepView and Vec<(K, Vec<V>)>.deep_view() won't work.
//
//  RESULT: Works.

use vstd::prelude::*;

verus! {

proof fn test_1_tuple_deep_view(v: Vec<(Vec<u32>,)>)
    ensures
        v.deep_view().len() == v@.len(),
{
}

proof fn test_2_tuple_deep_view(v: Vec<(u32, Vec<u32>)>)
    ensures
        v.deep_view().len() == v@.len(),
{
}

} // verus!
