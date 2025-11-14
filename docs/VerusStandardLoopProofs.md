<style>
body, .markdown-body {
    max-width: 1600px !important;
    margin: 0 auto;
}
</style>

# A Top Down View of How Verus Proves Loops

Here we present some trivial loops on vectors supported in Verus and how a user would prove them
using asserts.

**Sections:**
1. The Basic Loops
2. The Asserted Loops  
3. The Verus Compilation and Verification Pipeline
4. How Verus Annotates Loops for Proof

---

## 1. The Basic Loops

### a. While Loop

```rust
pub fn vec_length_while(s: &Vec<usize>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;

    while i < s.len()
        invariant
            i <= s.len(),
            length == i,
        decreases s.len() - i,
    {
        i += 1;
        length += 1;
    }
    length
}
```

### b. Loop (with break)

```rust
pub fn vec_length_loop(s: &Vec<usize>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;
    
    loop
        invariant
            i <= s.len(),
            length == i,
        decreases s.len() - i,
    {
        if i >= s.len() {
            return length;
        }
        i += 1;
        length += 1;
    }
}
```

### c. For Loop (Range)

```rust
pub fn vec_length_for_range(s: &Vec<usize>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    
    for _i in 0..s.len()
        invariant
            length == _i,
    {
        length += 1;
    }
    length
}
```

### d. For Loop (General - over collection)

```rust
// d. For Loop (General - over Vec consuming iterator) - Membership check

#[verifier::external_body] // The proof fails so we turn it off here. 
pub fn vec_mem_for_vec(v: Vec<usize>, elt: usize) -> (result: bool)
     ensures result == seq_usize_mem(v@, elt)
 {
     for val in v
     {
         if val == elt {
             return true;
         }
     }
     false
 }
```

## 2. The Asserted Loops

### a. While Loop

```rust
pub fn vec_length_while_asserted(s: &Vec<usize>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;
    
    assert(i <= s.len());
    assert(length == i);
    
    while i < s.len()
        invariant
            i <= s.len(),
            length == i,
        decreases s.len() - i,
    {
        i += 1;
        length += 1;
        
        assert(i <= s.len());
        assert(length == i);
    }
    
    assert(length == s@.len());
    length
}
```

### b. Loop (with return)

`break` doesn't propagate the exit condition, making postconditions unprovable. Use `return` instead.

#### b2. Loop with break - DOES NOT VERIFY

```rust
// b2. Loop with break - DOES NOT VERIFY
// Problem: break exits without propagating the exit condition
// After break, Verus only knows invariants, not which path caused exit
// Requires invariant_except_break (not implemented in current Verus)
// Attempted workaround with ghost variable fails at: assert(exit_cond)

// The proof fails so we turn it off here. 
#[verifier::external_body]
pub fn vec_length_loop_break(s: &Vec<usize>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;
    let ghost mut exit_cond: bool = false;
    
    loop
        invariant
            i <= s.len(),
            length == i,
            exit_cond ==> i == s.len(),
        decreases s.len() - i,
    {
        if i >= s.len() {
            proof { exit_cond = true; }
            break;
        }
        i += 1;
        length += 1;
    }
    
    assert(exit_cond);  // FAILS: Verus doesn't know exit_cond is true after break
    length
}
```

```rust
pub fn vec_length_loop_return_asserted(s: &Vec<usize>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    let mut i: usize = 0;
    
    assert(i <= s.len());
    assert(length == i);
    
    loop
        invariant
            i <= s.len(),
            length == i,
        decreases s.len() - i,
    {
        if i >= s.len() {
            assert(i == s.len());
            assert(length == i);
            assert(length == s@.len());
            return length;
        }
        
        i += 1;
        length += 1;
        
        assert(i <= s.len());
        assert(length == i);
    }
}
```

### c. For Loop (Range)

```rust
pub fn vec_length_for_range_asserted(s: &Vec<usize>) -> (length: usize)
    ensures length == s@.len()
{
    let mut length: usize = 0;
    
    assert(length == 0);
    assert(0 <= s.len());  // range bounds
    
    for _i in 0..s.len()
        invariant
            length == _i,
    {
        length += 1;
        assert(length == _i + 1);
    }
    
    assert(length == s@.len());
    length
}
```

## 3. The Verus Compilation and Verification Pipeline

Verus transforms your code through multiple intermediate representations (IRs) before verification:

```
Verus Code (.rs with verus! macro)
    ↓
Rust Compiler (rustc)
    ↓
Rust HIR (High-level Intermediate Representation)
    ↓ [rust_verify/src/rust_to_vir_expr.rs]
VIR AST (Verification IR - Abstract Syntax Tree)
    ↓ [vir/src/ast_to_sst.rs]
VIR SST (Verification IR - Statement-oriented Syntax Tree)
    ↓ [vir/src/sst_to_air.rs]
AIR (Assertion Intermediate Representation)
    ↓ [air/src/]
SMT Queries (Z3 format)
    ↓
Z3 SMT Solver
    ↓
✅ Verified or ❌ Error
```

### What Each IR Is:

**Rust HIR (High-level Intermediate Representation)**
- Rust compiler's internal representation after parsing and macro expansion
- Mutually recursive expressions and statements
- Includes type information from Rust's type checker

**VIR AST (Verification IR - Abstract Syntax Tree)**
- Verus's subset of Rust features that can be verified
- Separates `spec`, `proof`, and `exec` modes
- Still has mutually recursive expressions/statements

**VIR SST (Statement-oriented Syntax Tree)**
- **Key transformation**: Expressions no longer contain statements
- Flattens control flow into statement sequences
- `let x = if P { 5 } else { 10 };` → separate statements with temp vars

**AIR (Assertion Intermediate Representation)**
- Pure verification language with `assert`, `assume`, mutable local variables
- **If-branches inject `assume(P)` and `assume(!P)`** at `/home/milnes/projects/verus-lang/source/vir/src/sst_to_air.rs:2343`
- Loops become recursive functions with invariants as pre/postconditions

**SMT Queries (Z3)**
- Low-level logical formulas in SMT-LIB format
- Z3 solver checks satisfiability

### Generating IR Files:

```bash
verus --log vir --log vir-sst --log air --log smt --log-dir ./logs file.rs
```

Produces:
- `crate.vir` - VIR AST
- `root-sst.vir` - VIR SST
- `root.air` - AIR
- `root.smt2` - SMT queries

**Documentation**: `/home/milnes/projects/verus-lang/source/CODE.md` and `/home/milnes/projects/verus-lang/source/vir/src/lib.rs`

## 4. How Verus Annotates Loops for Proof
