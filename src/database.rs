// Database module - SQLite highscore management

use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighScore {
    pub id: Option<i32>,
    pub player_name: String,
    pub score: u32,
    pub level_reached: usize,
    pub timestamp: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS highscores (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_name TEXT NOT NULL,
                score INTEGER NOT NULL,
                level_reached INTEGER NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Database { conn })
    }
    
    pub fn insert_score(&self, score: &HighScore) -> Result<()> {
        self.conn.execute(
            "INSERT INTO highscores (player_name, score, level_reached, timestamp)
             VALUES (?1, ?2, ?3, ?4)",
            [
                &score.player_name,
                &score.score.to_string(),
                &score.level_reached.to_string(),
                &score.timestamp,
            ],
        )?;
        Ok(())
    }
    
    pub fn get_top_scores(&self, limit: usize) -> Result<Vec<HighScore>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, player_name, score, level_reached, timestamp
             FROM highscores
             ORDER BY score DESC
             LIMIT ?1",
        )?;
        
        let scores = stmt.query_map([limit], |row| {
            Ok(HighScore {
                id: Some(row.get(0)?),
                player_name: row.get(1)?,
                score: row.get(2)?,
                level_reached: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })?;
        
        let mut result = Vec::new();
        for score in scores {
            result.push(score?);
        }
        Ok(result)
    }
    
    pub fn is_high_score(&self, score: u32) -> Result<bool> {
        let top_scores = self.get_top_scores(10)?;
        
        if top_scores.len() < 10 {
            return Ok(true);
        }
        
        Ok(top_scores.last().map_or(true, |s| score > s.score))
    }
}
