use crate::errors::Error;
use redb::{Database, ReadableTable, TableDefinition};

const NAMESTABLE: TableDefinition<&str, &str> = TableDefinition::new("names");
pub struct PortanDb {
    db: Database,
}

impl Default for PortanDb {
    fn default() -> Self {
        Self::new()
    }
}

impl PortanDb {
    pub fn new() -> Self {
        let db = unsafe { Database::create("my_db.redb").unwrap() };
        let write_txn = db.begin_write().unwrap();
        {
            let mut _table = write_txn.open_table(NAMESTABLE).unwrap();
        }
        write_txn.commit().unwrap();

        Self { db }
    }

    pub fn write_name(&mut self, pubkey: &str, name: &str) -> Result<(), Error> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(NAMESTABLE)?;
            table.insert(pubkey, name)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn read_name(&self, pubkey: &str) -> Result<Option<String>, Error> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(NAMESTABLE)?;
        if let Some(name) = table.get(pubkey)? {
            return Ok(Some(name.to_string()));
        }
        Ok(None)
    }
}
