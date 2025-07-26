use crate::storage::Result;
use borsh::{BorshDeserialize, BorshSerialize};

pub trait Storage<K, V>
where
    K: Ord + Clone + BorshSerialize + BorshDeserialize,
    V: Clone + BorshSerialize + BorshDeserialize,
{
    fn put(&self, key: K, value: V) -> Result<()>;
    fn get(&self, key: &K) -> Option<V>;
    fn delete(&self, key: &K) -> bool;
}
