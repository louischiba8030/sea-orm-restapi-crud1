// Define Book model

use uuid::Uuid;

#[derive(Queryable)]
pub struct Book {
	pub uuid: Uuid,
	pub title: String,
	pub author: String,
	pub pages: i32,
	pub pub_data: Date,
	pub isbn-13: i32,
}

#[derive(Insertable)]
pub struct NewBook {
	pub uuid: Uuid,
	pub title: String,
	pub author: String,
	pub pages: i32,
	pub pub_data: Date,
	pub isbn-13: i32,
}
