/// 计算梅森哈希
/// 具体过程：
///     将`seed`中的每个字符的ASCII值和对应下标值加1相乘，用得到的值对梅森素数127取余，然后对余数求3次幂，将求得的值作为seed的哈希值.
///
pub fn mersenne_hash(seed: &str) -> usize {
    let mut hash: usize = 0;
    for (i, c) in seed.chars().enumerate() {
        hash += (i + 1) * (c as usize);
        // println!("{}: {}, hash = {}", i, c as usize, hash);
    }
    (hash % 127).pow(3) - 1
}

#[cfg(test)]
mod tests {
    use crate::merhash::mersenne_hash;

    #[test]
    fn mersenne_hash_works() {
        let seed = String::from("jdxjp"); // 106,100,120,106,112
        let hash = mersenne_hash(&seed); // 106*1+100*2+120*3+106*4+112*5=1650 1650%127 = 126 126^3-1=2000375
        assert_eq!(2000375, hash);
    }
}
