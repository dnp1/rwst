extern crate regex;
use std::slice::Iter;
use std::cmp::Ordering;
use std::cmp::Ord;
use std::fmt::Debug;
pub use regex::Regex;

#[derive(Debug)]
enum Error {
    InvalidPath
}

#[derive(Debug)]
enum NodeIndex {
    Value(String),
    Param(Regex),
    Match(Regex)
}

#[derive(Debug)]
struct Node<V> {
    index: NodeIndex,
    handler: Option<V>,
    children: Option<Vec<Node<V>>>,
}

impl <'a>From<&'a str> for NodeIndex {
    fn from(index_str: &'a str) -> Self {
        NodeIndex::Value(index_str.to_owned())
        //TODO: fix_it
    }
}

const SPLIT_CHAR: &str = "/";


impl <V>Node<V> {
    fn new(index: NodeIndex) -> Self {
        Node {
            index,
            handler: None,
            children: None
        }
    }

    fn from_path(path: &str) -> Result<Self, Error> {
        let mut path = path.split(SPLIT_CHAR).filter(|x| x.trim() != "");
        match path.next() {
            Some(index_str) => {
                let first = NodeIndex::from(index_str);
                Ok(Self::new(first))
            },
            None => Err(Error::InvalidPath)
        }
    }

    fn number_of_children(&self) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeIndex};

    #[test]
    fn test_after_create_len_is_0() {
        let mut node = Node::new(NodeIndex::Value("banana".to_owned()));
        node.handler = Some(10);
        assert_eq!(0 as u32, node.number_of_children())
    }

    #[test]
    fn test_create_from_a_path_work_properly() {
        let mut node = Node::<i32>::from_path("/banana/abacate").unwrap();
        assert_eq!("banana", match node.index {
            NodeIndex::Value(v) => v,
            _ => "_".to_owned()
        })
    }
}



