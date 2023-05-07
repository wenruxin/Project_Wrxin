use rand::seq::SliceRandom;
use rand::thread_rng;

// Generates a random directed graph represented as an adjacency list with n vertices and m edges
pub fn generate_random_graph(n: usize, m: usize) -> Vec<Vec<usize>> {
    // initialize an empty adjacency list with n vertices
    let mut adj_list = vec![Vec::new(); n];

    // calculate the maximum number of edges
    let max_edges = n * (n - 1);
// ensure that m is not greater than the maximum number of edges
    let m = std::cmp::min(m, max_edges);

    // generate possible edges
    let edges: Vec<(usize, usize)> = (0..n)
        .flat_map(|u| (0..n).map(move |v| (u, v)))//flatten the resulting
        .collect();

    let mut rng = thread_rng();

    
    edges.choose_multiple(&mut rng, m).for_each(|(u, v)| { // choose m random edges
        adj_list[*u].push(*v);
    });

   
    adj_list
}
