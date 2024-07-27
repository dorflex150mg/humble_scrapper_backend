use rusqlite::Connection;
//use anyhow::Result;
use std::fmt;
use thiserror::Error;


use crate::model::item::Item;
use crate::model::agent::Agent;

//use crate::Agent;
//use crate::Item;


pub struct DbHandle {
    connection: Connection,
    name: String,
}

#[derive(Error, Debug, derive_more::From, derive_more::Display)]
pub enum QuerryError {
    RusqliteError(rusqlite::Error),
    EmptyTableErrorW(EmptyTableError),
}

#[derive(Error, Debug)]
pub enum EmptyTableError {
    NoItems,
    NoAgents,
}

impl fmt::Display for EmptyTableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmptyTableError::NoItems => write!(f, "No Items in the database.\n Consider inserting items with push_item(id: String, name: String, price: f64)"),
            EmptyTableError::NoAgents => write!(f, "No Agents in the database.\n Consider creating agents with Agent::new()"),
        }
    }
}

fn create_tables(connection: Connection) -> Result<Connection, rusqlite::Error> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS item(
            id TEXT PRIMARY KEY, 
            name TEXT NOT NULL, 
            price REAL NOT NULL
        );",
        []
    )?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS agent(
            id TEXT PRIMARY KEY
        );",
        []
    )?;
    Ok(connection)
}

impl DbHandle {
    pub fn new(name: String) -> Result<Self, rusqlite::Error> {
        let mut connection = Connection::open(&name)?;
        connection = create_tables(connection)?;
        Ok(DbHandle {
            connection,
            name,
        })
    }

    pub fn push_item(&self, id: String, name: String, price: f64) -> Result<String, rusqlite::Error> {
        self.connection.execute(
            &format!("INSERT INTO item(id, name, price) VALUES({}, {}, {}));",
                id.clone(),
                name,
                price,
            ),
            [],
        )?;
        Ok(id)
    }

    pub fn get_item(&self, id: String) -> Result<Item, QuerryError> {
        let mut query = self.connection.prepare("SELECT name, price FROM item WHERE id = ?1;")?;
        let mut rows = query.query(rusqlite::params![id])?;
        let maybe_row = rows.next()?;
        let row = maybe_row.ok_or(EmptyTableError::NoItems)?;
        Ok(Item {
            id, 
            name: row.get(1)?,
            price: row.get(2)?,
        })
    }

    pub fn push_agent(&self, id: String) -> Result<String, rusqlite::Error> {
        self.connection.execute(
            &format!("INSERT INTO agent(id) VALUES({})", id.clone()),
            [],
        )?;
        Ok(id)
    }
        
    pub fn get_agents(&self) -> Result<Vec<Agent>, QuerryError> {
        let mut query = self.connection.prepare("SELECT id FROM agent")?;
        let agents_iter = query.query_map([], |row| {
            Ok(Agent {
                id: row.get(0)?
              })
        })?;
        let mut agents: Vec<Agent> = vec![];
        for maybe_agent in agents_iter {
            let agent = maybe_agent.or(Err(EmptyTableError::NoAgents))?;
            agents.push(agent);
        }
        Ok(agents)
    }

}

    

