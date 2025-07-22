pub enum RecordMode {
    NoRecord,
    Record,
}

impl Into<bool> for RecordMode {
    fn into(self) -> bool {
        match self {
            Self::NoRecord => false,
            Self::Record => true,
        }
    }
}

impl From<bool> for RecordMode {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Record,
            false => Self::NoRecord,
        }
    }
}

pub trait Storage<K, V>
where
    K: Ord + Clone + Into<Vec<u8>> + From<Vec<u8>>,
    V: Clone + Into<Vec<u8>> + From<Vec<u8>>,
{
    fn put(&self, key: K, value: V, record: RecordMode);
    fn get(&self, key: &K) -> Option<V>;
    fn delete(&self, key: &K, record: RecordMode) -> bool;
}
