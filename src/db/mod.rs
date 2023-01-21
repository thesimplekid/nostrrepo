use crate::{errors::Error, globals::GLOBALS};
use portan::repository::RepoInfo;
use redb::{Database, ReadableTable, TableDefinition};
use tokio::task::spawn_blocking;

// key is hex pubkey value is name
const NAMESTABLE: TableDefinition<&str, &str> = TableDefinition::new("names");
// key is the event id of publish repo value is serialized repo info
const REPOSTABLE: TableDefinition<&str, &str> = TableDefinition::new("repos");

pub async fn setup_database() -> Result<(), Error> {
    let db = unsafe { Database::create("my_db.redb").unwrap() };
    let write_txn = db.begin_write().unwrap();
    {
        // Opens the table to create it
        let _ = write_txn.open_table(NAMESTABLE).unwrap();
        let _ = write_txn.open_table(REPOSTABLE).unwrap();
    }
    write_txn.commit().unwrap();

    // Save the connection globally
    {
        let mut g_db = GLOBALS.db.lock().await;
        *g_db = Some(db);
    }

    Ok(())
}

pub async fn write_repo_info(repo_info: RepoInfo) -> Result<(), Error> {
    spawn_blocking(move || {
        let db = GLOBALS.db.blocking_lock();
        let write_txn = db.as_ref().unwrap().begin_write().unwrap();
        {
            let mut table = write_txn.open_table(REPOSTABLE).unwrap();
            table
                .insert(&repo_info.id, &serde_json::to_string(&repo_info).unwrap())
                .unwrap();
        }
        write_txn.commit().unwrap();
    })
    .await
    .unwrap();
    Ok(())
}

/*

pub fn read_repo_info(id: &str) -> Result<RepoInfo, Error> {
    let read_txn = self.db.begin_read()?;
    let table = read_txn.open_table(REPOSTABLE)?;
    if let Some(repo_info) = table.get(id)? {
        return Ok(serde_json::from_str(repo_info)?);
    }
    Err(Error::MissingValue)
}
*/
