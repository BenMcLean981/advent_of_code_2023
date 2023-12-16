pub fn hash(s: &str) -> u32 {
    let ascii = s.chars().map(|c| c as u32);

    return ascii.fold(0, |hash, code| (hash + code) * 17 % 256);
}
