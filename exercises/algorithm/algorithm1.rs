/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
    单链表合并
    这个问题要求你将两个有序单链表合并成一个有序单链表
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
    where
        T: Default+PartialOrd+Copy
	{
		//TODO
        // 在lista的结尾添加指向listb开头的指针
        // if let Some(end_ptr_a) = list_a.end { // 使用iflet处理
        //     unsafe{ (*end_ptr_a.as_ptr()).next = list_b.start;}
        // };
        match list_a.end {
            Some(end_ptr_a) => unsafe{(*end_ptr_a.as_ptr()).next = list_b.start;},
            None => (),
        };
		let mut linklist_no_sort = Self {
            length: list_a.length+list_b.length,
            start: list_a.start,
            end: list_b.end,
        };
        // 对链表做排序->从小到大
        let T_default = T::default();
        let mut node_ori = Box::new(Node::new(T_default)); // 创建一个空的起始节点
        node_ori.next = None;
        let node_ptr_ori = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node_ori)) }); // 这里nodeori已经被释放
        let mut end_ptr = node_ptr_ori;
        for i in 0..linklist_no_sort.length {
            let mut node_now = Box::new(Node::new(T_default));
            match linklist_no_sort.get(i as i32) {
                Some(value) => node_now = Box::new(Node::new(*value)),
                None => ()
            };
            node_now.next = None;
            let mut current_ptr = node_ptr_ori;
            let mut former_ptr = node_ptr_ori;
            loop {
                match unsafe{(*current_ptr.unwrap().as_ptr()).next} {
                    None => unsafe{
                        (*current_ptr.unwrap().as_ptr()).next = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node_now)) }); // 插入i节点
                        // 记录终止节点
                        end_ptr = (*current_ptr.unwrap().as_ptr()).next;
                        break;
                    }
                    Some(next_ptr) => unsafe{
                        if node_now.val > (*next_ptr.as_ptr()).val {
                            former_ptr = current_ptr;
                            current_ptr = Some(next_ptr);
                        } else {
                            // 执行插入操作
                            node_now.next = current_ptr;
                            (*former_ptr.unwrap().as_ptr()).next = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node_now)) });
                            break;
                        }
                    }
                }
            }
        }
        Self {
            length: linklist_no_sort.length,
            start: unsafe { (*node_ptr_ori.unwrap().as_ptr()).next },
            end: end_ptr,
        }
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}