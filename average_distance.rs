use crate::shortest_path::shortest_path;

// Calculates the average distance 
pub fn average_distance(adj_list: &[Vec<usize>]) -> f64 {
    let n = adj_list.len();
    let mut total_distance = 0;
    let mut num_pairs = 0;

    // iterate over all pairs of vertices
    for u in 0..n {
        for v in 0..n {
            if u != v {
                // calculate shortest path from u to v
                let u_to_v = shortest_path(adj_list, u, v);

               
                if let Some(path) = u_to_v {
                    total_distance += path.len() - 1;
                    num_pairs += 1;
                }
            }
        }
    }

    // calculate and return the average distance
    total_distance as f64 / num_pairs as f64
}
