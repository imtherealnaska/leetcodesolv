pub struct KarpRabin {
    base: u64,
    prime: u64,
}

impl KarpRabin {
    pub fn new(base: u64, prime: u64) -> Self {
        KarpRabin { base, prime }
    }

    fn calculate_hash(&self, text: &[u8], length: usize) -> u64 {
        let mut hash_value = 0;

        for i in 0..length {
            hash_value = (hash_value * self.base + text[i] as u64) % self.prime;
        }
        hash_value
    }

    pub fn recalculate_hash(&self, old_hash: u64, old_char: u8, new_char: u8, h: u64) -> u64 {
        ((old_hash + self.prime - (old_char as u64 * h) % self.prime) * self.base + new_char as u64)
            % self.prime
    }

    pub fn search(&self, pattern: &[u8], text: &[u8]) -> Vec<usize> {
        let mut matches = Vec::new();
        let pattern_len = pattern.len();
        let text_len = text.len();

        if pattern_len > text_len {
            return matches;
        }

        let pattern_hash = self.calculate_hash(pattern, pattern_len);
        let mut text_hash = self.calculate_hash(text, pattern_len);

        let mut h = 1;
        for _ in 0..pattern_len - 1 {
            h = (h * self.base) % self.prime;
        }

        for i in 0..=text_len - pattern_len {
            if pattern_hash == text_hash {
                let mut match_found = true;
                for j in 0..pattern_len {
                    if text[i + j] != pattern[j] {
                        match_found = false;
                        break;
                    }
                }
                if match_found {
                    matches.push(i);
                }
            }

            if i < text_len - pattern_len {
                text_hash = self.recalculate_hash(text_hash, text[i], text[i + pattern_len], h);
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karp_rabin() {
        let kr = KarpRabin::new(256, 101);
        let text = b"AABAACAADAABAAABAA";
        let pattern = b"AABA";
        let matches = kr.search(pattern, text);
        assert_eq!(matches, vec![0, 9, 13]);

        let text = b"ABCDEFG";
        let pattern = b"XYZ";
        let matches = kr.search(pattern, text);
        assert_eq!(matches, vec![]);

        let text = b"ABC";
        let pattern = b"ABCD";
        let matches = kr.search(pattern, text);
        assert_eq!(matches, vec![]);
    }
}
