/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
    写一个二叉树
*/

//I AM DONE
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        // 从根节点开始插入搜索
        // self.root是Option<Box<TreeNode<T>>>类型
        let mut ptr_root = &mut self.root;
        match ptr_root {
            None => self.root = Some(Box::new(TreeNode::new(value))),
            Some(ref mut root_node) => {
                // &mut Box<TreeNode>
                root_node.insert(value);
            }
        };
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        // 做搜索
        let ptr_root = &self.root;
        match ptr_root {
            None => false,
            Some(ref root_node) => root_node.search(value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree->从一个节点开始寻找位置。这里实现迭代函数更方便
    fn insert(&mut self, value: T) {
        //TODO
        // 判断当前节点的值。如果大就插入在右边，反之左边
        if self.value == value {
            ()
        } else if self.value > value {
            // 插入在左侧
            match self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut left) => left.insert(value),
            };
        } else {
            match self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut right) => right.insert(value),
            };
        }
    }

    fn search(& self, value: T) -> bool {
        // 从一个节点开始进行搜索
        if self.value == value {
            true
        } else if self.value > value {
            match self.left {
                None => false,
                Some(ref left) => left.search(value),
            }
        } else {
            match self.right {
                None => false,
                Some(ref right) => right.search(value),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


