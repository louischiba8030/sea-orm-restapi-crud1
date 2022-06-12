// Define Book model

//use uuid::Uuid;
//use serde_json;

#[derive(sqlx::FromRow)]
pub struct Book {
	pub id: String,
	pub title: String,
	pub author: String,
	pub pages: i32,
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
