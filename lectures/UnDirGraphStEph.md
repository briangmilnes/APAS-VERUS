% UnDirGraphStEph — Data Struct, Trait, Impl
% Veracity Project
% 2026-05-09

# UnDirGraphStEph — Data Struct + View {.shrink}

```rust
#[verifier::reject_recursive_types(V)]
pub struct UnDirGraphStEph<V: StT + Hash> {
    pub V: SetStEph<V>,
    pub E: SetStEph<Edge<V>>,
}

impl<V: StT + Hash> View for UnDirGraphStEph<V> {
    type V = GraphView<<V as View>::V>;
    open spec fn view(&self) -> Self::V {
        GraphView { V: self.V@, A: self.E@ }
    }
}
```

# UnDirGraphStEph — Trait (`spec_neighborhood` + `neighborhood`) {.shrink}

```rust
pub trait UnDirGraphStEphTrait<V: StT + Hash>:
    View<V = GraphView<<V as View>::V>> + Sized
{
    open spec fn spec_neighborhood(&self, v: V::V) -> Set<V::V>
        recommends spec_graphview_wf(self@), self@.V.contains(v)
    { Set::new(|w| self@.A.contains((v,w)) || self@.A.contains((w,v))) }

    fn neighborhood(&self, v: &V) -> (nbrs: SetStEph<V>)
        requires spec_graphview_wf(self@), self@.V.contains(v@), ...
        ensures  nbrs@ == self.spec_neighborhood(v@), nbrs@ <= self@.V;
}
```

# UnDirGraphStEph — Impl: `neighborhood` exec body {.shrink}

```rust
fn neighborhood(&self, v: &V) -> (nbrs: SetStEph<V>) {
    let mut nbrs: SetStEph<V> = SetStEph::empty();
    let mut it = self.E.iter();
    let ghost edges_seq = it@.1;
    loop
        invariant nbrs@ == Set::new(|w| exists |i|
            0 <= i < it@.0 && /* edges_seq[i] hits v on either side */ ...),
        decreases edges_seq.len() - it@.0,
    {
        match it.next() {
            None    => { proof { /* set-equality lemmas */ } return nbrs; }
            Some(e) => { /* if feq(&e.0,v) insert e.1; sym. */ ... }
        }
    }
}
```
