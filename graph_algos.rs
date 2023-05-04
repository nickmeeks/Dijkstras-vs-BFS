use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BinaryHeap, HashMap, VecDeque};


//                         Breadth First Search
// ------------------------------------------------------------------------------------------------

// DistancePair represents two developers from Github and the shortest
// distance between them in the graph.
pub struct DistancePair {
    pub node_1: u32,
    pub node_2: u32,
    pub distance: u32,
}

// This function will randomly generate a collection of vertices of size num_vertices
// and find the shortest distance between all pairs of that collection.
pub fn run_random_test_bfs(adjacency_list: &HashMap<u32, Vec<u32>>, num_vertices: usize) -> Vec<DistancePair> {
    assert!(
        num_vertices <= adjacency_list.len(),
        "Requested sample is bigger than graph"
    );

    // let mut shortest_dists: HashMap<(u32, u32), u32> = HashMap::new();
    let mut test_shortest_distances: Vec<DistancePair> = Vec::new();

    //randomly choose 100 vertices
    let mut rng = thread_rng();
    //make a copy of the keys of the adjacency list
    let mut vertices: Vec<u32> = adjacency_list.keys().cloned().collect();
    //shuffle the copy of the keys (used to simulate randomly choosing x nodes)
    vertices.shuffle(&mut rng);
    //save the last num_vertices elements of the vector
    let chosen_vertices = &vertices[..num_vertices];

    // outer loop proceeds over all nodes
    for i in 0..num_vertices {
        let start_node = chosen_vertices[i];
        let dists = breadth_first_search(adjacency_list, start_node);
        // inner loop proceeds for all nodes in the list after location i.
        for j in (i + 1)..num_vertices {
            let end_node = chosen_vertices[j];
            let dist = dists[&end_node] as u32;
            // shortest_dists.insert((start_node, end_node), dist);
            let current_pair = DistancePair {
                node_1: start_node,
                node_2: end_node,
                distance: dist,
            };
            test_shortest_distances.push(current_pair);
        }
    }

    test_shortest_distances
}

// Helper function for BFS
// Initializes a hashmap with all the vertices in the graph and their distances set to MAX int
// in preparation for BFS algorithm
fn init_hashmap(adjacency_list: &HashMap<u32, Vec<u32>>, start_node: u32) -> HashMap<u32, u32> {
    let mut dists: HashMap<u32, u32> = HashMap::new();
    for node in adjacency_list.keys() {
        dists.insert(*node, u32::MAX);
    }

    // start the process with the start_node with distance 0.
    dists.insert(start_node, 0);
    dists
}


// Standard BFS graph search algorithm.  Starts with a particular node and returns a Map
// holding the shortest distance to all other nodes in the graph.
pub fn breadth_first_search(adjacency_list: &HashMap<u32, Vec<u32>>, start_node: u32) -> HashMap<u32, u32> {
    let mut dists = init_hashmap(adjacency_list, start_node);

    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_back(start_node);

    // standard bfs search using a queue
    while let Some(curr_node) = queue.pop_front() {
        for neighbor in adjacency_list.get(&curr_node).unwrap_or(&vec![]) {
            let new_dist = dists[&curr_node] + 1;
            if new_dist < dists[neighbor] {
                dists.insert(*neighbor, new_dist);
                queue.push_back(*neighbor);
            }
        }
    }

    dists
}




//         Dijkstra's Algorithm
//
// ------------------------------------------------------------------------------------------------

// NodeCost is used in the p-queue.
#[derive(Eq, PartialEq)]
struct NodeCost(u32, u32);

// Dijkstra's algorithm uses a min-ordered priority queue.  This means that it needs to pop out
// the smallest value from the priority queue.  Since we are using a binary heap, we need to reverse the
// ordering so that it will be a min-heap since a regular binary heap is a max-heap.
impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


// Implementation of Dijkstra's algorithm - a single-source shortest path algorithm.
// My implementation uses a Binary Heap for the priority queue with values reversed so it is a
// min-heap.
fn dijkstras(adj_list: &HashMap<u32, Vec<(u32, u32)>>, start: u32) -> HashMap<u32, u32> {

    //create a hashmap of distances for all nodes in the graph.  Will be updated by
    //the algorithm as we discover new nodes
    let mut dists: HashMap<u32, u32> = adj_list.keys().map(|node| (*node, u32::MAX)).collect();

    // a Binary Heap is used for the priority queue
    let mut p_queue = BinaryHeap::new();

    dists.insert(start, 0);
    p_queue.push(NodeCost(start, 0));

    while let Some(NodeCost(curr_node, curr_cost)) = p_queue.pop() {
        if curr_cost > dists[&curr_node] {
            continue;
        }
        for (neighbor, weight) in adj_list.get(&curr_node).unwrap_or(&vec![]) {
            let new_cost = curr_cost + weight;
            if new_cost < dists[neighbor] {
                dists.insert(*neighbor, new_cost);
                p_queue.push(NodeCost(*neighbor, new_cost));
            }
        }
    }
    dists
}

// This function will randomly generate a collection of vertices of size num_vertices
// and find the shortest distance between all pairs of that collection.
pub fn run_random_test_dijkstras(
    adjacency_list: &HashMap<u32, Vec<(u32, u32)>>,
    num_vertices: usize,
) -> Vec<DistancePair> {
    assert!(
        num_vertices <= adjacency_list.len(),
        "Requested sample is bigger than graph"
    );

    //create a vector to store calculated distances
    let mut dijkstras_shortest_dists: Vec<DistancePair> = Vec::new();

    //randomly choose 100 vertices
    let mut rng = thread_rng();
    //make a copy of the keys of the adjacency list
    let mut vertices: Vec<u32> = adjacency_list.keys().cloned().collect();
    //shuffle the copy of the keys (used to simulate randomly choosing x nodes)
    vertices.shuffle(&mut rng);
    //save the last num_vertices elements of the vector
    let chosen_vertices = &vertices[..num_vertices];

    // outer loop proceeds over all nodes
    for i in 0..num_vertices {
        let start_node = chosen_vertices[i];
        let dists = dijkstras(adjacency_list, start_node);
        // inner loop proceeds for all nodes in the list after location i.
        for j in (i + 1)..num_vertices {
            let end_node = chosen_vertices[j];
            let dist = dists[&end_node] as u32;
            // shortest_dists.insert((start_node, end_node), dist);
            let current_pair = DistancePair {
                node_1: start_node,
                node_2: end_node,
                distance: dist,
            };
            dijkstras_shortest_dists.push(current_pair);
        }
    }

    dijkstras_shortest_dists
}
