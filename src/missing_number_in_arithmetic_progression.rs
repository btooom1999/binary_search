fn missing_number(mut arr: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = arr.len();
    let d = (arr[r-1] - arr[l]) / r as i32;

    while l < r {
        let m = (r + l) / 2;
        if arr[m] == arr[0] + m as i32 * d {
            l = m + 1;
        } else {
            r = m;
        }
    }

    arr[0] + d * l as i32
}

pub fn main() {
    let arr = vec![15,13,12];
    println!("{}", missing_number(arr));
}
