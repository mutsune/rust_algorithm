use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

#[derive(Clone)]
struct Hash {
    state: RandomState,
}

impl Hash {
    fn new() -> Hash {
        Hash { state: RandomState::new() }
    }

    fn get(&self, key: u64) -> u64 {
        let mut hasher = self.state.build_hasher();
        hasher.write_u64(key);
        hasher.finish()
    }
}

pub struct BloomFilter {
    hashes: Vec<Hash>,
    filter: Vec<bool>,
}

impl BloomFilter {
    pub fn new(k: u64, m: u64) -> BloomFilter {
        BloomFilter {
            hashes: vec![Hash::new(); k as usize],
            filter: vec![false; m as usize],
        }
    }

    pub fn maybe_exist(&self, s: &str) -> bool {
        self.hashes.iter().all({
            |h| {
                *self.filter.get(self.convert_to_index(h.get(BloomFilter::str_to_u64(s))) as
                    usize).unwrap() == true
            }
        })
    }

    pub fn add(&mut self, s: &str) {
        for hash in &self.hashes {
            let index = &self.convert_to_index(hash.get(BloomFilter::str_to_u64(s)));
            self.filter[*index as usize] =
                true;
        }
    }

    fn str_to_u64(s: &str) -> u64 {
        s.chars().map({ |c| c as u64 }).sum::<u64>()
    }

    fn convert_to_index(&self, n: u64) -> u64 {
        n % self.filter.len() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bloom_filter() {
        let dict = vec![
            "Hochelaga",
            "anthropologist",
            "archantagonist",
            "assenting",
            "costectomy",
            "isleted",
            "raash",
            "repossession",
            "toffing",
            "uncriticising",
        ];
        let w0 = "costectomy";
        let w1 = "foo";

        let mut bloom_filter = BloomFilter::new(3, 30);
        for w in dict {
            bloom_filter.add(w);
        }
        println!("{:?}", bloom_filter.filter);

        assert!(bloom_filter.maybe_exist(w0));
        assert!(!bloom_filter.maybe_exist(w1));
    }
}
