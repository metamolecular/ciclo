use std::{fmt::Debug, hash::Hash};

use crate::Graph;

pub fn all_cycles<T: Eq + Hash + Clone + Debug>(
    mut graph: Graph<T>,
) -> Vec<Vec<T>> {
    let mut result = Vec::new();

    for key in graph.keys() {
        for paths in graph.remove(&key) {
            for path in paths {
                if &path.target != &key {
                    continue;
                }

                result.push(Vec::from(path))
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::Cycle;

    use super::*;
    use pretty_assertions::assert_eq;

    fn to_set(cycles: Vec<Vec<u32>>) -> HashSet<Cycle<u32>> {
        cycles
            .into_iter()
            .map(|c| Cycle::new(c))
            .collect::<HashSet<_>>()
    }

    #[test]
    fn p3() {
        let graph = Graph::from_edges(vec![(0, 1)]).unwrap();

        assert_eq!(all_cycles(graph), Vec::<Vec<u32>>::new())
    }

    #[test]
    fn c3() {
        let graph = Graph::from_edges(vec![(0, 1), (1, 2), (2, 0)]).unwrap();

        assert_eq!(
            to_set(all_cycles(graph)),
            HashSet::from([Cycle::new(vec![0, 1, 2])])
        )
    }

    #[test]
    fn butterfly() {
        let graph = Graph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 2),
            (2, 0),
        ])
        .unwrap();

        assert_eq!(
            to_set(all_cycles(graph)),
            HashSet::from([
                Cycle::new(vec![0, 1, 2]),
                Cycle::new(vec![2, 3, 4])
            ])
        )
    }

    #[test]
    fn diamond() {
        let graph =
            Graph::from_edges(vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 1)])
                .unwrap();

        assert_eq!(
            to_set(all_cycles(graph)),
            HashSet::from([
                Cycle::new(vec![0, 1, 2]),
                Cycle::new(vec![1, 2, 3]),
                Cycle::new(vec![0, 1, 3, 2])
            ])
        )
    }

    #[test]
    fn k4() {
        let graph = Graph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (0, 2),
            (1, 3)
        ])
        .unwrap();

        assert_eq!(
            to_set(all_cycles(graph)),
            HashSet::from([
                Cycle::new(vec![0, 1, 2]),
                Cycle::new(vec![0, 1, 3]),
                Cycle::new(vec![0, 2, 3]),
                Cycle::new(vec![1, 2, 3]),
                Cycle::new(vec![1, 2, 0, 3]),
                Cycle::new(vec![1, 0, 2, 3]),
                Cycle::new(vec![0, 1, 2, 3])
            ])
        )
    }

    // see: http://efficientbits.blogspot.com/2013/06/allringsfinder-sport-edition.html
    #[test]
    fn k5() {
        let graph = Graph::from_edges(vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 0),
            (0, 3),
            (0, 2),
            (1, 3),
            (1, 4),
            (2, 4),
        ]).unwrap();

        assert_eq!(all_cycles(graph).len(), 37 * 2)
    }
}
