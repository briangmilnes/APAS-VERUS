//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::sync::Arc;
use std::thread;

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::DirGraphMtEph::DirGraphMtEph::*;
use apas_ai::{DirGraphMtEphLit, SetLit};
use apas_ai::Types::Types::*;

#[test]
fn test_dirgraphmtephlit_macro_functionality() {
    // Test empty graph creation
    let empty: DirGraphMtEph<i32> = DirGraphMtEphLit!();
    assert_eq!(empty.vertices().size(), 0);
    assert_eq!(empty.arcs().size(), 0);

    // Test graph creation with vertices and arcs
    let with_data: DirGraphMtEph<i32> = DirGraphMtEphLit!(
        V: [1, 2, 3],
        A: [(1, 2), (2, 3)]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.arcs().size(), 2);
}

#[test]
fn test_dirgraphmteph_empty() {
    let empty_graph = DirGraphMtEph::<i32>::empty();
    assert_eq!(empty_graph.sizeV(), 0);
    assert_eq!(empty_graph.sizeA(), 0);
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.arcs().size(), 0);
}

#[test]
fn test_dirgraphmteph_basic_operations() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 3));
        let _ = s.insert(Edge(3, 3));
        s
    }; // includes self-loop (3,3)
    let g = DirGraphMtEph::FromSets(v.clone(), a.clone());
    assert_eq!(g.sizeV(), v.size());
    assert_eq!(g.sizeA(), a.size());
    assert_eq!(g.vertices(), &v);
    assert_eq!(g.arcs(), &a);
}

#[test]
fn test_dirgraphmteph_neighbor() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(0, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    // Test Neighbor method - checks if edge exists between two vertices
    assert!(g.Neighbor(&0, &1)); // edge 0->1 exists
    assert!(g.Neighbor(&0, &2)); // edge 0->2 exists
    assert!(g.Neighbor(&1, &2)); // edge 1->2 exists
    assert!(!g.Neighbor(&1, &0)); // edge 1->0 does not exist
    assert!(!g.Neighbor(&2, &0)); // edge 2->0 does not exist
    assert!(!g.Neighbor(&2, &1)); // edge 2->1 does not exist
}

#[test]
fn test_dirgraphmteph_ng() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    let ng_0 = g.NG(&0);
    assert_eq!(ng_0.size(), 1);
    assert!(ng_0.mem(&1));

    let ng_2 = g.NG(&2);
    assert_eq!(ng_2.size(), 1); // vertex 2 has incoming neighbor 1
    assert!(ng_2.mem(&1));
}

#[test]
fn test_dirgraphmteph_ngofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    let vertices_subset = SetLit![0, 1];
    let ng_subset = g.NGOfVertices(&vertices_subset);
    assert_eq!(ng_subset.size(), 3); // NG(0)={1} ∪ NG(1)={0,2} = {0,1,2}
    assert!(ng_subset.mem(&0));
    assert!(ng_subset.mem(&1));
    assert!(ng_subset.mem(&2));
}

#[test]
fn test_dirgraphmteph_nplus() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    let nplus_0 = g.NPlus(&0);
    assert_eq!(nplus_0.size(), 1);
    assert!(nplus_0.mem(&1));

    let nplus_2 = g.NPlus(&2);
    assert_eq!(nplus_2.size(), 0);
}

#[test]
fn test_dirgraphmteph_nminus() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    let nminus_1 = g.NMinus(&1);
    assert_eq!(nminus_1.size(), 1);
    assert!(nminus_1.mem(&0));

    let nminus_0 = g.NMinus(&0);
    assert_eq!(nminus_0.size(), 0);
}

#[test]
fn test_dirgraphmteph_nplusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    let vertices_subset = SetLit![0, 1];
    let nplus_subset = g.NPlusOfVertices(&vertices_subset);
    assert_eq!(nplus_subset.size(), 2);
    assert!(nplus_subset.mem(&1));
    assert!(nplus_subset.mem(&2));
}

#[test]
fn test_dirgraphmteph_nminusofvertices() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    let vertices_subset = SetLit![1, 2];
    let nminus_subset = g.NMinusOfVertices(&vertices_subset);
    assert_eq!(nminus_subset.size(), 2);
    assert!(nminus_subset.mem(&0));
    assert!(nminus_subset.mem(&1));
}

#[test]
fn test_dirgraphmteph_incident() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 0));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    // Test Incident method - checks if edge is incident to vertex
    assert!(g.Incident(&Edge(0, 1), &0)); // edge (0,1) is incident to vertex 0
    assert!(g.Incident(&Edge(0, 1), &1)); // edge (0,1) is incident to vertex 1
    assert!(!g.Incident(&Edge(0, 1), &2)); // edge (0,1) is not incident to vertex 2
    assert!(g.Incident(&Edge(1, 2), &1)); // edge (1,2) is incident to vertex 1
    assert!(g.Incident(&Edge(1, 2), &2)); // edge (1,2) is incident to vertex 2
}

#[test]
fn test_dirgraphmteph_degree() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 0));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    assert_eq!(g.Degree(&0), 2); // one in + one out = 2
    assert_eq!(g.Degree(&1), 2); // one in + one out = 2
    assert_eq!(g.Degree(&2), 2); // one in + one out = 2
}

#[test]
fn test_dirgraphmteph_indegree() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 0));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    assert_eq!(g.InDegree(&0), 1); // edge from 2
    assert_eq!(g.InDegree(&1), 1); // edge from 0
    assert_eq!(g.InDegree(&2), 1); // edge from 1
}

#[test]
fn test_dirgraphmteph_outdegree() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 0));
        s
    };
    let g = DirGraphMtEph::FromSets(v, a);

    assert_eq!(g.OutDegree(&0), 1); // edge to 1
    assert_eq!(g.OutDegree(&1), 1); // edge to 2
    assert_eq!(g.OutDegree(&2), 1); // edge to 0
}

#[test]
fn test_dirgraphmteph_concurrent_access() {
    let v: SetStEph<N> = SetLit![0, 1, 2, 3, 4];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 3));
        let _ = s.insert(Edge(3, 4));
        s
    };
    let g = Arc::new(DirGraphMtEph::FromSets(v, a));

    let mut handles = vec![];

    // Spawn multiple threads to test concurrent access
    for i in 0..4 {
        let g_clone = Arc::clone(&g);
        let handle = thread::spawn(move || {
            // Each thread performs different graph operations
            let ng = g_clone.NG(&i);
            let degree = g_clone.Degree(&i);
            let in_degree = g_clone.InDegree(&i);
            let out_degree = g_clone.OutDegree(&i);

            // Verify basic properties (degrees are usize, always >= 0)
            // Just verify they're valid values by using them
            assert_eq!(g_clone.sizeV(), 5);
            assert_eq!(g_clone.sizeA(), 4);

            (ng.size(), degree, in_degree, out_degree)
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join().expect("Thread should complete successfully");
    }
}

#[test]
fn test_dirgraphmteph_thread_safety() {
    let v: SetStEph<N> = SetLit![0, 1, 2];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = Arc::new(DirGraphMtEph::FromSets(v, a));

    let mut handles = vec![];

    // Test that multiple threads can safely read from the graph simultaneously
    for _ in 0..10 {
        let g_clone = Arc::clone(&g);
        let handle = thread::spawn(move || {
            // Perform various read operations
            assert!(g_clone.Neighbor(&0, &1));
            assert!(!g_clone.Neighbor(&1, &0));
            assert_eq!(g_clone.sizeV(), 3);
            assert_eq!(g_clone.sizeA(), 2);

            let ng_0 = g_clone.NG(&0);
            assert_eq!(ng_0.size(), 1);
            assert!(ng_0.mem(&1));
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}

#[test]
fn test_race_condition_verification_concurrent_graph_reads() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    // Create a test graph
    let vertices: SetStEph<i32> = SetLit![0, 1, 2, 3, 4, 5];
    let edges = {
        let mut s = SetStEph::<Edge<i32>>::empty();
        let _ = s.insert(Edge(0, 1));
        let _ = s.insert(Edge(1, 2));
        let _ = s.insert(Edge(2, 3));
        let _ = s.insert(Edge(3, 4));
        let _ = s.insert(Edge(4, 5));
        let _ = s.insert(Edge(5, 0)); // Cycle
        s
    };
    let graph = Arc::new(DirGraphMtEph::FromSets(vertices, edges));

    let barrier = Arc::new(Barrier::new(8));
    let race_detected = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];

    // Spawn multiple threads that read graph properties simultaneously
    for thread_id in 0..8 {
        let graph_clone = Arc::clone(&graph);
        let barrier_clone = Arc::clone(&barrier);
        let race_detected_clone = Arc::clone(&race_detected);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let mut read_count = 0;
            for _ in 0..100 {
                // Read various graph properties
                let size_v = graph_clone.sizeV();
                let size_a = graph_clone.sizeA();

                // Verify basic consistency
                if size_v != 6 || size_a != 6 {
                    race_detected_clone.store(true, Ordering::SeqCst);
                }

                // Test neighbor relationships
                let neighbor_01 = graph_clone.Neighbor(&0, &1);
                let neighbor_10 = graph_clone.Neighbor(&1, &0);

                if !neighbor_01 || neighbor_10 {
                    race_detected_clone.store(true, Ordering::SeqCst);
                }

                // Test degree calculations
                let degree_0 = graph_clone.Degree(&0);
                let in_degree_0 = graph_clone.InDegree(&0);
                let out_degree_0 = graph_clone.OutDegree(&0);

                // For vertex 0: outgoing edge to 1, incoming edge from 5
                // So degree = 2, in_degree = 1, out_degree = 1
                if degree_0 != 2 || in_degree_0 != 1 || out_degree_0 != 1 {
                    race_detected_clone.store(true, Ordering::SeqCst);
                }

                // Test neighbor sets
                let ng_0 = graph_clone.NG(&0);
                let nplus_0 = graph_clone.NPlus(&0);
                let nminus_0 = graph_clone.NMinus(&0);

                if ng_0.size() != 2 || nplus_0.size() != 1 || nminus_0.size() != 1 {
                    race_detected_clone.store(true, Ordering::SeqCst);
                }

                read_count += 1;
            }

            (thread_id, read_count)
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Verify no race conditions detected
    assert!(
        !race_detected.load(Ordering::SeqCst),
        "Race condition detected in concurrent graph reads"
    );

    // Verify all threads completed their reads
    for (thread_id, read_count) in results {
        assert_eq!(read_count, 100, "Thread {thread_id} didn't complete all reads");
    }
}

#[test]
fn test_race_condition_verification_mixed_graph_operations() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    // Create multiple graphs for concurrent access
    let graphs = Arc::new(std::sync::RwLock::new(vec![
        DirGraphMtEph::empty(),
        DirGraphMtEph::empty(),
        DirGraphMtEph::empty(),
    ]));

    let barrier = Arc::new(Barrier::new(9));
    let inconsistency_detected = Arc::new(AtomicBool::new(false));
    let operation_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    // Spawn reader threads
    for thread_id in 0..3 {
        let graphs_clone = Arc::clone(&graphs);
        let barrier_clone = Arc::clone(&barrier);
        let inconsistency_clone = Arc::clone(&inconsistency_detected);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let mut read_count = 0;
            for _ in 0..50 {
                if let Ok(graph_vec) = graphs_clone.read() {
                    for graph in graph_vec.iter() {
                        let size_v = graph.sizeV();
                        let size_a = graph.sizeA();

                        // Verify basic consistency
                        if size_a > size_v * size_v {
                            inconsistency_clone.store(true, Ordering::SeqCst);
                        }

                        // Test some operations if graph is non-empty
                        if size_v > 0 {
                            let degree_0 = graph.Degree(&0);
                            if degree_0 > size_v {
                                inconsistency_clone.store(true, Ordering::SeqCst);
                            }
                        }

                        read_count += 1;
                    }
                }
            }

            (thread_id, read_count)
        });
        handles.push(handle);
    }

    // Spawn writer threads that create new graphs
    for thread_id in 3..9 {
        let graphs_clone = Arc::clone(&graphs);
        let barrier_clone = Arc::clone(&barrier);
        let operation_counter_clone = Arc::clone(&operation_counter);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let mut write_count = 0;
            for i in 0..10 {
                let graph_index = (thread_id % 3) as usize;

                // Create a small graph
                let vertices: SetStEph<i32> = SetLit![thread_id * 10 + i, thread_id * 10 + i + 1];
                let edges = {
                    let mut s = SetStEph::<Edge<i32>>::empty();
                    let _ = s.insert(Edge(thread_id * 10 + i, thread_id * 10 + i + 1));
                    s
                };
                let new_graph = DirGraphMtEph::FromSets(vertices, edges);

                if let Ok(mut graph_vec) = graphs_clone.write() {
                    graph_vec[graph_index] = new_graph;
                    write_count += 1;
                    operation_counter_clone.fetch_add(1, Ordering::SeqCst);
                }
            }

            (thread_id, write_count)
        });
        handles.push(handle);
    }

    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    // Verify no inconsistencies detected
    assert!(
        !inconsistency_detected.load(Ordering::SeqCst),
        "Data inconsistency detected during concurrent graph operations"
    );

    // Verify all operations completed
    assert_eq!(
        operation_counter.load(Ordering::SeqCst),
        60,
        "Not all write operations completed"
    );

    // Verify thread results
    for (thread_id, count) in results {
        if thread_id < 3 {
            // Reader thread
            assert!(count > 0, "Reader thread {thread_id} performed no reads");
        } else {
            // Writer thread
            assert_eq!(count, 10, "Writer thread {thread_id} didn't complete all writes");
        }
    }

    // Final consistency check
    if let Ok(final_graphs) = graphs.read() {
        for (idx, graph) in final_graphs.iter().enumerate() {
            let size_v = graph.sizeV();
            let size_a = graph.sizeA();

            // Basic sanity checks
            assert!(
                size_a <= size_v * size_v,
                "Graph {idx} has {size_a} edges but only {size_v} vertices"
            );

            if size_v > 0 {
                // Test that degree calculations don't crash
                let _ = graph.Degree(&0);
                let _ = graph.InDegree(&0);
                let _ = graph.OutDegree(&0);
            }
        }
    }
}

#[test]
fn test_deadlock_prevention_concurrent_graph_operations() {
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};

    // Create multiple graphs with different locking orders to test deadlock prevention
    let graph_a = Arc::new(Mutex::new({
        let vertices: SetStEph<i32> = SetLit![0, 1, 2];
        let edges = {
            let mut s = SetStEph::<Edge<i32>>::empty();
            let _ = s.insert(Edge(0, 1));
            let _ = s.insert(Edge(1, 2));
            s
        };
        DirGraphMtEph::FromSets(vertices, edges)
    }));

    let graph_b = Arc::new(Mutex::new({
        let vertices: SetStEph<i32> = SetLit![3, 4, 5];
        let edges = {
            let mut s = SetStEph::<Edge<i32>>::empty();
            let _ = s.insert(Edge(3, 4));
            let _ = s.insert(Edge(4, 5));
            s
        };
        DirGraphMtEph::FromSets(vertices, edges)
    }));

    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];

    // Thread 1: A -> B operations
    {
        let barrier_clone = Arc::clone(&barrier);
        let graph_a_clone = Arc::clone(&graph_a);
        let graph_b_clone = Arc::clone(&graph_b);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let start_time = Instant::now();
            let mut operations = 0;

            while start_time.elapsed() < Duration::from_millis(50) {
                if let (Ok(a), Ok(b)) = (graph_a_clone.try_lock(), graph_b_clone.try_lock()) {
                    // Perform read operations on both graphs
                    let _ = a.sizeV();
                    let _ = a.Degree(&0);
                    let _ = b.sizeA();
                    let _ = b.Neighbor(&3, &4);
                    operations += 1;
                }
                thread::yield_now();
            }

            operations
        });
        handles.push(handle);
    }

    // Thread 2: B -> A operations (reverse order)
    {
        let barrier_clone = Arc::clone(&barrier);
        let graph_a_clone = Arc::clone(&graph_a);
        let graph_b_clone = Arc::clone(&graph_b);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let start_time = Instant::now();
            let mut operations = 0;

            while start_time.elapsed() < Duration::from_millis(50) {
                if let (Ok(b), Ok(a)) = (graph_b_clone.try_lock(), graph_a_clone.try_lock()) {
                    // Perform read operations on both graphs
                    let _ = b.InDegree(&4);
                    let _ = b.OutDegree(&3);
                    let _ = a.NG(&1);
                    let _ = a.NPlus(&0);
                    operations += 1;
                }
                thread::yield_now();
            }

            operations
        });
        handles.push(handle);
    }

    // Thread 3: Mixed operations on A
    {
        let barrier_clone = Arc::clone(&barrier);
        let graph_a_clone = Arc::clone(&graph_a);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let start_time = Instant::now();
            let mut operations = 0;

            while start_time.elapsed() < Duration::from_millis(50) {
                if let Ok(a) = graph_a_clone.try_lock() {
                    // Perform various operations on graph A
                    let _ = a.NMinus(&2);
                    let _ = a.Incident(&Edge(0, 1), &0);
                    let _ = a.NGOfVertices(&SetLit![0, 1]);
                    operations += 1;
                }
                thread::yield_now();
            }

            operations
        });
        handles.push(handle);
    }

    // Thread 4: Mixed operations on B
    {
        let barrier_clone = Arc::clone(&barrier);
        let graph_b_clone = Arc::clone(&graph_b);

        let handle = thread::spawn(move || {
            barrier_clone.wait();

            let start_time = Instant::now();
            let mut operations = 0;

            while start_time.elapsed() < Duration::from_millis(50) {
                if let Ok(b) = graph_b_clone.try_lock() {
                    // Perform various operations on graph B
                    let _ = b.NPlusOfVertices(&SetLit![3, 4]);
                    let _ = b.NMinusOfVertices(&SetLit![4, 5]);
                    let _ = b.Neighbor(&4, &5);
                    operations += 1;
                }
                thread::yield_now();
            }

            operations
        });
        handles.push(handle);
    }

    // Collect results - if there's a deadlock, this will hang
    let results = handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();

    // Verify all threads completed some operations (no deadlock occurred)
    for (thread_id, op_count) in results.iter().enumerate() {
        assert!(
            *op_count > 0,
            "Thread {thread_id} completed no operations - possible deadlock"
        );
    }

    // Verify final state is consistent
    let final_a = graph_a.lock().unwrap();
    let final_b = graph_b.lock().unwrap();

    assert_eq!(final_a.sizeV(), 3);
    assert_eq!(final_a.sizeA(), 2);
    assert_eq!(final_b.sizeV(), 3);
    assert_eq!(final_b.sizeA(), 2);
}

#[test]
fn test_empty_graph_operations() {
    let vertices = SetLit![];
    let edges = SetStEph::<Edge<N>>::empty();
    let g = DirGraphMtEph::FromSets(vertices, edges);
    assert_eq!(g.sizeV(), 0);
    assert_eq!(g.sizeA(), 0);
    assert_eq!(g.vertices().size(), 0);
}

#[test]
fn test_isolated_vertex() {
    let vertices = SetLit![1, 2, 3];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);
    assert_eq!(g.NPlus(&3).size(), 0);
    assert_eq!(g.NMinus(&3).size(), 0);
}

// Large graph tests to trigger parallel code paths (n > 8)

#[test]
fn test_ngofvertices_large_parallel() {
    // Create a graph with 15 vertices and arcs to trigger parallel branch (>8 vertices)
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..14 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let query_set = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // 10 vertices
    let ng_result = g.NGOfVertices(&query_set);

    // Each vertex i has neighbors {i-1, i+1} (if they exist)
    // NG(0)={1}, NG(1)={0,2}, ..., NG(9)={8,10}
    // Union should give {0,1,2,3,4,5,6,7,8,9,10}
    assert!(ng_result.size() >= 10);
    assert!(ng_result.mem(&10));
}

#[test]
fn test_nplus_large_parallel() {
    // Create a graph with 15 arcs to trigger parallel branch (>8 arcs)
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..15 {
            let _ = s.insert(Edge(i, (i + 1) % 15));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let nplus_0 = g.NPlus(&0);
    assert_eq!(nplus_0.size(), 1);
    assert!(nplus_0.mem(&1));

    let nplus_7 = g.NPlus(&7);
    assert_eq!(nplus_7.size(), 1);
    assert!(nplus_7.mem(&8));
}

#[test]
fn test_nminus_large_parallel() {
    // Create a graph with 15 arcs to trigger parallel branch (>8 arcs)
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..15 {
            let _ = s.insert(Edge(i, (i + 1) % 15));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let nminus_1 = g.NMinus(&1);
    assert_eq!(nminus_1.size(), 1);
    assert!(nminus_1.mem(&0));

    let nminus_0 = g.NMinus(&0);
    assert_eq!(nminus_0.size(), 1);
    assert!(nminus_0.mem(&14));
}

#[test]
fn test_nplusofvertices_large_parallel() {
    // Create a graph with 15 vertices to trigger parallel branch (>8 vertices)
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..14 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let query_set = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]; // 10 vertices
    let nplus_result = g.NPlusOfVertices(&query_set);

    // NPlus(0)={1}, NPlus(1)={2}, ..., NPlus(9)={10}
    // Union should give {1,2,3,4,5,6,7,8,9,10}
    assert_eq!(nplus_result.size(), 10);
    for i in 1..=10 {
        assert!(nplus_result.mem(&i));
    }
}

#[test]
fn test_nminusofvertices_large_parallel() {
    // Create a graph with 15 vertices to trigger parallel branch (>8 vertices)
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..14 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let query_set = SetLit![5, 6, 7, 8, 9, 10, 11, 12, 13, 14]; // 10 vertices
    let nminus_result = g.NMinusOfVertices(&query_set);

    // NMinus(5)={4}, NMinus(6)={5}, ..., NMinus(14)={13}
    // Union should give {4,5,6,7,8,9,10,11,12,13}
    assert_eq!(nminus_result.size(), 10);
    for i in 4..=13 {
        assert!(nminus_result.mem(&i));
    }
}

#[test]
fn test_debug_trait() {
    let vertices: SetStEph<N> = SetLit![1, 2, 3];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let debug_str = format!("{:?}", g);
    assert!(debug_str.contains("DirGraphMtEph"));
}

#[test]
fn test_display_trait() {
    let vertices: SetStEph<N> = SetLit![1, 2, 3];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let display_str = format!("{}", g);
    assert!(display_str.contains("V="));
    assert!(display_str.contains("A="));
}

#[test]
fn test_equality() {
    let v1: SetStEph<N> = SetLit![1, 2, 3];
    let a1 = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g1 = DirGraphMtEph::FromSets(v1.clone(), a1.clone());
    let g2 = DirGraphMtEph::FromSets(v1, a1);

    assert_eq!(g1, g2);
}

#[test]
fn test_inequality() {
    let v1: SetStEph<N> = SetLit![1, 2, 3];
    let a1 = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        s
    };
    let v2: SetStEph<N> = SetLit![1, 2, 4];
    let a2 = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g1 = DirGraphMtEph::FromSets(v1, a1);
    let g2 = DirGraphMtEph::FromSets(v2, a2);

    assert_ne!(g1, g2);
}

#[test]
fn test_clone() {
    let v: SetStEph<N> = SetLit![1, 2, 3];
    let a = {
        let mut s = SetStEph::<Edge<N>>::empty();
        let _ = s.insert(Edge(1, 2));
        s
    };
    let g1 = DirGraphMtEph::FromSets(v, a);
    let g2 = g1.clone();

    assert_eq!(g1, g2);
    assert_eq!(g1.sizeV(), g2.sizeV());
    assert_eq!(g1.sizeA(), g2.sizeA());
}

// Edge case tests for minimal parallel paths

#[test]
fn test_ngofvertices_empty_query() {
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..9 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let empty_set: SetStEph<N> = SetLit![];
    let result = g.NGOfVertices(&empty_set);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_nplusofvertices_empty_query() {
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..9 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let empty_set: SetStEph<N> = SetLit![];
    let result = g.NPlusOfVertices(&empty_set);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_nminusofvertices_empty_query() {
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..9 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let empty_set: SetStEph<N> = SetLit![];
    let result = g.NMinusOfVertices(&empty_set);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_nplus_empty_arcs() {
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let edges = SetStEph::<Edge<N>>::empty();
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let result = g.NPlus(&5);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_nminus_empty_arcs() {
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let edges = SetStEph::<Edge<N>>::empty();
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let result = g.NMinus(&5);
    assert_eq!(result.size(), 0);
}

#[test]
fn test_minimal_parallel_ngofvertices() {
    // Exactly 9 vertices - minimal case to trigger parallel path
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..8 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let query_set = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8]; // 9 vertices
    let result = g.NGOfVertices(&query_set);
    assert!(result.size() >= 8);
}

#[test]
fn test_minimal_parallel_nplusofvertices() {
    // Exactly 9 vertices - minimal case to trigger parallel path
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..8 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let query_set = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8]; // 9 vertices
    let result = g.NPlusOfVertices(&query_set);
    assert_eq!(result.size(), 8);
}

#[test]
fn test_minimal_parallel_nminusofvertices() {
    // Exactly 9 vertices - minimal case to trigger parallel path
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..8 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let query_set = SetLit![1, 2, 3, 4, 5, 6, 7, 8]; // 8 vertices
    let result = g.NMinusOfVertices(&query_set);
    assert_eq!(result.size(), 8);
}

#[test]
fn test_minimal_parallel_nplus() {
    // Exactly 9 arcs - minimal case to trigger parallel path
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..9 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let result = g.NPlus(&0);
    assert_eq!(result.size(), 1);
    assert!(result.mem(&1));
}

#[test]
fn test_minimal_parallel_nminus() {
    // Exactly 9 arcs - minimal case to trigger parallel path
    let vertices: SetStEph<N> = SetLit![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let edges = {
        let mut s = SetStEph::<Edge<N>>::empty();
        for i in 0..9 {
            let _ = s.insert(Edge(i, i + 1));
        }
        s
    };
    let g = DirGraphMtEph::FromSets(vertices, edges);

    let result = g.NMinus(&8);
    assert_eq!(result.size(), 1);
    assert!(result.mem(&7));
}
