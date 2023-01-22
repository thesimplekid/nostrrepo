use crate::{errors::Error, globals::GLOBALS};
use dashmap::DashMap;
use nostr_types::PublicKeyHex;
use portan::repository::RepoInfo;
use redb::{Database, ReadableTable, TableDefinition};
use tokio::task::spawn_blocking;

// key is hex pubkey value is name
const NAMESTABLE: TableDefinition<&str, &str> = TableDefinition::new("names");
// key is the event id of publish repo value is serialized repo info
const REPOSTABLE: TableDefinition<&str, &str> = TableDefinition::new("repos");

pub async fn setup_database() -> Result<(), Error> {
    let db = unsafe { Database::create("my_db.redb")? };
    let write_txn = db.begin_write()?;
    {
        // Opens the table to create it
        let _ = write_txn.open_table(NAMESTABLE)?;
        let _ = write_txn.open_table(REPOSTABLE)?;
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
    let _ = spawn_blocking(move || {
        let db = GLOBALS.db.blocking_lock();
        let write_txn = db.as_ref().expect("Missing DB").begin_write()?;
        {
            let mut table = write_txn.open_table(REPOSTABLE)?;
            table.insert(&repo_info.id, &serde_json::to_string(&repo_info).unwrap())?;
        }
        write_txn.commit()
    })
    .await?;
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

pub async fn read_name(pubkey: PublicKeyHex) -> Result<String, Error> {
    let name = spawn_blocking(move || {
        let db = GLOBALS.db.blocking_lock();
        let read_txn = db.as_ref().unwrap().begin_read()?;
        let table = read_txn.open_table(NAMESTABLE)?;
        if let Some(name) = table.get(&pubkey.to_string())? {
            return Ok(name.to_string());
        } else {
            Err(Error::MissingValue)
        }
    })
    .await??;
    Ok(name)
}

pub async fn add_names(names: DashMap<PublicKeyHex, String>) -> Result<(), Error> {
    let _ = spawn_blocking(move || {
        let db = GLOBALS.db.blocking_lock();
        let write_txn = db.as_ref().expect("Missing DB").begin_write()?;
        for (k, n) in names {
            let mut table = write_txn.open_table(NAMESTABLE)?;
            table.insert(&k.to_string(), &n)?;
        }
        write_txn.commit()
    })
    .await?;

    Ok(())
}
