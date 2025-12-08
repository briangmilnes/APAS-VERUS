//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Thread utilities - self-contained copy of vstd::thread with is_finished added.
//! We copy rather than extend because vstd::thread doesn't expose is_finished.

#![allow(unused_imports)]

pub mod threads_plus {
    use core::marker;
    use vstd::prelude::*;

verus! {

    #[verifier::external_body]
    #[verifier::reject_recursive_types(Ret)]
    pub struct JoinHandlePlus<Ret> { handle: std::thread::JoinHandle<Ret> }

    impl<Ret> JoinHandlePlus<Ret> {
        pub uninterp spec fn predicate(&self, ret: Ret) -> bool;

    /// Check if thread finished without blocking. (Addition over vstd::thread.)
        #[verifier::external_body]
        pub fn is_finished(&self) -> (finished: bool) { self.handle.is_finished() }

        #[verifier::external_body]
        pub fn join(self) -> (res: Result<Ret, ()>)
            ensures match res { Result::Ok(r) => self.predicate(r), Result::Err(_) => true }
        {
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                match self.handle.join() { Ok(v) => Ok(v), Err(_) => Err(()) }
            }));
            match res {
                Ok(res) => res,
                Err(_) => { println!("panic on join"); std::process::abort(); }
            }
        }
    }

    #[verifier::external_body]
    pub fn spawn_plus<F, Ret>(f: F) -> (handle: JoinHandlePlus<Ret>)
    where F: FnOnce() -> Ret + Send + 'static, Ret: Send + 'static requires f.requires(()) ensures forall|ret: Ret| #[trigger] handle.predicate(ret) ==> f.ensures((), ret)
    {
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            JoinHandlePlus { handle: std::thread::spawn(move || f()) }
        }));
        match res {
            Ok(res) => res,
            Err(_) => { println!("panic on spawn_plus"); std::process::abort(); }
        }
    }

    #[verifier::external_body]
    pub struct ThreadIdPlus { thread_id: std::thread::ThreadId }

    #[cfg(verus_keep_ghost)]
    #[verifier::external_body]
    pub tracked struct IsThreadPlus {}

    #[cfg(verus_keep_ghost)]
    impl !Sync for IsThreadPlus {}

    #[cfg(verus_keep_ghost)]
    impl !Send for IsThreadPlus {}

    #[cfg(not(verus_keep_ghost))]
    #[verifier::external_body]
    pub tracked struct IsThreadPlus { _no_send_sync: core::marker::PhantomData<*const ()> }

    impl IsThreadPlus {
        pub uninterp spec fn view(&self) -> ThreadIdPlus;

        pub axiom fn agrees(tracked self, tracked other: IsThreadPlus)
        ensures self@ == other@;
    }

    #[verifier::external]
    impl Clone for IsThreadPlus {
        #[cfg(verus_keep_ghost)]
        fn clone(&self) -> Self { IsThreadPlus {} }

        #[cfg(not(verus_keep_ghost))]
        fn clone(&self) -> Self { IsThreadPlus { _no_send_sync: Default::default() } }
    }

    impl Copy for IsThreadPlus {}

    #[verifier::external_body]
    pub fn thread_id_plus() -> (res: (ThreadIdPlus, Tracked<IsThreadPlus>))
        ensures res.1@@ == res.0
    {
        let id = ThreadIdPlus { thread_id: std::thread::current().id() };
        (id, Tracked::assume_new())
    }

    pub axiom fn ghost_thread_id_plus() -> (tracked res: IsThreadPlus);

    #[verifier::external_body]
    #[verifier::accept_recursive_types(V)]
    tracked struct ThreadShareablePlus<V> { phantom: marker::PhantomData<V> }

    #[verifier::external]
    unsafe impl<V> Sync for ThreadShareablePlus<V> {}

    #[verifier::external]
    unsafe impl<V> Send for ThreadShareablePlus<V> {}

    impl<V> ThreadShareablePlus<V> {
        pub uninterp spec fn view(&self)                                 -> V;
        pub uninterp spec fn id(&self)                                   -> ThreadIdPlus;

        pub axiom fn into(tracked self, tracked is_thread: IsThreadPlus) -> (tracked res: V)
        requires self.id() == is_thread@
        ensures
            res == self@;
            pub axiom fn borrow(tracked &self, tracked is_thread: IsThreadPlus) -> (tracked res: &V)
        requires self.id() == is_thread@
        ensures *res == self@;
    }

    impl<V: Send> ThreadShareablePlus<V> {
        pub axiom fn send_into(tracked self) -> (tracked res: V)
        ensures res == self@;
    }

    impl<V: Sync> ThreadShareablePlus<V> {
        pub axiom fn sync_borrow(tracked &self) -> (tracked res: &V)
        ensures *res == self@;
    }

 } // verus!
}