/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.// 排序算法
*/
// I AM DONE

fn sort<T>(array: &mut [T]) where T:PartialOrd+PartialEq { // 添加泛型限制
	//TODO
    // 使用冒泡排序的方式
    loop {
        let mut Target:bool = false; // 决定要不要迭代
        // 只要存在一次的交换则置为true，为false则没问题
        if array.len() == 1 {
            break;
        } else {
            for index in 0..array.len()-1 {
                let left_num:&T = &array[index];
                let right_num:&T = &array[index+1];
                if left_num>right_num {
                    // 进行交换->使用swap不需要泛型特征
                    array.swap(index,index+1);
                    Target = true; // 标记为交换过
                }
            }
            // 全部交换完毕
            if !Target {
                break;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}