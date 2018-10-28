fn main() {
    println!(" ");
    println!("Merge Sort in Rust");
    println!("==================");
    println!(" ");

    let mut nums = vec![3,1,6,2,4,7,5];
    println!("Unsorted: {:?}", nums);
    merge_sort(&mut nums);
    println!("Sorted: {:?}", nums);
}

fn merge_sort(nums: &mut std::vec::Vec<usize>) { 
    if nums.len() > 1 {
        let mid: usize = nums.len()/2;
        let mut left: Vec<usize> = nums[0..mid].to_vec();
        let mut right: Vec<usize> = nums[mid..nums.len()].to_vec();

        merge_sort(&mut left);
        merge_sort(&mut right);

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k: usize = 0;

        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                nums[k] = left[i];
                i += 1;
            } else {
                nums[k] = right[j];
                j += 1;
            }
            k += 1;
        }

        while i < left.len() {
            nums[k] = left[i];
            i += 1;
            k += 1;
        }

        while j < right.len() {
            nums[k] = right[j];
            j += 1;
            k += 1;
        }
        
        
    }
}