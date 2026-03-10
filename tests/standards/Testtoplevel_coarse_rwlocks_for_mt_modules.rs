//! Runtime tests for standards::toplevel_coarse_rwlocks_for_mt_modules.

use apas_verus::standards::toplevel_coarse_rwlocks_for_mt_modules::toplevel_coarse_rwlocks_for_mt_modules::*;

#[test]
fn test_countdown_to_zero() {
    let mut cd = LockedCountDown::new(3);

    assert!(cd.count_down().is_ok());
    assert_eq!(cd.count(), 2);
    assert!(!cd.done());

    assert!(cd.count_down().is_ok());
    assert_eq!(cd.count(), 1);
    assert!(!cd.done());

    assert!(cd.count_down().is_ok());
    assert_eq!(cd.count(), 0);
    assert!(cd.done());
}

#[test]
fn test_countdown_err_at_zero() {
    let mut cd = LockedCountDown::new(1);
    assert!(cd.count_down().is_ok());
    assert!(cd.done());

    assert!(cd.count_down().is_err());
    assert_eq!(cd.count(), 0);
    assert!(cd.done());
}

#[test]
fn test_countdown_new_zero() {
    let mut cd = LockedCountDown::new(0);
    assert!(cd.done());
    assert_eq!(cd.count(), 0);
    assert!(cd.count_down().is_err());
}
