// This a list of different sorting algorithms done in Rust
// This is just meant as a simple exercise, so the implementations might not be good
// Values used in testing are fixed but were randomly generated, I wanted to avoid adding any dependencies if possible

pub fn insertion_sort<T: Ord + Copy>(values: &[T])->Vec<T>{
    let mut result = Vec::new();

    for &item in values{
        let mut target_index = 0;
        while target_index < result.len() && result[target_index] < item{
            target_index+=1;
        }
        result.insert(target_index, item);
    }
    result
}

// selection sort is in-place
pub fn selection_sort<T: Ord + Copy>(values: &mut [T]){
    for i in 0..values.len(){
        let mut min_idx = i;
        for j in i..values.len(){
            if values[j] < values[min_idx]{
                min_idx = j
            }
        }

        // Swaps the smallest value in slice with first value in slice
        let new_at_i = values[min_idx];
        values[min_idx] = values[i];
        values[i] = new_at_i;
    }
}

pub fn merge_sort<T: Ord + Copy>(values: &[T])->Vec<T>{
    if values.len() == 1{
        vec![values[0]]
    } else {
        let count = values.len();
        let left = merge_sort(&values[..count/2]);
        let mut left_idx = 0;

        let right = merge_sort(&values[count/2..]);
        let mut right_idx = 0;

        let mut result = Vec::new();
        while left_idx+right_idx < count{
            if left_idx == left.len(){
                for i in right_idx..right.len(){
                    result.push(right[i])
                }
                return result;
            }

            if right_idx == right.len(){
                for i in left_idx..left.len(){
                    result.push(left[i])
                }
                return result;
            }

            if right[right_idx] < left[left_idx]{
                result.push(right[right_idx]);
                right_idx+=1;
            } else {
                result.push(left[left_idx]);
                left_idx+=1;
            }
        }
        return result
    }
}

// Heap sort is in-place
pub fn heap_sort<T: Ord + Copy>(values: &mut [T]){
    // Build the heap
    for i in (0..values.len()).rev(){
        let mut root = i;
        while root < values.len(){
            let mut max_idx = root;
            if root*2+1 < values.len() && values[root] < values[root*2+1]{
                max_idx = root*2+1;
            }
            if root*2+2 < values.len() && values[max_idx] < values[root*2+2]{
                max_idx = root*2+2;
            }
            if max_idx != root{
                let tmp = values[root];
                values[root] = values[max_idx];
                values[max_idx] = tmp;
                root = max_idx;
            } else {
                break;
            }
        }
    }

    // For each value in the heap, swap the first and last, shrink heap by 1, then fix heap
    for i in (0..values.len()).rev(){
        let tmp = values[0];
        values[0] = values[i];
        values[i] = tmp;

        let mut root = 0;
        while root < i{
            let mut max_idx = root;
            if root*2+1 < i && values[root] < values[root*2+1]{
                max_idx = root*2+1;
            }
            if root*2+2 < i && values[max_idx] < values[root*2+2]{
                max_idx = root*2+2;
            }
            if max_idx != root{
                let tmp = values[root];
                values[root] = values[max_idx];
                values[max_idx] = tmp;
                root = max_idx;
            } else {
                break;
            }
        }

    }
}

#[cfg(test)]
mod tests{
    // Helper function that checks that an array is non-decreasing
    fn is_sorted<T: Ord>(values: &[T])->bool{
        for i in 1..values.len(){
            if values[i] < values[i-1]{
                return false
            }
        }
        return true;
    }

    // Make sure is_sorted is actually returning false
    #[test]
    fn test_is_sorted(){
        assert!(is_sorted(&[1,1,5,8,9]));
        assert!(!is_sorted(&[5,9,2]));
    }

    #[test]
    fn insertion_sort(){
        let unsorted = [92,41,98, 4, 2,23,17,55,16,64,33,20,78,40,59,19,81,80,36,45,28,62,1];
        let sorted = crate::insertion_sort(&unsorted);

        assert!(is_sorted(&sorted));
        assert_eq!(unsorted.len(), sorted.len()); // Make sure no values got dropped
    }

    #[test]
    fn selection_sort(){
        let mut values = [61,4,17,19,50,28,49,73,55,83,40,46,93,70,85,54,47,67,30,16,77,82,90];
        let start_len = values.len();

        crate::selection_sort(&mut values);

        assert!(is_sorted(&values));
        assert_eq!(start_len, values.len()); // Make sure no values got dropped
    }

    #[test]
    fn merge_sort(){
        let unsorted = [9,64,53,21,18,14,59,65,41,69,84,91,94,74,60,1,32,15,99,78,6,13,81,20,86];
        let sorted = crate::merge_sort(&unsorted);

        assert!(is_sorted(&sorted));
        assert_eq!(sorted.len(), unsorted.len());
    }

    #[test]
    fn heap_sort(){
        let mut values = [52,92,98,31,45,62,36,43,80,12,79,68,75,66,100,72,2,38,3,87,42,51,33,22,10,35,23,26,11,27,95,44,8,97];
        let length = values.len();

        crate::heap_sort(&mut values);

        assert!(is_sorted(&values));
        assert_eq!(length, values.len()); // Make sure no values got dropped
    }
}
