use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

struct Hash {
    state: RandomState,
}

impl Hash {
    fn new() -> Hash {
        Hash { state: RandomState::new() }
    }

    fn get(&self, key: u64) -> u64 {
        let mut hasher = state.build_hasher();
        hasher.write_u64(key);
        hasher.finish()
    }
}

pub struct BloomFilter {
    hashes: Vec<Hash>,
    filter: Vec<bool>,
}

impl BloomFilter {
    pub fn new(k: u32, m: u32) -> BloomFilter {
        BloomFilter {
            hashes: vec![Hash::new(); k],
            filter: vec![false; m],
        }
    }

    pub fn maybe_exist(&self, s: &str) -> bool {
        self.hashes.iter().all({
            |h|
                filter[h.get(str_to_u64(s))] == true
        })
    }

    pub fn str_to_u64(s: &str) -> u64 {
        s.chars().map({ |c| c.to_digit(10).unwrap() }).sum()
    }

    pub fn add(&self, s: &str) {
        for hash in self.hashes {
            filter[convert_to_index(hash.get(self.str_to_u64(s)))] = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloom_filter() {
        let dict = vec![
            "Archidiskodon",
            "Pareioplitae",
            "Troezenian",
            "dauber",
            "hydroselenuret",
            "impishly",
            "mobocratical",
            "photechy",
            "stophound",
            "telacoustic"];
        let w0 = "dauber";
        let w1 = "foo";

        assert!(maybeExist(w0));
        assert!(!maybeExist(w1));
    }
}
