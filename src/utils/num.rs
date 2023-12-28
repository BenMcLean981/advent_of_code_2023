pub fn multiple_lcm(nums: Vec<u64>) -> u64 {
    let mut result = nums[0];

    for num in nums.iter().skip(1) {
        result = lcm(result, *num);
    }

    return result;
}

pub fn lcm(a: u64, b: u64) -> u64 {
    return a * b / gcd(a, b);
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}
