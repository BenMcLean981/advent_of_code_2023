pub fn hash(s: &str) -> u64 {
    let ascii = s.chars().map(|c| c as u64);

    return ascii.fold(0, |hash, code| (hash + code) * 17 % 256);
}
