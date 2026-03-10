//! Runtime tests for standards::spec_naming_convention.

use apas_verus::standards::spec_naming_convention::spec_naming_convention::*;

#[test]
fn test_bounded_counter_increment_to_full() {
    let mut bc = LockedBoundedCounter::new(3);

    assert_eq!(bc.value(), 0);
    assert!(!bc.full());

    assert!(bc.increment().is_ok());
    assert_eq!(bc.value(), 1);

    assert!(bc.increment().is_ok());
    assert_eq!(bc.value(), 2);

    assert!(bc.increment().is_ok());
    assert_eq!(bc.value(), 3);
    assert!(bc.full());
}

#[test]
fn test_bounded_counter_err_when_full() {
    let mut bc = LockedBoundedCounter::new(1);

    assert!(bc.increment().is_ok());
    assert!(bc.full());

    assert!(bc.increment().is_err());
    assert_eq!(bc.value(), 1);
    assert!(bc.full());
}

#[test]
fn test_bounded_counter_bound_preserved() {
    let mut bc = LockedBoundedCounter::new(5);

    for _ in 0..5 {
        assert!(bc.increment().is_ok());
    }
    assert!(bc.full());
    assert!(bc.increment().is_err());
    assert_eq!(bc.value(), 5);
}
