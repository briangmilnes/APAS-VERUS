//! Runtime tests for standards::rwlock_standard.

use apas_verus::standards::rwlock_standard::rwlock_standard::*;

#[test]
fn test_new_and_read() {
    let c = BoundedCounter::new(10, 0);
    let val = c.read_val();
    assert_eq!(val, 0);
}

#[test]
fn test_increment_succeeds() {
    let c = BoundedCounter::new(10, 0);
    let ok = c.try_increment();
    assert!(ok);
    let val = c.read_val();
    assert_eq!(val, 1);
}

#[test]
fn test_increment_at_max() {
    let c = BoundedCounter::new(1, 1);
    let ok = c.try_increment();
    assert!(!ok);
    let val = c.read_val();
    assert_eq!(val, 1);
}

#[test]
fn test_multiple_increments() {
    let c = BoundedCounter::new(3, 0);
    assert!(c.try_increment());
    assert!(c.try_increment());
    assert!(c.try_increment());
    assert!(!c.try_increment());
    assert_eq!(c.read_val(), 3);
}

#[test]
fn test_display() {
    let c = BoundedCounter::new(100, 0);
    assert_eq!(format!("{}", c), "BoundedCounter(max=100)");
    assert_eq!(format!("{:?}", c), "BoundedCounter(max=100)");
}
