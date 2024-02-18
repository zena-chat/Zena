//! Database connection and queries

use rusqlite::{Connection, Result};

/// Holds a database connection
pub struct Db {
    conn: rusqlite::Connection,
}

impl Db {
    pub fn init() -> Result<Self> {
        let mut conn = Connection::open("client.db")?;

        migrations::MIGRATIONS.to_latest(&mut conn).unwrap(); // FIXME: handle error

        // TEMP: insert a channel so there's always some data even if you have a fresh db
        let general_channel = "General".to_string();
        conn.execute(
            "INSERT INTO channels (name) VALUES (?1)",
            [&general_channel],
        )?;

        Ok(Db { conn })
    }
}

pub mod queries {
    use crate::model::Channel;

    use super::Db;
    use rusqlite::Result;

    impl Db {
        pub fn get_all_channels(&self) -> Result<Vec<Channel>> {
            let mut stmt = self.conn.prepare(
                "SELECT channel_id, name, priority, created_at, updated_at FROM channels",
            )?;
            let channels: Vec<_> = stmt
                .query_map([], |row| {
                    Ok(Channel {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        priority: row.get(2)?,
                        created_at: row.get(3)?,
                        updated_at: row.get(4)?,
                    })
                })?
                .flatten() // we're ignoring errors for the time being
                .collect();
            Ok(channels)
        }

        pub fn create_channel(&self, name: String) -> Result<Channel> {
            self.conn
                .execute("INSERT INTO channels (name) VALUES (?1)", [&name])?;
            // FIXME
            Ok(Channel {
                id: 100,
                name,
                priority: 1,
                created_at: 999,
                updated_at: 999,
            })
        }

        pub fn delete_all_channels(&self) -> Result<()> {
            self.conn.execute("DELETE FROM channels;", ())?;
            Ok(())
        }
    }
}

mod migrations {
    use lazy_static::lazy_static;
    use rusqlite_migration::{Migrations, M};

    lazy_static! {
        pub static ref MIGRATIONS: Migrations<'static> = Migrations::new(vec![
            M::up(
                r#"
            CREATE TABLE channels (
                channel_id INTEGER PRIMARY KEY,
              
                -- updateable fields
                name TEXT NOT NULL,
                priority INTEGER NOT NULL DEFAULT 1,
              
                -- audit
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
              );
            "#
            )
            .down("DELETE FROM channels;"),
            M::up(
                r#"
            CREATE TABLE users (
                user_id INTEGER PRIMARY KEY,
              
                -- updateable fields
                username TEXT NOT NULL,
                email TEXT, -- optional
                password TEXT NOT NULL,
              
                -- audit
                created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
              
                -- soft delete
                deleted INTEGER NOT NULL DEFAULT 0
              );
            "#
            )
            .down("DELETE FROM users;"),
            M::up(
                r#"
            create table messages (
                msg_id integer primary key,
                author_id integer not null references users(user_id),
                channel_id integer not null references channels(channel_id),
                content text not null,
              
                -- audit
                created_at integer not null default (strftime('%s', 'now')),
                updated_at integer not null default (strftime('%s', 'now')),
              
                -- soft delete
                deleted integer not null default 0 -- 0 = not deleted, 1 = deleted
              );
            "#
            )
            .down("DELETE FROM messages;")
        ]);
    }
}
