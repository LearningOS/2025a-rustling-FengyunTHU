/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM DONE

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator, // 比较函数->表达优先级->优先级高为父节点
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        if self.count == 0 { // 用于初始化的类型->很奇怪
            self.items.clear();
        }
        self.items.push(value);
        self.count+=1;
        if self.count >= 2 {
            // 执行交换操作
            let mut current_index = self.count;
            // let mut vec_list = &mut self.items; // &vec<T>
            while current_index >= 2 && (self.comparator)(&self.items[current_index-1],&self.items[self.parent_idx(current_index)-1]) {
                self.items.swap(current_index-1,current_index/2-1);
                current_index = current_index/2;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		// 返回两个子节点中更优先的那个
        let left_idx = self.left_child_idx(idx);
        let right_idx = left_idx + 1;
        if right_idx > self.count {
            return left_idx; // 只有一个节点
        } else {
            let target:bool = (self.comparator)(&self.items[left_idx-1],&self.items[right_idx-1]);
            if target {
                return left_idx;
            } else {
                return right_idx;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        //不使用vec的迭代器，手动声明->堆需要每次弹出根节点
        // return Some(self.items.remove(0));
        if self.is_empty() {
            return None;
        } else {
            // 下沉调整
            let root = self.items.remove(0);
            self.count-=1;
            match self.items.pop() {
                None => {return Some(root);}
                Some(value) => {
                    self.items.push(value); // 将底部的元素放在顶部->下沉调整
                    let mut current_index = 1;
                    while self.children_present(current_index) { // 子节点存在
                        // 选取子节点中更优先的进行交换
                        let better_child_idx = self.smallest_child_idx(current_index);
                        if (self.comparator)(&self.items[better_child_idx-1],&self.items[current_index-1]) {
                            self.items.swap(better_child_idx-1,current_index-1);
                        } else {
                            break;
                        }
                        current_index = better_child_idx;
                    }
                    return Some(root);
                }
            }
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}