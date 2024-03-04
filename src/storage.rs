mod memory;

use crate::{KvError, Kvpair, Value};
pub use memory::MemTable;

pub trait Storage {
    /// Retrieves the value associated with the given key from the specified table.
    ///
    /// # Arguments
    /// * `table` - The name of the table to perform the lookup in.
    /// * `key` - The key to retrieve the value for.
    ///
    /// # Returns
    /// A `Result` which is either:
    /// - `Ok(Some(Value))` if the key exists,
    /// - `Ok(None)` if the key does not exist,
    /// - `Err(KvError)` if an error occurs during the lookup.
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    /// Inserts or updates a key-value pair in the specified table.
    ///
    /// # Arguments
    /// * `table` - The name of the table to insert or update the key-value pair in.
    /// * `key` - The key associated with the value.
    /// * `value` - The value to set for the given key.
    ///
    /// # Returns
    /// A `Result` which is either:
    /// - `Ok(Some(Value))` containing the old value if the key existed,
    /// - `Ok(None)` if the key was not previously present,
    /// - `Err(KvError)` if an error occurs during the insertion or update.
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;

    /// Checks if the specified table contains the given key.
    ///
    /// # Arguments
    /// * `table` - The name of the table to check for the key in.
    /// * `key` - The key to check the existence of.
    ///
    /// # Returns
    /// A `Result` which is either:
    /// - `Ok(true)` if the key exists,
    /// - `Ok(false)` if the key does not exist,
    /// - `Err(KvError)` if an error occurs during the check.
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;

    /// Removes a key-value pair from the specified table, if it exists.
    ///
    /// # Arguments
    /// * `table` - The name of the table to remove the key-value pair from.
    /// * `key` - The key to remove along with its associated value.
    ///
    /// # Returns
    /// A `Result` which is either:
    /// - `Ok(Some(Value))` containing the value of the removed key if it existed,
    /// - `Ok(None)` if the key was not present,
    /// - `Err(KvError)` if an error occurs during the removal.
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    /// Get the iterator of all kv-pairs
    ///
    /// # Arguments
    /// * `table` - The name of the table to iterate over
    ///
    /// # Returns
    /// A `Result` which is either:
    /// - `Ok(Box<dyn Iterator<Item = Kvpair>>)` with the iterator,
    /// - Err(KvError)` if an error occurs while obtaining the iterator.
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    // #[test]
    // fn memtable_get_iter_should_work() {
    //     let store = MemTable::new();
    //     test_get_iter(store);
    // }

    fn test_basic_interface(store: impl Storage) {
        // the first time to insert kv pair to a table. should create the table, insert the kv pair, and return None.
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());

        // set new value to the same key. should get the previous value
        let v1 = store.set("t1", "hello".into(), "today".into());
        assert_eq!(v1, Ok(Some("world".into())));

        // should get the latest value
        let v = store.get("t1", "hello");
        assert_eq!(v, Ok(Some("today".into())));

        // should get None if using a nonexist table or key
        assert_eq!(Ok(None), store.get("t1", "hello1"));
        assert!(store.get("t2", "hello1").unwrap().is_none());

        // return true if contains the key in the table. otherwise, return false
        assert_eq!(store.contains("t1", "hello"), Ok(true));
        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));

        // delete an existing key from the table
        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(Some("today".into())));
        assert_eq!(store.contains("t1", "hello"), Ok(false));
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();
        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        )
    }
}
