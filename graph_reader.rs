use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;

//This function converts a vector of edges to an adjacency list structure
// returns a hashmap of Key:ints & Value:vectors of ints
pub fn edges_to_adjacency_list(edges: &Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    let mut adjacency_list = HashMap::new();
    for &(src_node, dst_node) in edges {
        adjacency_list.entry(src_node).or_insert(vec![]).push(dst_node);
        adjacency_list.entry(dst_node).or_insert(vec![]).push(src_node);
    }
    adjacency_list
}


// converts an adjacency list (stored as a hashmap) for an unweighted graph into
// an adjacency list with weights (for algorithms that require a weighted graph)
// Since the code starts with an unweighted graph, the weight of 1 is assigned to each edge.
pub fn edges_to_weighted_adjacency_list(original_adj_list: &HashMap<u32, Vec<u32>>) -> HashMap<u32, Vec<(u32, u32)>> {
    original_adj_list
        .iter()
        .map(|(node, adj)| { //iterate over all the nodes
            let adj_with_weights = adj.iter().map(|&adj_node| (adj_node, 1)).collect(); //iterate over all the nodes adjacent to current node
            (*node, adj_with_weights)
        })
        .collect()
}


// read_edge_data will read the input file of edges into a vector of edges
// returns a vector of tuples.
pub fn read_edge_data(filename: &str) -> Vec<(u32, u32)> {

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut edges = vec![];

    //Input file is CSV formatted
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(reader);

    for result in csv_reader.records() {
        // println!("result = {:?}", result);
        let record = result.unwrap();
        let src_node = record[0].parse::<u32>().unwrap();
        let dst_node = record[1].parse::<u32>().unwrap();
        edges.push((src_node, dst_node));
    }

    edges

}
