# Chapter 12 Lock-Free Exercises — Proof Holes

## Overview

Chapter 12 exercises implement lock-free concurrent data structures. Verification of these structures is intentionally limited: atomic operations (fetch_add, compare_exchange, load) have no vstd specs, and correctness depends on hardware memory ordering guarantees outside Verus's model.

## Exercise 12.1 — Spin Lock (Ticket Lock)

| Hole Type | Count | Rationale |
|-----------|-------|-----------|
| external_body | 6 | AtomicUsize (fetch_add, load) has no vstd specs. Ticket-lock correctness depends on memory ordering. |

All holes are marked with `// accept hole`. A concurrency logic (e.g., TSM) would be required for meaningful verification — disproportionate for a ticket-lock exercise.

## Exercise 12.2

Clean — no proof holes.

## Exercise 12.5 — Lock-Free Concurrent Stack (Treiber Stack)

| Hole Type | Count | Rationale |
|-----------|-------|-----------|
| unsafe {} | 4 | Raw pointer manipulation (Box::from_raw, (*head).next) for CAS-based stack. |

Uses AtomicPtr and compare-and-swap. Linearizability would require a tokenized state machine (TSM). RTTs validate runtime behavior.

## Expected State

- Exercise12_1: 6 external_body (accepted)
- Exercise12_2: clean
- Exercise12_5: 4 unsafe {} (accepted for lock-free CAS)

No plans to verify these exercises fully; documentation serves as rationale for the holes.
