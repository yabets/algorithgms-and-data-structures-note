    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = vec![0; nums.len()+1];
        for i in 0..nums.len() {
            sum[i+1] = sum[i] + nums[i];
        }
        println!("{:?}", sum);
        for i in 0..nums.len() {
            for j in i..nums.len() {
                println!("{i} to {j} | {} - {} = {}", sum[j+1], sum[i], sum[j+1] - sum[i]);
                if sum[j+1] - sum[i] == k {
                    count += 1;
                }
            }
        }

        count
    }

fn main() {
    let nums = vec![1,1,1];
    let k = 2;
    println!("{}", subarray_sum(nums, k));
}