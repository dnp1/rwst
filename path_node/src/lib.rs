use std::slice::Iter;
use std::cmp::Ordering;
use std::cmp::Ord;

struct Node<I, V> {
    index: I,
    value: Option<V>,
    children: NodeSet<I, V>,
}


struct NodeSet<I, V> {
    nodes: Vec<Node<I, V>>
}


impl<I, V> NodeSet<I, V>
    where I: Ord + Clone + std::fmt::Debug, V: Clone
{
    fn new() -> Self {
        NodeSet { nodes: Vec::new() }
    }

    fn contains<T> (&self, path: T) -> bool where T: Iterator<Item=I> + Clone {
        match self.lookup(path)   {
            Ordering::Equal => true,
            _ => false
        }
    }

    fn lookup<T>(&self, path: T) -> Ordering where T: Iterator<Item=I> + Clone {
        for node in &self.nodes {
            match node.contains(path.clone()) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => return Ordering::Equal,
                Ordering::Less => continue,
            }
        }
        return Ordering::Less;
    }

    fn add<T>(&mut self, path: T, value: Option<V>) -> Option<Ordering> where T: Iterator<Item=I> + Clone {
        for root in &mut self.nodes {
            match root.try_add(path.clone(), value.clone()) {
                Some(ord) => match ord {
                    Ordering::Greater => break,
                    Ordering::Equal => return Some(Ordering::Equal),
                    Ordering::Less => continue,
                },
                None => return None,
            }
        }

        match Node::new(path, value) {
            Some(t) => {
                self.nodes.push(t);
                self.nodes.sort();
                Some(Ordering::Equal)
            }
            None => None
        }
    }
}

impl<I, V> std::cmp::Eq for Node<I, V> where I: Ord {}

impl<I, V> std::cmp::Ord for Node<I, V> where I: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl<I, V> std::cmp::PartialEq for Node<I, V> where I: Ord {
    fn eq(&self, other: &Self) -> bool {
        self.index.cmp(&other.index) == Ordering::Equal
    }
}


impl<I, V> std::cmp::PartialOrd for Node<I, V> where I: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

impl<I, V> Node<I, V>
    where I: Ord + Clone + std::fmt::Debug, V: Clone
{
    //Check if the tree contains slice

    fn new<T>(mut path: T, value: Option<V>) -> Option<Self> where T: Iterator<Item=I> + Clone {
        match path.next() {
            Some(index) => {
                let index: I = index.clone();
                let mut t = Node::<I, V> { index, children: NodeSet { nodes: Vec::new() },  value: None };
                if let None = t.children.add(path, value.clone()) {
                    t.value = value.clone()
                }
                Some(t)
            }
            None => None
        }
    }
    fn try_add<T>(&mut self, mut path: T, value: Option<V>) -> Option<Ordering> where T: Iterator<Item=I> + Clone {
        match path.next() {
            Some(index) => match self.index.cmp(&index) {
                Ordering::Equal => self.children.add(path, value),
                Ordering::Less => Some(Ordering::Less),
                Ordering::Greater => Some(Ordering::Greater)
            },
            None => {
                self.value = value;
                None
            }
        }
    }

    fn contains<T>(&self, mut path: T) -> Ordering where T: Iterator<Item=I> + Clone {
        match path.next() {
            Some(element) => {
                let ord = self.index.cmp(&element);
                if let Ordering::Equal = ord {
                    if path.clone().count() == 0 {
                        return Ordering::Equal
                    }
                    return self.children.lookup(path);
                }
                ord
            }
            None => {
                println!("gol!");
                return Ordering::Equal
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::NodeSet;

    #[test]
    fn test_add_and_lookup() {
        let mut router = NodeSet::new();
        const S: &'static str = "/fruta/banana/da-terra";
        let pieces: ::std::str::Split<_> = S.split("/");
        let v: Vec<_> = pieces.clone().collect();
        assert_eq!(v, ["", "fruta", "banana", "da-terra"]);
        router.add(pieces, Some("dsada"));
        assert_eq!(router.contains(S.split("/")), true)
    }
}
