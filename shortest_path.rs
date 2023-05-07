use std::collections::VecDeque;
pub fn shortest_path(adj_list: &[Vec<usize>], start: usize, end: usize) -> Option<Vec<usize>> {
    
    let mut parent = vec![None; adj_list.len()];

    // Double-ended queue for BFS
    let mut queue = VecDeque::new();
    parent[start] = Some(start);

    queue.push_back(start);

    // BFS 
    while let Some(u) = queue.pop_front() {
       
        if u == end {
            let mut path = Vec::new();
            let mut v = end;
            while v != start {
                path.push(v);
                v = parent[v].expect("Parent of vertex not set");
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        // Traverse all adjacent vertices
        for &v in &adj_list[u] {
            // If the adjacent vertex is not visited, add it to the queue
            if parent[v].is_none() {
                parent[v] = Some(u);
                queue.push_back(v);
            }
        }
    }

   
    None
}