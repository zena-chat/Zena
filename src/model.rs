//! The (rough) data model. This is all the data that is stored in the database
//! and essentially defines what the app can do and store.

// NOTE(Josh): need to look into SQLite types a bit more but it is my understanding that all sqlite
// ints are 64 bit signed ints. we can cast using FromSql and ToSql later.
pub type ChannelId = i64;
pub type UserId = i64;
pub type MessageId = i64;

#[derive(Debug)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    /// when the channel was originally created as a unix epoch
    pub created_at: i64,
    /// when the channel was last modified as a unix epoch
    pub updated_at: i64,
}

pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: Option<String>,
    // password: we don't query the password out
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted: bool,
}

pub struct Message {
    pub id: MessageId,
    pub author_id: UserId,
    pub channel_id: ChannelId,
    /// when the message was originally received by the server as a unix epoch
    pub created_at: i64,
    /// when the message was last modified as a unix epoch
    pub updated_at: i64,
    pub deleted: bool,
}
