use crate::{errors::Error, repository::RepoInfo};
use redb::{Database, ReadableTable, TableDefinition};

// key is hex pubkey value is name
const NAMESTABLE: TableDefinition<&str, &str> = TableDefinition::new("names");
// key is the event id of publish repo value is serialized repo info
const REPOSTABLE: TableDefinition<&str, &str> = TableDefinition::new("Repos");

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
            // Opens the table to create it
            let _ = write_txn.open_table(NAMESTABLE).unwrap();
            let _ = write_txn.open_table(REPOSTABLE).unwrap();
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

    pub fn read_name(&self, pubkey: &str) -> Result<String, Error> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(NAMESTABLE)?;
        if let Some(name) = table.get(pubkey)? {
            return Ok(name.to_string());
        }
        Err(Error::MissingValue)
    }

    pub fn write_repo_info(&mut self, repo_info: &RepoInfo) -> Result<(), Error> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(REPOSTABLE)?;
            table.insert(&repo_info.id, &serde_json::to_string(repo_info)?)?;
        }
        write_txn.commit()?;
        Ok(())
    }

    pub fn read_repo_info(&self, id: &str) -> Result<RepoInfo, Error> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(REPOSTABLE)?;
        if let Some(repo_info) = table.get(id)? {
            return Ok(serde_json::from_str(repo_info)?);
        }
        Err(Error::MissingValue)
    }
}
