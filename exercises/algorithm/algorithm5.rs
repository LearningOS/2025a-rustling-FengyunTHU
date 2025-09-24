/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

//I AM DONE
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices->n个节点
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		//TODO
        let mut search_from:Vec<usize> = vec![start];
        let mut visit_order = vec![start];
        // 检查所有的搜索到的节点是不是在集合中->用队列更合理
        loop {
            let mut search_to = vec![];
            for start_index in &search_from {
                for to_index in &self.adj[*start_index] {
                    if !search_to.contains(to_index) {
                        search_to.push(*to_index);
                    }
                }
            }
            let mut target:bool = true;
            search_from.clear();
            for posi in &search_to {
                if !visit_order.contains(posi) {
                    visit_order.push(*posi);
                    search_from.push(*posi);
                    target = false;
                }
            }
            if target {
                break;
            }
        }
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

