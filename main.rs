//Cooperators：None
mod read_adj_list;
mod shortest_path;
mod average_distance;
mod generate_random_graph;

fn main() {
    let adj_list = read_adj_list::read_adj_list("email-Eu-core.txt");
    let avg_distance = average_distance::average_distance(&adj_list);
    println!("Average distance in email-Eu-core: {:.2}", avg_distance);

    let n = adj_list.len();
    let m = adj_list.iter().map(|edges| edges.len()).sum::<usize>() / 2;
    let random_graph = generate_random_graph::generate_random_graph(n, m);
    let random_avg_distance = average_distance::average_distance(&random_graph);
    println!("Average distance in random graph: {:.2}", random_avg_distance);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path() {
        let adj_list = vec![
            vec![1],
            vec![0, 2],
            vec![1],
        ];

        // Test that the shortest path between two vertices is correct
        let path = shortest_path::shortest_path(&adj_list, 0, 2).expect("No path found.");
        assert_eq!(path, vec![0, 1, 2]);

        // Test that there is no path between two disconnected vertices
        let path = shortest_path::shortest_path(&adj_list, 0, 3);
        assert!(path.is_none(), "Path should be None.");
    }

    #[test]
    fn test_average_distance() {
        // Construct a simple graph with three vertices and two edges
        let adj_list = vec![
            vec![1],
            vec![0, 2],
            vec![1],
        ];

        // Test that the average distance between all pairs of vertices is correct
        let avg_distance = average_distance::average_distance(&adj_list);
        assert_eq!(avg_distance, 4.0 / 3.0, "Average distance is incorrect.");
    }
    #[test]
    fn test_generate_random_graph() {
        // test graph with 5 vertices and 6 edges
        let n = 5;
        let m = 6;
        let graph = generate_random_graph::generate_random_graph(n, m);
    
        // check that the graph has the correct number of edges
        let num_edges = graph.iter().map(|v| v.len()).sum::<usize>();
        assert_eq!(num_edges, m, "Incorrect number of edges.");
    }
    
    
}
