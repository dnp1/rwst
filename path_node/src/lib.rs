use std::slice::Iter;
use std::cmp::Ordering;
use std::cmp::Ord;

struct Node<T> {
    index: T,
    children: NodeSet<T>
}


struct NodeSet<T> {
    nodes: Vec<Node<T>>
}



impl<T> NodeSet<T>
    where T: Ord + Clone  {
    fn lookup(&self, path: &[T]) -> Ordering {
        for node in &self.nodes {
            match node.contains(&path) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => return Ordering::Equal,
                Ordering::Less => continue,
            }
        }
        return Ordering::Less;
    }

    fn add(&mut self, path: &[T]) -> Ordering {
        for root in &mut self.nodes {
            match root.try_add(&path) {
                Ordering::Greater => break,
                Ordering::Equal => return Ordering::Equal,
                Ordering::Less => continue,
            }
        }
        match Node::new(path) {
            Some(t) => {
                self.nodes.push(t);
                self.nodes.sort();
                Ordering::Equal
            }
            None => Ordering::Equal
        }
    }
}

impl <T> std::cmp::Eq for Node<T> where T: Ord {

}
impl <T> std::cmp::Ord for Node<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl <T> std::cmp::PartialEq for Node<T> where T: Ord {
    fn eq(&self, other: &Self) -> bool {
        self.index.cmp(&other.index) == Ordering::Equal
    }
}


impl <T> std::cmp::PartialOrd for Node<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index.cmp(&other.index))
    }
}
impl<T> Node<T>
    where T: Ord + Clone {
    //Check if the tree contains slice

    fn new(path: &[T]) -> Option<Self> {
        match path.first() {
            Some(index) => {
                let index: T = index.clone();
                let mut t = Node {index, children: NodeSet { nodes: Vec::new()}};
                t.try_add(&path[1..]);
                Some(t)
            }
            None => None
        }
    }
    fn try_add(&mut self, path: &[T]) -> Ordering {
        match path.first() {
            Some(index) => match self.index.cmp(&index) {
                Ordering::Equal => self.children.add(&path[1..]),
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater
            },
            None => Ordering::Less
        }
    }
    fn contains(&self, path: &[T]) -> Ordering {
        match &path.first() {
            &Some(ref element) => {
                let ord = self.index.cmp(element);
                if let Ordering::Equal = ord {
                    return self.children.lookup(&path[1..])
                }
                ord
            }
            &None => return Ordering::Equal
        }
    }
}


#[cfg(test)]
mod tests {
    fn print_all<T: ::std::fmt::Display>(v: &[T]) {
        for k in v {
            println!("{}", k)
        }
    }

    #[test]
    fn it_works() {
        let mut i: Vec<_> = [1, 2, 3, 5, 6].iter().collect();
        let mut it = i.into_iter();
        print_all(it.as_slice());
        print_all(it.as_slice());


        assert_eq!(2 + 2, 4);
    }
}
