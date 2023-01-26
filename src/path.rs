use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub struct Path<T: PartialEq + Debug> {
    pub bridge: Vec<T>,
    pub target: T,
}

impl<T: PartialEq + Debug> From<Path<T>> for Vec<T> {
    fn from(value: Path<T>) -> Self {
        value
            .bridge
            .into_iter()
            .chain(std::iter::once(value.target))
            .collect()
    }
}

impl<T: PartialEq + Clone + core::fmt::Debug> Path<T> {
    pub fn new(bridge: Vec<T>, target: T) -> Self {
        Self { bridge, target }
    }

    pub fn to(target: T) -> Self {
        Self {
            bridge: Vec::new(),
            target,
        }
    }

    pub fn concat(&self, source: &T, other: &Self) -> Option<Self> {
        if &other.target == source {
            if self.bridge.is_empty() && other.bridge.is_empty() {
                None
            } else {
                self.splice(other)
            }
        } else {
            self.splice(other)
        }
    }

    fn splice(&self, other: &Self) -> Option<Self> {
        let mut bridge = self.bridge.clone();

        bridge.push(self.target.clone());

        for id in other.bridge.iter() {
            if bridge.contains(id) {
                return None;
            } else {
                bridge.push(id.clone())
            }
        }

        Some(Self {
            bridge,
            target: other.target.clone(),
        })
    }
}

#[cfg(test)]
mod concatenate {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn back_edge() {
        let in_path = Path::to(1);
        let out_path = Path::to(0);

        assert_eq!(in_path.concat(&0, &out_path), None)
    }

    #[test]
    fn out_and_back() {
        let in_path = Path::new(vec![1], 2);
        let out_path = Path::new(vec![1], 0);

        assert_eq!(in_path.concat(&0, &out_path), None)
    }

    #[test]
    fn straight() {
        let in_path = Path::new(vec![1], 2);
        let out_path = Path::new(vec![3], 4);

        assert_eq!(
            in_path.concat(&0, &out_path),
            Some(Path {
                bridge: vec![1, 2, 3],
                target: 4
            })
        )
    }

    #[test]
    fn cycle() {
        let in_path = Path::new(vec![1], 2);
        let out_path = Path::to(0);

        assert_eq!(
            in_path.concat(&0, &out_path),
            Some(Path::new(vec![1, 2], 0))
        )
    }
}
