use std::fs::File;
use std::io::{BufRead, BufReader};

// Reads an adjacency list from a file and returns it as a vector of vectors
pub fn read_adj_list(filename: &str) -> Vec<Vec<usize>> {
   
    let file = File::open(filename).expect("Failed to open file");

    let reader = BufReader::new(file);
    let mut adj_list = vec![Vec::new(); 1005];// empty adjacency list with capacity 1005

    
    for (line_number, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if line.trim().is_empty() {
                    // Skip empty lines
                    continue;
                }

                // Parse the line into a vector of vertices
                let vertices: Vec<usize> = line
                    .split_whitespace()
                    .map(|s| s.parse().expect(&format!("Failed to parse number on line {}", line_number)))
                    .collect();

                // Skip lines that don't have at least 2 vertices
                if vertices.len() < 2 {
                    continue;
                }

                let u = vertices[0];
                let v = vertices[1];
                if u >= adj_list.len() {
                    adj_list.resize(u + 1, Vec::new());
                }
                if v >= adj_list.len() {
                    adj_list.resize(v + 1, Vec::new());
                }
                adj_list[u].push(v);
                adj_list[v].push(u);
            }

            Err(e) => {
                panic!("Failed to read line {}: {}", line_number, e);////panic with an error message
            }
        }
    }

    // Return the adjacency list
    adj_list
}
