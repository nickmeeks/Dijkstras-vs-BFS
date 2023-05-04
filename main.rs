mod graph_algos;
mod graph_reader;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Result,Write};
use crate::graph_algos::{run_random_test_bfs, DistancePair, run_random_test_dijkstras};

//for measuring runtime.
use std::time::{Duration, Instant};


///The Main Driver method
fn main() -> Result<()> {
    // Read the dataset and create an adjacency list representation of it
    let edges = graph_reader::read_edge_data(&"../musae_git_edges.txt");
    //let edges = graph_reader::read_edge_data("data_sets/test.csv");
    let adjacency_list = graph_reader::edges_to_adjacency_list(&edges);

    // display some helpful dataset statistics
    println!("Welcome to the GitHub Degrees of Separation\n");
    println!("Graph Statistics:");
    println!("  Number of edges: {}", edges.len());
    println!("  Number of vertices: {}", adjacency_list.len());

    // count is the number of nodes we will randomly choose from the graph
    let count = 200;

    // ------ BFS Algorithm ------

    println!("\n\n--> BFS Algorithm Implementation\n");

    let start_time = Instant::now();
    let shortest_dists = run_random_test_bfs(&adjacency_list, count);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;

    // print out all the distances with node pairs
    for d in &shortest_dists {
         println!("Shortest distance between {} and {} is {}", d.node_1, d.node_2, d.distance);
    }

    let (num_distances, mean_distance, std_dev) = calc_distance_stats(&shortest_dists);
    // print statistics from shortest paths search
    println!("Total pairs: {}", num_distances);
    println!("Mean Distance: {:.2}", mean_distance);
    println!("Standard Deviation: {:.3}", std_dev);
    println!("Elapsed Time: {:?}", elapsed_time);

    let output_filename = "BFS.txt";
    let algoname = "Breadth First Search Algorithm";
    create_output_file(&output_filename,
                       &algoname,
                       &edges,
                       &adjacency_list,
                       &shortest_dists,
                       num_distances,
                       mean_distance,
                       std_dev)
        .expect("Failed to write BFS output!");

    //  ------ Dijkstra's Algorithm ------

    println!("\n\n--> Dijkstra\'s Algorithm Implementation\n");

    //convert adj list to weighted adj list.
    let weighted_adj_list = graph_reader::edges_to_weighted_adjacency_list(&adjacency_list);

    //run random test using Dijkstra's Algorithm.  Same dataset will be used as for BFS above.
    let start = Instant::now();
    let dijkstras_shortest_dists = run_random_test_dijkstras(&weighted_adj_list, count);
    let end = Instant::now();
    let elapsed = end - start;

    let (dijkstras_num_distances,
         dijkstras_mean_distance,
         dijkstras_std_dev) = calc_distance_stats(&dijkstras_shortest_dists);
    println!("Total pairs: {}", dijkstras_num_distances);
    println!("Mean Distance: {:.2}", dijkstras_mean_distance);
    println!("Standard Deviation: {:.3}", dijkstras_std_dev);
    println!("Elapsed Time: {:?}", elapsed);

    //prepare and output data and distances to text file.
    let output_filename = "Dijkstras.txt";
    let algo_name = "Dijkstras Algorithm";
    create_output_file(&output_filename,
                       &algo_name,
                       &edges,
                       &adjacency_list,
                       &dijkstras_shortest_dists,
                       dijkstras_num_distances,
                       dijkstras_mean_distance,
                       dijkstras_std_dev)
        .expect("Couldn\'t write Dijkstras output.");

    //time_test(&adjacency_list);

    Ok(())
}


// this function runs multiple trials for 10 - 100 randomly chosen nodes (different nodes for each trial)
// It calculates the time for each trial and records that in a hash map.  That data was output and
// copied into an Excel spreadsheet for plotting.
fn time_test(adj_list: &HashMap<u32, Vec<u32>>) {

    // sizes contains the different sized datasets we are gathering data about.
    let sizes = vec![10, 100];

    // ----------------------------   BFS    ----------------------------
    println!("BFS Timings");
    let mut bfs_timings: HashMap<i32, f64> = HashMap::new();
    for cur_size in &sizes {
        println!("    Processing size {}", cur_size);
        let mut total_time = Duration::from_secs(0);
        for _iteration in 0..5 {
            let start = Instant::now();
            let _bfs_shortest_dists = run_random_test_bfs(adj_list, *cur_size as usize);
            let end = Instant::now();
            let elapsed = end - start;
            total_time += elapsed;
        }
        println!("       Avg time for 5 iterations: {:?}", total_time.as_secs_f64()/5.0);
        bfs_timings.insert(*cur_size, total_time.as_secs_f64() / 5.0);
    }
    println!("  Timings:");
    for (k, v) in bfs_timings.iter() {
        println!("    Size = {}; avg time = {}", k, v);
    }
    // ---------------------------- Dijkstras ----------------------------
    println!("Dijkstra\'s Timings");

    let mut dijkstras_timings: HashMap<i32, f64> = HashMap::new();
    let weighted_adj_list = graph_reader::edges_to_weighted_adjacency_list(&adj_list);
    for cur_size in &sizes {
        println!("    Processing size {}", cur_size);
        let mut total_time = Duration::from_secs(0);
        for _iteration in 0..5 {
            let start = Instant::now();
            let _dijkstras_shortest_dists = run_random_test_dijkstras(&weighted_adj_list, *cur_size as usize);
            let end = Instant::now();
            let elapsed = end - start;
            total_time += elapsed;
        }
        println!("       Avg time for 5 iterations: {:?}", total_time.as_secs_f64()/5.0);
        dijkstras_timings.insert(*cur_size, total_time.as_secs_f64() / 5.0);
    }

    println!("  Timings:");
    for (k, v) in dijkstras_timings.iter() {
        println!("  size = {}; avg time = {}", k, v);
    }

}

// Calculates n, mean, and std dev of a collection of distance structures
fn calc_distance_stats(shortest_dists: &Vec<DistancePair>) -> (usize, f64, f64) {
    let num_distances = shortest_dists.len();
    assert!(num_distances > 0, "Number of distances is invalid");

    // sum all distances and calculate mean.
    let mut total_distance = 0;
    for i in shortest_dists {
        let dist = i.distance;
        total_distance += dist;
    }
    let algo_mean_of_differences = total_distance as f64 / num_distances as f64;

    //sum squares of differences, then calculate variance followed by std. dev.
    let mut sum_of_squares: f64 = 0.0;
    for i in shortest_dists {
        sum_of_squares += (i.distance as f64 - algo_mean_of_differences).powi(2);
    }
    let variance = sum_of_squares / num_distances as f64;
    let algo_standard_deviation = variance.sqrt();

    (
        num_distances,
        algo_mean_of_differences,
        algo_standard_deviation,
    )
}

///Testing the different steps of the project to ensure correctness
///Using a graph with 4 nodes and 5 edges.
///
///      1 ----- 2 ----- 3
///       \      |      /
///        \     |     /
///         \    |    /
///          \   |   /
///           \  |  /
///            \ | /
///              4
#[test]
fn test_algorithm() {
    let test_file = "/Users/nickmeeks/Downloads/210project/data_sets/test.csv";

    //read edge data
    let edges = graph_reader::read_edge_data(test_file);

    //confirm number of edges is 5
    assert_eq!(edges.len(), 5, "Read too many edges");

    //convert edge data to adjacency list
    let adjacency_list = graph_reader::edges_to_adjacency_list(&edges);

    //confirm 4 vertices
    assert_eq!(adjacency_list.len(), 4, "Wrong number of nodes read.");

    //run random test on all 4 vertices
    let sample_size = 4;
    // let shortest_dists = run_random_test(&adjacency_list, sample_size);

    let shortest_dists_v2 = run_random_test_bfs(&adjacency_list, sample_size);
    let v2_len = shortest_dists_v2.len();

    //confirm that there are 6 distances for the test graph (4C2)
    assert_eq!(v2_len, 6, "Number of distances in test graph is incorrect");

    //confirm that the mean of the differences is within 0.01 of hand-calculated mean.
    let mut total_distance = 0;
    for i in shortest_dists_v2 {
        let dist = i.distance;
        total_distance += dist;
    }
    let algo_mean_of_differences = total_distance as f64 / v2_len as f64;
    let actual_mean_of_differences: f64 = 1.1667;
    let epsilon = algo_mean_of_differences - actual_mean_of_differences;
    assert!(
        epsilon.abs() < 0.01,
        "Mean of distances in test graph is incorrect!"
    );
}


// Helper function to write output of distances between nodes in
// sample to a txt file.
fn create_output_file(filename: &str,
                      algoname: &str,
                      edges: &Vec<(u32, u32)>,
                      adjacency_list: &HashMap<u32, Vec<u32>>,
                      shortest_dists: &Vec<DistancePair>,
                      num_distances: usize,
                      mean_distance: f64,
                      std_dev: f64) -> Result<()> {
    // Create an file containing all the distances.
    let mut file = File::create(filename)?;
    writeln!(file, "Algorithm: {}\n", algoname)?;

    writeln!(file, "Graph Statistics:")?;
    writeln!(file, "  Number of edges: {}", edges.len())?;
    writeln!(file, "  Number of vertices: {}\n", adjacency_list.len())?;

    writeln!(file, "Run Statistics:")?;
    writeln!(file, "  Number of distances computed: {}", num_distances)?;
    writeln!(file, "  Mean distance: {:.2}", mean_distance)?;
    writeln!(file, "  Std. Dev of distances: {:.3}\n", std_dev)?;

    writeln!(file, "\n------ All Shortest Distances ------")?;
    for d in shortest_dists {
        writeln!(file, "Shortest distance between {} and {} is {}", d.node_1, d.node_2, d.distance)?;
    }
    drop(file);
    Ok(())
}

// Helper function to print the weighted edge list returned from
// converting unweighted adj list to weighted adj list.
fn print_weighted_edge_list(weighted_adj_list: &HashMap<u32, Vec<(u32, u32)>>) {
    for (node, adj_with_weights) in weighted_adj_list {
        print!("{} : ", node);
        for (adj_node, weight) in adj_with_weights {
            print!("({}, {})", adj_node, weight);
        }
        println!();
    }
}

