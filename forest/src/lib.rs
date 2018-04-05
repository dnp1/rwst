use std::slice::Iter;
use std::cmp::Ordering;
use std::cmp::Ord;

struct Tree<T> {
    value: T,
    children: Option<Forest<T>>,
}


struct Forest<T>(Vec<Tree<T>>);

impl<T> Forest<T>
    where T: Ord {
    fn lookup(&self, elements: &[T]) -> Ordering {
        for tree in &self.0 {
            match tree.contains(&elements) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => return Ordering::Equal,
                Ordering::Less => continue,
            }
        }
        return Ordering::Less
    }
}

impl<T> Tree<T>
    where T: Ord {
    //Check if the tree contains slice
//    fn append(&self, elements: &[T]) {
//
//    }
    fn contains(&self, elements: &[T]) -> Ordering {
        match &elements.get(0) {
            &Some(ref element) => {
                let ord = self.value.cmp(element);
                if let Ordering::Equal = ord {
                    return match &self.children {
                        &Some(ref children) => children.lookup(&elements[1..]),
                        &None => if elements.len() == 1 {
                            Ordering::Equal
                        } else {
                            Ordering::Greater
                        }
                    }
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
