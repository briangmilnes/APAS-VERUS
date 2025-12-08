//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Thread utilities - self-contained copy of vstd::thread with is_finished added.
//!
//! This is a complete copy of vstd::thread with _plus suffix naming.
//! We copy rather than extend because vstd::thread doesn't expose is_finished.

#![allow(unused_imports)]

pub mod threads_plus {
    use core::marker;
    use vstd::prelude::*;

verus! {

/// Object returned by [`spawn_plus()`](spawn_plus) to allow thread joining.
/// (Wrapper around [`std::thread::JoinHandle`].)
#[verifier::external_body]
#[verifier::reject_recursive_types(Ret)]
pub struct JoinHandlePlus<Ret> {
    handle: std::thread::JoinHandle<Ret>,
}

impl<Ret> JoinHandlePlus<Ret> {
    /// Predicate restricting the possible return values. This is determined by the
    /// postcondition of the closure provided when calling `spawn_plus()`.
    pub uninterp spec fn predicate(&self, ret: Ret) -> bool;

    /// Check if the thread has finished without blocking.
    /// This is the addition over vstd::thread.
    #[verifier::external_body]
    pub fn is_finished(&self) -> (finished: bool) {
        self.handle.is_finished()
    }

    /// Wait for a thread to complete.
    #[verifier::external_body]
    pub fn join(self) -> (r_result: Result<Ret, ()>)
        ensures
            match r_result {
                Result::Ok(r) => self.predicate(r),
                Result::Err(_) => true,
            },
    {
        let res = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(
                || {
                    match self.handle.join() {
                        Ok(v) => Ok(v),
                        Err(_) => Err(()),
                    }
                },
            ),
        );
        match res {
            Ok(res) => res,
            Err(_) => {
                println!("panic on join");
                std::process::abort();
            },
        }
    }
}

/// Spawns a thread. (Wrapper around [`std::thread::spawn`].)
///
/// This takes as input a `FnOnce` closure with no arguments.
/// The `spawn_plus` returns a [`JoinHandlePlus`], on which the client can call
/// [`join()`](JoinHandlePlus::join) to wait for the thread to complete, or
/// [`is_finished()`](JoinHandlePlus::is_finished) to poll without blocking.
/// The return value of the closure is returned via `join()`.
///
/// The closure must be callable (i.e., its precondition must be satisfied)
/// but it may have an arbitrary postcondition. The postcondition is passed through
/// the `JoinHandlePlus` via [`JoinHandlePlus::predicate()`](JoinHandlePlus::predicate).
#[verifier::external_body]
pub fn spawn_plus<F, Ret>(f: F) -> (handle: JoinHandlePlus<Ret>) where
    F: FnOnce() -> Ret,
    F: Send + 'static,
    Ret: Send + 'static,

    requires
        f.requires(()),
    ensures
        forall|ret: Ret| #[trigger] handle.predicate(ret) ==> f.ensures((), ret),
{
    let res = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(
            || {
                let handle = std::thread::spawn(move || f());
                JoinHandlePlus { handle }
            },
        ),
    );
    match res {
        Ok(res) => res,
        Err(_) => {
            println!("panic on spawn_plus");
            std::process::abort();
        },
    }
}

/// Wrapper around Rust's [`ThreadId`].
/// This is an opaque type.
#[verifier::external_body]
pub struct ThreadIdPlus {
    thread_id: std::thread::ThreadId,
}

/// Proof object that guarantees the owning thread has the given ThreadIdPlus.
#[cfg(verus_keep_ghost)]
#[verifier::external_body]
pub tracked struct IsThreadPlus {}

#[cfg(verus_keep_ghost)]
impl !Sync for IsThreadPlus {}

#[cfg(verus_keep_ghost)]
impl !Send for IsThreadPlus {}

// TODO: remove this when !Sync, !Send are supported by stable Rust
#[cfg(not(verus_keep_ghost))]
#[verifier::external_body]
pub tracked struct IsThreadPlus {
    _no_send_sync: core::marker::PhantomData<*const ()>,
}

impl IsThreadPlus {
    pub uninterp spec fn view(&self) -> ThreadIdPlus;

    /// Guarantees that any two `IsThreadPlus` objects on the same thread
    /// will have the same ID.
    pub axiom fn agrees(tracked self, tracked other: IsThreadPlus)
        ensures
            self@ == other@,
    ;
}

#[verifier::external]
impl Clone for IsThreadPlus {
    #[cfg(verus_keep_ghost)]
    fn clone(&self) -> Self {
        IsThreadPlus {}
    }

    #[cfg(not(verus_keep_ghost))]
    fn clone(&self) -> Self {
        IsThreadPlus { _no_send_sync: Default::default() }
    }
}

impl Copy for IsThreadPlus {}

/// Gets the current thread ID. Also returns a ghost object representing proof of being on this thread.
#[verifier::external_body]
pub fn thread_id_plus() -> (res: (ThreadIdPlus, Tracked<IsThreadPlus>))
    ensures
        res.1@@ == res.0,
{
    let id: std::thread::ThreadId = std::thread::current().id();
    let id = ThreadIdPlus { thread_id: id };
    (id, Tracked::assume_new())
}

/// Returns _just_ the ghost object, without physically obtaining the thread ID.
pub axiom fn ghost_thread_id_plus() -> (tracked res: IsThreadPlus);

/// Tracked object that makes any tracked object `Send` or `Sync`.
/// Requires the client to prove that they are the correct thread in order to
/// access the underlying object.
#[verifier::external_body]
#[verifier::accept_recursive_types(V)]
tracked struct ThreadShareablePlus<V> {
    phantom: marker::PhantomData<V>,
}

#[verifier::external]
unsafe impl<V> Sync for ThreadShareablePlus<V> {}

#[verifier::external]
unsafe impl<V> Send for ThreadShareablePlus<V> {}

impl<V> ThreadShareablePlus<V> {
    pub uninterp spec fn view(&self) -> V;

    pub uninterp spec fn id(&self) -> ThreadIdPlus;

    /// Recover the inner value provided we are on the same thread.
    pub axiom fn into(tracked self, tracked is_thread: IsThreadPlus) -> (tracked res: V)
        requires
            self.id() == is_thread@,
        ensures
            res == self@,
    ;

    /// Borrow the inner value provided we are on the same thread.
    pub axiom fn borrow(tracked &self, tracked is_thread: IsThreadPlus) -> (tracked res: &V)
        requires
            self.id() == is_thread@,
        ensures
            *res == self@,
    ;
}

impl<V: Send> ThreadShareablePlus<V> {
    /// Recover the inner value.
    /// Unlike `into`, this has no thread requirement, but it does
    /// require the inner type to be `Send`.
    pub axiom fn send_into(tracked self) -> (tracked res: V)
        ensures
            res == self@,
    ;
}

impl<V: Sync> ThreadShareablePlus<V> {
    /// Borrow the inner value.
    /// Unlike `borrow`, this has no thread requirement, but it does
    /// require the inner type to be `Sync`.
    pub axiom fn sync_borrow(tracked &self) -> (tracked res: &V)
        ensures
            *res == self@,
    ;
}

} // verus!
}
