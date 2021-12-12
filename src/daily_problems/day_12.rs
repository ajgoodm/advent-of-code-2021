pub mod solutions {
    use std::collections::{HashMap, HashSet};

    use lazy_static::lazy_static;

    use crate::AocBufReader;
    use crate::utils::str_utils::is_lower_case;

    lazy_static! {
        static ref START: String  = "start".to_string();
        static ref END: String = "end".to_string();
    }


    #[derive(Clone)]
    struct Path {
        nodes: Vec<String>,
        has_revisited: bool
    }


    impl Path {
        fn new(node: &String) -> Path {
            let mut path = Path { nodes: vec![], has_revisited: false };
            path.add_node(node);
            path            
        }

        fn add_node(&mut self, node: &String) {
            self.nodes.push(node.clone());
        }

        fn terminus(&self) -> &String {
            self.nodes.last().unwrap()
        }

        fn was_small_cave_visited(&self, node: &String) -> bool {
            if !is_lower_case(node) {
                false
            } else {
                self.nodes.contains(node)
            }
        }

        fn print(&self) {
            println!("{}", self.nodes.join(","));
        }
    }


    struct CaveMap {
        graph: HashMap<String, HashSet<String>>
    }


    impl CaveMap {
        fn from_reader(aoc_reader: AocBufReader) -> CaveMap {
            let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
            for line in aoc_reader {
                let nodes: Vec<&str> = line.split("-").collect();
                assert_eq!(nodes.len(), 2);

                graph.entry(nodes[0].to_string()).or_insert(HashSet::new()).insert(nodes[1].to_string());
                graph.entry(nodes[1].to_string()).or_insert(HashSet::new()).insert(nodes[0].to_string());
            }
            CaveMap { graph: graph }
        }

        fn get_adjacent_nodes(&self, node: &String) -> Vec<&String> {
            if let Some(neighbors) = self.graph.get(node) {
                return neighbors.iter().collect()
            } else {return vec![]}
        }

        fn find_paths_pt_1(&self) -> Vec<Path> {
            let mut living_paths: Vec<Path> = vec![Path::new(&START)];
            let mut complete_paths: Vec<Path> = vec![];
            while living_paths.len() > 0 {
                let this_path = living_paths.pop().unwrap();
                for next_node in self.get_adjacent_nodes(this_path.terminus()) {
                    if !this_path.was_small_cave_visited(next_node) {
                        if &next_node[..] == &END[..] {
                            let mut finished_path = this_path.clone();
                            finished_path.add_node(&END);
                            complete_paths.push(finished_path);
                        } else {
                            let mut living_path = this_path.clone();
                            living_path.add_node(next_node);
                            living_paths.push(living_path);
                        }
                    }
                }
            }
            complete_paths
        }

        fn find_paths_pt_2(&self) -> Vec<Path> {
            let mut living_paths: Vec<Path> = vec![Path::new(&START)];
            let mut complete_paths: Vec<Path> = vec![];
            while living_paths.len() > 0 {
                let this_path = living_paths.pop().unwrap();
                for next_node in self.get_adjacent_nodes(this_path.terminus()) {
                    if &next_node[..] == &START[..] { continue }
                    if !this_path.was_small_cave_visited(next_node) || !this_path.has_revisited {
                        if &next_node[..] == &END[..] {
                            let mut finished_path = this_path.clone();
                            finished_path.add_node(&END);
                            complete_paths.push(finished_path);
                        } else {
                            let mut living_path = this_path.clone();
                            if this_path.was_small_cave_visited(next_node) {
                                living_path.has_revisited = true;
                            }
                            living_path.add_node(next_node);
                            living_paths.push(living_path);
                        }
                    }
                }
            }
            complete_paths
        }
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let cave_map = CaveMap::from_reader(aoc_reader);
        cave_map.find_paths_pt_1().len()
    }

    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let cave_map = CaveMap::from_reader(aoc_reader);
        cave_map.find_paths_pt_2().len()
    }

}