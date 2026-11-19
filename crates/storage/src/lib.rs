#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

use std::{fs, path::Path, time::Duration};

use rusqlite::{Connection, params};
use thiserror::Error;
use todocraft_core::{Task, TaskList, Workspace};

const SCHEMA: &str = "
    CREATE TABLE IF NOT EXISTS lists (
        id TEXT PRIMARY KEY NOT NULL,
        payload TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS tasks (
        id TEXT PRIMARY KEY NOT NULL,
        list_id TEXT NOT NULL,
        payload TEXT NOT NULL,
        FOREIGN KEY(list_id) REFERENCES lists(id)
    );
";

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("File access failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("SQLite failure: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Invalid persisted task data: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Tasks refer to an unknown list; original data was preserved")]
    MissingList,
}

pub struct Store {
    connection: Connection,
}

impl Store {
    pub fn open(path: &Path) -> Result<Self, StoreError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let connection = Connection::open(path)?;
        Self::initialize(connection)
    }

    pub fn load(&self) -> Result<Workspace, StoreError> {
        let mut lists = Vec::new();
        let mut tasks = Vec::new();
        {
            let mut statement = self.connection.prepare("SELECT payload FROM lists ORDER BY rowid")?;
            let mut rows = statement.query([])?;
            while let Some(row) = rows.next()? {
                let payload: String = row.get(0)?;
                lists.push(serde_json::from_str::<TaskList>(&payload)?);
            }
        }
        {
            let mut statement = self.connection.prepare("SELECT payload FROM tasks ORDER BY rowid")?;
            let mut rows = statement.query([])?;
            while let Some(row) = rows.next()? {
                let payload: String = row.get(0)?;
                tasks.push(serde_json::from_str::<Task>(&payload)?);
            }
        }
        if lists.is_empty() && tasks.is_empty() {
            return Ok(Workspace::default());
        }
        if lists.is_empty() || tasks.iter().any(|task| !lists.iter().any(|list| list.id == task.list_id)) {
            return Err(StoreError::MissingList);
        }
        Ok(Workspace { lists, tasks })
    }

    pub fn save(&mut self, workspace: &Workspace) -> Result<(), StoreError> {
        if workspace.tasks.iter().any(|task| !workspace.lists.iter().any(|list| list.id == task.list_id)) {
            return Err(StoreError::MissingList);
        }
        let transaction = self.connection.transaction()?;
        transaction.execute("DELETE FROM tasks", [])?;
        transaction.execute("DELETE FROM lists", [])?;
        {
            let mut statement = transaction.prepare("INSERT INTO lists (id, payload) VALUES (?1, ?2)")?;
            for list in &workspace.lists {
                let payload = serde_json::to_string(list)?;
                statement.execute(params![list.id.to_string(), payload])?;
            }
        }
        {
            let mut statement = transaction.prepare(
                "INSERT INTO tasks (id, list_id, payload) VALUES (?1, ?2, ?3)"
            )?;
            for task in &workspace.tasks {
                let payload = serde_json::to_string(task)?;
                statement.execute(params![task.id.to_string(), task.list_id.to_string(), payload])?;
            }
        }
        transaction.commit()?;
        Ok(())
    }

    fn initialize(connection: Connection) -> Result<Self, StoreError> {
        connection.busy_timeout(Duration::from_secs(5))?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.execute_batch(SCHEMA)?;
        Ok(Self { connection })
    }

    #[cfg(test)]
    fn in_memory() -> Result<Self, StoreError> {
        Self::initialize(Connection::open_in_memory()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_preserves_lists_and_task_details() -> Result<(), Box<dyn std::error::Error>> {
        let mut store = Store::in_memory()?;
        let mut workspace = Workspace::default();
        let inbox = workspace.inbox_id().ok_or("no default list")?;
        let task_id = workspace.add_task(inbox, "Write release notes")?;
        let task = workspace.task_mut(task_id)?;
        task.important = true;
        task.add_step("Check tests")?;
        task.set_note("Keep this data offline")?;
        store.save(&workspace)?;
        assert_eq!(store.load()?, workspace);
        Ok(())
    }

    #[test]
    fn second_save_replaces_only_the_current_snapshot() -> Result<(), Box<dyn std::error::Error>> {
        let mut store = Store::in_memory()?;
        let mut workspace = Workspace::default();
        let inbox = workspace.inbox_id().ok_or("no default list")?;
        workspace.add_task(inbox, "One")?;
        store.save(&workspace)?;
        workspace.tasks.clear();
        store.save(&workspace)?;
        assert!(store.load()?.tasks.is_empty());
        Ok(())
    }
}
