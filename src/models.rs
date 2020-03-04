use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::schema::{users,invitations};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime, // only NaiveDateTime works here due to diesel limitations
}


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: NaiveDateTime,
}

/// This is db executor actor. can be run in parallel
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);


// Actors communicate exclusively by exchanging messages. 
// The sending actor can optionally wait for the response. 
// Actors are not referenced directly, but by means of addresses.
// Any rust type can be an actor, it only needs to implement the Actor trait.
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}



impl User {
    // this is just a helper function to remove password from user just before we return the value out later
    pub fn remove_pwd(mut self) -> Self {
        self.password = "".to_string();
        self
    }
}