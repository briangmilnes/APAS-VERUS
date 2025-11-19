//! Test whether public fields allow direct construction from outside the module

pub mod inner {
    // Struct with public fields but NO public constructor
    pub struct PublicFieldsStruct {
        pub x: u32,
        pub y: u32,
    }
    
    // Only private constructor that enforces invariant y == x + 1
    impl PublicFieldsStruct {
        fn _new_valid(x: u32) -> Self {
            PublicFieldsStruct { x, y: x + 1 }
        }
    }
    
    // Struct with private fields
    pub struct PrivateFieldsStruct {
        x: u32,
        y: u32,
    }
    
    impl PrivateFieldsStruct {
        pub fn new(x: u32) -> Self {
            PrivateFieldsStruct { x, y: x + 1 }
        }
        
        pub fn get_x(&self) -> u32 { self.x }
        pub fn get_y(&self) -> u32 { self.y }
    }
}

pub mod outer {
    use super::inner::*;
    
    pub fn test_public_fields() {
        // CAN we construct PublicFieldsStruct directly from outside?
        let bad = PublicFieldsStruct { x: 10, y: 999 };  // Should violate invariant y == x + 1!
        
        println!("bad.x = {}, bad.y = {} (expected y = 11, but got {}!)", bad.x, bad.y, bad.y);
        assert!(bad.x == 10);
        assert!(bad.y == 999);  // Invariant broken!
    }
    
    pub fn test_private_fields() {
        // CAN'T construct PrivateFieldsStruct directly - this should fail to compile:
        // let bad = PrivateFieldsStruct { x: 10, y: 999 };  // Uncomment to see error
        
        let good = PrivateFieldsStruct::new(10);
        println!("good.x = {}, good.y = {}", good.get_x(), good.get_y());
        assert!(good.get_x() == 10);
        assert!(good.get_y() == 11);
    }
}

fn main() {
    outer::test_public_fields();
    outer::test_private_fields();
}
