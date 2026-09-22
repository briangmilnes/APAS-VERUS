// r217: measure what `#[derive(Debug)]` does to a struct that is not a unit
// struct and whose field type carries no `Debug` impl — the shape of every
// constant-string `Debug` body this round leaves alone (`Seq`, `Map`, `Set`,
// `Ghost`, `RwLock`, `Box<dyn Fn>` fields). Plain Rust, no vstd.
// Run: rustc -O scratch/r217-derive-debug-non-unit.rs -o scratch/r217-derive-debug-non-unit

pub struct GhostLike;

#[derive(Debug)]
pub struct WithField {
    pub inner: GhostLike,
}

#[derive(Debug)]
pub struct WithPlainField {
    pub n: u64,
}

fn main() {
    // Prints the field, not the bare type name: the constant-string body it
    // would replace prints "WithPlainField" and the derive prints more.
    println!("{:?}", WithPlainField { n: 7 });
    println!("{:?}", WithField { inner: GhostLike });
}
