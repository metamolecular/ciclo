use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::{Error, Path};

#[derive(Debug, PartialEq)]
pub struct Graph<T: Eq + Hash + Clone + Debug> {
    paths: HashMap<T, Vec<Path<T>>>,
}

impl<T: Eq + Hash + Clone + Debug> Graph<T> {
    pub fn from_edges(edges: Vec<(T, T)>) -> Result<Self, Error> {
        let mut paths: HashMap<T, Vec<Path<T>>> = HashMap::new();

        for (source, target) in edges {
            if source == target {
                return Err(Error::Dummy);
            }

            match paths.entry(source.clone()) {
                std::collections::hash_map::Entry::Occupied(mut occupied) => {
                    for path in occupied.get().iter() {
                        if &path.target == &target {
                            return Err(Error::Dummy);
                        }
                    }

                    occupied.get_mut().push(Path::to(target.clone()));
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(vec![Path::to(target.clone())]);
                }
            }
            match paths.entry(target) {
                std::collections::hash_map::Entry::Occupied(mut occupied) => {
                    occupied.get_mut().push(Path::to(source.clone()))
                }
                std::collections::hash_map::Entry::Vacant(vacant) => {
                    vacant.insert(vec![Path::to(source.clone())]);
                }
            }
        }

        Ok(Self { paths })
    }

    pub fn keys(&self) -> Vec<T> {
        self.paths.keys().cloned().collect()
    }

    pub fn remove(&mut self, out_source: &T) -> Option<Vec<Path<T>>> {
        let out_paths = self.paths.remove(out_source)?;
        let mut paths = HashMap::new();

        std::mem::swap(&mut self.paths, &mut paths);

        for (in_source, in_paths) in paths {
            let mut new_in_paths = Vec::new();

            for in_path in in_paths.into_iter() {
                if &in_path.target == out_source {
                    for out_path in out_paths.iter() {
                        if let Some(new_path) =
                            in_path.concat(&in_source, &out_path)
                        {
                            new_in_paths.push(new_path)
                        }
                    }
                } else {
                    new_in_paths.push(in_path)
                }
            }

            if !new_in_paths.is_empty() {
                self.paths.insert(in_source, new_in_paths);
            }
        }

        Some(out_paths)
    }
}

#[cfg(test)]
mod from_edges {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn loop_edge() {
        let edges = vec![(1, 1)];

        assert_eq!(Graph::from_edges(edges), Err(Error::Dummy))
    }

    #[test]
    fn duplicate_edge_forward() {
        let edges = vec![(0, 1), (0, 1)];

        assert_eq!(Graph::from_edges(edges), Err(Error::Dummy))
    }

    #[test]
    fn duplicate_edge_reverse() {
        let edges = vec![(0, 1), (1, 0)];

        assert_eq!(Graph::from_edges(edges), Err(Error::Dummy))
    }

    #[test]
    fn p3_duplicate_second_reverse() {
        let edges = vec![(0, 1), (1, 2), (2, 1)];

        assert_eq!(Graph::from_edges(edges), Err(Error::Dummy))
    }

    #[test]
    fn p2() {
        let edges = vec![(0, 1)];

        assert_eq!(
            Graph::from_edges(edges),
            Ok(Graph {
                paths: vec![(0, vec![Path::to(1)]), (1, vec![Path::to(0)]),]
                    .into_iter()
                    .collect()
            })
        )
    }

    #[test]
    fn p3() {
        let edges = vec![(0, 1), (1, 2)];

        assert_eq!(
            Graph::from_edges(edges),
            Ok(Graph {
                paths: vec![
                    (0, vec![Path::to(1)]),
                    (1, vec![Path::to(0), Path::to(2)]),
                    (2, vec![Path::to(1)])
                ]
                .into_iter()
                .collect()
            })
        )
    }

    #[test]
    fn p3_inverted_first() {
        let edges = vec![(1, 0), (1, 2)];

        assert_eq!(
            Graph::from_edges(edges),
            Ok(Graph {
                paths: vec![
                    (0, vec![Path::to(1)]),
                    (1, vec![Path::to(0), Path::to(2)]),
                    (2, vec![Path::to(1)])
                ]
                .into_iter()
                .collect()
            })
        )
    }

    #[test]
    fn p3_inverted_second() {
        let edges = vec![(0, 1), (2, 1)];

        assert_eq!(
            Graph::from_edges(edges),
            Ok(Graph {
                paths: vec![
                    (0, vec![Path::to(1)]),
                    (1, vec![Path::to(0), Path::to(2)]),
                    (2, vec![Path::to(1)])
                ]
                .into_iter()
                .collect()
            })
        )
    }

    #[test]
    fn s3() {
        let edges = vec![(0, 1), (1, 2), (1, 3)];

        assert_eq!(
            Graph::from_edges(edges),
            Ok(Graph {
                paths: vec![
                    (0, vec![Path::to(1)]),
                    (1, vec![Path::to(0), Path::to(2), Path::to(3)]),
                    (2, vec![Path::to(1)]),
                    (3, vec![Path::to(1)])
                ]
                .into_iter()
                .collect()
            })
        )
    }
}

#[cfg(test)]
mod remove {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn missing_node() {
        let mut graph = Graph {
            paths: vec![(0, vec![Path::to(1)]), (1, vec![Path::to(0)])]
                .into_iter()
                .collect(),
        };

        assert_eq!(graph.remove(&2), None)
    }

    #[test]
    fn p3_terminal() {
        let mut graph = Graph {
            paths: HashMap::from([
                (0, vec![Path::to(1)]),
                (1, vec![Path::to(0), Path::to(2)]),
                (2, vec![Path::to(1)]),
            ]),
        };

        assert_eq!(graph.remove(&0), Some(vec![Path::to(1)]));
        assert_eq!(
            graph,
            Graph {
                paths: HashMap::from([
                    (1, vec![Path::to(2)]),
                    (2, vec![Path::to(1)])
                ])
            }
        )
    }

    #[test]
    fn p3_internal() {
        let mut graph = Graph {
            paths: HashMap::from([
                (0, vec![Path::to(1)]),
                (1, vec![Path::to(0), Path::to(2)]),
                (2, vec![Path::to(1)]),
            ]),
        };

        assert_eq!(graph.remove(&1), Some(vec![Path::to(0), Path::to(2)]));
        assert_eq!(
            graph,
            Graph {
                paths: HashMap::from([
                    (0, vec![Path::new(vec![1], 2)]),
                    (2, vec![Path::new(vec![1], 0)])
                ])
            }
        )
    }

    #[test]
    fn s3_internal() {
        let mut graph = Graph {
            paths: HashMap::from([
                (0, vec![Path::to(1)]),
                (1, vec![Path::to(0), Path::to(2), Path::to(3)]),
                (2, vec![Path::to(1)]),
                (3, vec![Path::to(1)]),
            ]),
        };

        assert_eq!(
            graph.remove(&1),
            Some(vec![Path::to(0), Path::to(2), Path::to(3)])
        );
        assert_eq!(
            graph,
            Graph {
                paths: HashMap::from([
                    (0, vec![Path::new(vec![1], 2), Path::new(vec![1], 3)]),
                    (2, vec![Path::new(vec![1], 0), Path::new(vec![1], 3),]),
                    (3, vec![Path::new(vec![1], 0), Path::new(vec![1], 2)])
                ])
            }
        )
    }

    #[test]
    fn pre_cycle() {
        let mut graph = Graph {
            paths: HashMap::from([
                (0, vec![Path::new(vec![1], 2), Path::to(2)]),
                (2, vec![Path::new(vec![1], 0), Path::to(0)]),
            ]),
        };

        assert_eq!(
            graph.remove(&2),
            Some(vec![Path::new(vec![1], 0), Path::to(0)])
        );
        assert_eq!(
            graph,
            Graph {
                paths: HashMap::from([(
                    0,
                    vec![Path::new(vec![1, 2], 0), Path::new(vec![2, 1], 0)]
                )])
            }
        )
    }
}
