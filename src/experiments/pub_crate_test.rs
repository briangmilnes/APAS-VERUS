//! Test Rust's restricted pub visibility modifiers

pub mod inner {
    // pub(crate) = visible within this crate only
    pub struct PubCrateFieldsStruct {
        pub(crate) x: u32,
        pub(crate) y: u32,
    }
    
    impl PubCrateFieldsStruct {
        fn _new_valid(x: u32) -> Self {
            PubCrateFieldsStruct { x, y: x + 1 }
        }
    }
    
    // pub(super) = visible to parent module
    pub struct PubSuperFieldsStruct {
        pub(super) x: u32,
        pub(super) y: u32,
    }
    
    impl PubSuperFieldsStruct {
        fn _new_valid(x: u32) -> Self {
            PubSuperFieldsStruct { x, y: x + 1 }
        }
    }
}

pub mod outer {
    use super::inner::*;
    
    pub fn test_pub_crate() {
        // Can we construct PubCrateFieldsStruct directly?
        let bad = PubCrateFieldsStruct { x: 10, y: 999 };
        println!("pub(crate): bad.x = {}, bad.y = {}", bad.x, bad.y);
    }
    
    pub fn test_pub_super() {
        // Can we construct PubSuperFieldsStruct directly?
        let bad = PubSuperFieldsStruct { x: 10, y: 999 };
        println!("pub(super): bad.x = {}, bad.y = {}", bad.x, bad.y);
    }
}

fn main() {
    outer::test_pub_crate();
    outer::test_pub_super();
    println!("\nConclusion: pub(crate) and pub(super) still allow struct literal construction!");
}

