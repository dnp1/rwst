use std::slice::Iter;
use std::cmp::Ordering;
use std::cmp::Ord;

struct Tree<T> {
    value: T,
    children: Option<Vec<Tree<T>>>,
}
//
//struct TreeSet<T>(Vec<Tree<T>>);
//
//impl<T> TreeSet<T>
//    where T: PartialEq + PartialOrd {
//    fn lookup(&self, elements: &[T]) -> bool {
//        if let &Some(ref element) = &elements.get(0) {
//            for tree in &self.0 {
//                if tree.lookup(&element, &elements[1..]) {
//                    return true
//                }
//            }
//        }
//        return false
//    }
//}

impl<T> Tree<T>
    where T: Ord {
    fn lookup(&self, element: &T, mut elements: &[T]) -> Ordering {
        let ord = self.value.cmp(element);
        if let Ordering::Equal = ord {
            match &elements.get(0) {
                &Some(ref element) => {
                    match &self.children {
                        &Some(ref children) =>
                            for tree in children {
                                match tree.lookup(&element, &elements[1..]) {
                                    Ordering::Greater => return Ordering::Greater,
                                    Ordering::Equal => return Ordering::Equal,
                                    Ordering::Less => continue,
                                }
                            },
                        &None => return Ordering::Less,
                    }
                }
                &None => return Ordering::Equal
            }
        }
        return ord;
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
