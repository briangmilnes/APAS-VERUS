//  Experiment: DeepView on a struct with an Option<usize> field.


pub mod deep_view_struct {

    use vstd::prelude::*;

    verus! {

    pub struct Foo {
        pub val: Option<usize>,
    }

    impl View for Foo {
        type V = Option<usize>;

        open spec fn view(&self) -> Option<usize> {
            self.val
        }
    }

    impl DeepView for Foo {
        type V = Option<usize>;

        open spec fn deep_view(&self) -> Option<usize> {
            self.val
        }
    }

    pub trait FooTrait {
        spec fn spec_val(&self) -> Option<usize>;

        fn get_val(&self) -> (v: Option<usize>)
            ensures v == self.spec_val();
    }

    impl FooTrait for Foo {
        open spec fn spec_val(&self) -> Option<usize> {
            self.val
        }

        fn get_val(&self) -> (v: Option<usize>) {
            self.val
        }
    }

    fn test_deep_view() {
        let f = Foo { val: Some(42) };
        assert(f@ == Some(42usize));
        assert(f.deep_view() == Some(42usize));
    }

    } // verus!
}
