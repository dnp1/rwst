use std::slice::Iter;
use std::cmp::Ordering;
use std::cmp::Ord;

struct Tree<T> {
    index: T,
    children: Forest<T>
}


struct Forest<T>(Vec<Tree<T>>);



impl<T> Forest<T>
    where T: Ord + Clone  {
    fn lookup(&self, elements: &[T]) -> Ordering {
        for tree in &self.0 {
            match tree.contains(&elements) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => return Ordering::Equal,
                Ordering::Less => continue,
            }
        }
        return Ordering::Less;
    }

    fn append(&mut self, elements: &[T]) -> Ordering {
        for tree in &mut self.0 {
            match tree.append(&elements) {
                Ordering::Greater => break,
                Ordering::Equal => return Ordering::Equal,
                Ordering::Less => continue,
            }
        }
        match Tree::new(elements) {
            Some(t) => {
                self.0.push(t);
                self.0.sort();
                Ordering::Equal
            }
            None => Ordering::Equal
        }
    }
}

impl <T> std::cmp::Eq for Tree<T> where T: Ord {

}
impl <T> std::cmp::Ord for Tree<T> where T: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl <T> std::cmp::PartialEq for Tree<T> where T: Ord {
    fn eq(&self, other: &Self) -> bool {
        self.index.cmp(&other.index) == Ordering::Equal
    }
}


impl <T> std::cmp::PartialOrd for Tree<T> where T: Ord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index.cmp(&other.index))
    }
}
impl<T> Tree<T>
    where T: Ord + Clone {
    //Check if the tree contains slice

    fn new(elements: &[T]) -> Option<Self> {
        match elements.first() {
            Some(index) => {
                let index: T = index.clone();
                let mut t = Tree{index, children: Forest(Vec::new())};
                t.append_if_root_matches(&elements[1..]);
                Some(t)
            }
            None => None
        }
    }
    fn append_if_root_matches(&mut self, elements: &[T]) -> Ordering {
        match elements.first() {
            Some(index) => match self.index.cmp(&index) {
                Ordering::Equal => self.children.append(&elements[1..]),
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater
            },
            None => Ordering::Less
        }
    }
    fn contains(&self, elements: &[T]) -> Ordering {
        match &elements.first() {
            &Some(ref element) => {
                let ord = self.index.cmp(element);
                if let Ordering::Equal = ord {
                    return self.children.lookup(&elements[1..])
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
