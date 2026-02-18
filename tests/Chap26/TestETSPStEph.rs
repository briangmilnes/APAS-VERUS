//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for divide-and-conquer eTSP heuristic (Chapter 26).

use apas_verus::Chap26::ETSPStEph::ETSPStEph::*;

#[test]
fn test_etsp_two_points() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
    ];
    let tour = <Vec<Point>>::etsp(&points);
    assert_eq!(tour.len(), 2); // 2 points => 2 edges forming a cycle
}

#[test]
fn test_etsp_three_points() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 0.5, y: 1.0 },
    ];
    let tour = <Vec<Point>>::etsp(&points);
    assert_eq!(tour.len(), 3); // 3 points => 3 edges
}

#[test]
fn test_etsp_four_points_square() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 0.0, y: 1.0 },
    ];
    let tour = <Vec<Point>>::etsp(&points);
    assert_eq!(tour.len(), 4);
}

#[test]
fn test_etsp_collinear_points() {
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 3.0, y: 0.0 },
    ];
    let tour = <Vec<Point>>::etsp(&points);
    assert_eq!(tour.len(), 4);
}

#[test]
fn test_etsp_larger() {
    // 10 points in a circle
    let n = 10;
    let points: Vec<Point> = (0..n)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
            Point { x: angle.cos(), y: angle.sin() }
        })
        .collect();
    let tour = <Vec<Point>>::etsp(&points);
    assert_eq!(tour.len(), n);
}

#[test]
fn test_etsp_tour_length_reasonable() {
    // Square with side 1: optimal tour ~ 4.0
    let points = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.0 },
        Point { x: 1.0, y: 1.0 },
        Point { x: 0.0, y: 1.0 },
    ];
    let tour = <Vec<Point>>::etsp(&points);

    // Compute tour length
    let total_length: f64 = tour.iter().map(|e| e.from.distance(&e.to)).sum();

    // Should be close to 4.0 (optimal) but heuristic may be slightly longer
    assert!(total_length >= 4.0, "Tour length {} should be >= optimal 4.0", total_length);
    assert!(total_length <= 6.0, "Tour length {} should be reasonable (< 6.0)", total_length);
}

#[test]
fn test_etsp_20_random_points() {
    // Deterministic "random" points
    let points: Vec<Point> = (0..20)
        .map(|i| {
            let x = ((i * 17 + 3) % 100) as f64 / 100.0;
            let y = ((i * 31 + 7) % 100) as f64 / 100.0;
            Point { x, y }
        })
        .collect();
    let tour = <Vec<Point>>::etsp(&points);
    assert_eq!(tour.len(), 20);
}
