use super::Hash;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::super::u128_bytes;
    use super::Hashable;

    struct HashableObj {
        id: String,
        amount: u128,
    }
    impl Hashable for HashableObj {
        fn bytes(&self) -> Vec<u8> {
            let mut bytes = vec![];
            bytes.extend(self.id.as_bytes());
            bytes.extend(&u128_bytes(&self.amount));
            bytes
        }
    }

    #[test]
    fn test_hash() {
        let data = HashableObj {
            id: String::from("0x00"),
            amount: 1000000,
        };
        assert_ne!(data.bytes(), vec![0]);
    }
}
