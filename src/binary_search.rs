fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] == target {
            return m as i32;
        } else if nums[m] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    -1
}

pub fn main() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 9;
    println!("{}", search(nums, target));
}
