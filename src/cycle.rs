use std::fmt::Debug;

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Cycle<T>(Vec<T>);

impl<T: PartialOrd + Debug> Cycle<T> {
    pub fn new(nodes: Vec<T>) -> Self {
        let rotation = rotation(&nodes);
        let rotated = rotate(nodes, rotation);
        let flipped = if &rotated[1] > rotated.last().expect("last") {
            flip(rotated)
        } else {
            rotated
        };

        Self(flipped)
    }
}

fn rotation<T: PartialOrd + Debug>(nodes: &Vec<T>) -> usize {
    let mut iter = nodes.iter().rev().enumerate();
    let (mut index, mut value) = iter.next().expect("");

    while let Some((i, v)) = iter.next() {
        if v < &value {
            value = v;
            index = i
        }
    }

    if index == nodes.len() - 1 {
        0
    } else {
        index + 1
    }
}

fn rotate<T: Debug>(mut nodes: Vec<T>, count: usize) -> Vec<T> {
    let mut result = Vec::new();

    for _ in 0..count {
        result.push(nodes.pop().expect(""));
    }

    result.reverse();
    result.append(&mut nodes);

    result
}

fn flip<T: Debug>(mut nodes: Vec<T>) -> Vec<T> {
    let mut stack = Vec::new();

    for _ in 0..(nodes.len() - 1) {
        stack.push(nodes.pop().expect(""))
    }

    println!("stack {:?}", stack);

    nodes.append(&mut stack);

    nodes
}

#[cfg(test)]
mod rotate {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        let nodes = vec![0, 1, 2];

        assert_eq!(rotate(nodes, 0), vec![0, 1, 2])
    }

    #[test]
    fn one() {
        let nodes = vec![0, 1, 2];

        assert_eq!(rotate(nodes, 1), vec![2, 0, 1])
    }

    #[test]
    fn two() {
        let nodes = vec![1, 0, 2];

        assert_eq!(rotate(nodes, 2), vec![0, 2, 1])
    }

    #[test]
    fn foo() {
        let nodes = vec![1, 0, 2];

        assert_eq!(rotate(nodes, 2), vec![0, 2, 1])
    }
}

#[cfg(test)]
mod rotation {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn min_left() {
        let nodes = vec![0, 1, 2];

        assert_eq!(rotation(&nodes), 0)
    }

    #[test]
    fn min_middle() {
        let nodes = vec![1, 0, 2];

        assert_eq!(rotation(&nodes), 2)
    }

    #[test]
    fn min_right() {
        let nodes = vec![1, 2, 0];

        assert_eq!(rotation(&nodes), 1)
    }
}

#[cfg(test)]
mod flip {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn foo() {
        let nodes = vec![0, 2, 1];

        assert_eq!(flip(nodes), vec![0, 1, 2])
    }
}

#[cfg(test)]
mod new {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn no_rotate_no_flip() {
        let nodes = vec![0, 1, 2];

        assert_eq!(Cycle::new(nodes), Cycle(vec![0, 1, 2]))
    }

    #[test]
    fn rotate_no_flip() {
        let nodes = vec![2, 0, 1];

        assert_eq!(Cycle::new(nodes), Cycle(vec![0, 1, 2]))
    }

    #[test]
    fn no_rotate_flip() {
        let nodes = vec![0, 2, 1];

        assert_eq!(Cycle::new(nodes), Cycle(vec![0, 1, 2]))
    }
}
