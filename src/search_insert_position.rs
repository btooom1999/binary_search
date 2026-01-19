fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();

    while l < r {
        let m = l + (r - l) / 2;

        if nums[m] == target {
            return m as i32;
        } else if nums[m] < target {
            l = m + 1;
        } else {
            r = m;
        }
    }

    l as i32
}

pub fn main() {
    let nums = vec![1,3,4,5,6];
    let target = 7;
    println!("{}", search_insert(nums, target));
}
