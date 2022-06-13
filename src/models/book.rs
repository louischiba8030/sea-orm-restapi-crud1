// Define Book model

//use uuid::Uuid;
use ulid::Ulid;
use serde::{Deserialize, Serialize};
//use sqlx::{FromRow};
//use serde_json;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Book {
	pub ulid: String,
	pub title: String,
	pub author: String,
	pub pages: i32,
	pub publisher: String,
//	pub pub_date: NaiveDate,
	pub isbn13: String,
}
/*
#[derive(Insertable)]
pub struct NewBook {
	pub uuid: Uuid,
	pub title: String,
	pub author: String,
	pub pages: i32,
//	pub pub_date: Date,
	pub isbn13: String,
}
*/
