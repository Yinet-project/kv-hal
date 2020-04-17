use async_trait::async_trait;
use std::convert::AsRef;

/// the trait for storage.
#[async_trait]
pub trait Storage {
    /// Error of storage low level impl.
    type Error;

    /// Value stored in low level
    type Value: AsRef<[u8]>;

    /// set field.
    async fn field<F>(&mut self, field: F)
    where
        F: AsRef<[u8]> + Send;

    /// set value
    async fn set<K, V>(&mut self, key: K, value: V) -> Result<Option<Self::Value>, Self::Error>
    where
        K: AsRef<[u8]> + Send,
        V: AsRef<[u8]> + Send;

    /// get value
    async fn get<K>(&self, key: K) -> Result<Option<Self::Value>, Self::Error>
    where
        K: AsRef<[u8]> + Send;

    /// delete value
    async fn del<K>(&mut self, key: K) -> Result<Option<Self::Value>, Self::Error>
    where
        K: AsRef<[u8]> + Send;
}

