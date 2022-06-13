use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::fs;
use sqlx::prelude::*;
use sqlx::mysql::MySqlPoolOptions;

use ulid::Ulid;
//mod models;

//use crate::models::book::{Book};
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Book {
//	pub ulid: String,
	pub title: String,
	pub author: String,
	pub pages: i32,
	pub publisher: String,
//	pub pub_date: NaiveDate,
	pub isbn13: String,
}

#[tokio::main]
async fn main () -> anyhow::Result<()> {
	// Read from json
	let file_name = "initial_books.json";
	let content = fs::read_to_string(file_name)
		.expect("Failed to load JSON");
	
	let deserialized: Vec<Book> = serde_json::from_str(&content).unwrap();
	for v1 in deserialized {
		register_todb(v1).await?;
	}

	Ok(())
}

async fn register_todb(v1: Book) -> anyhow::Result<()> {
	// Create a connection pool
	let mut pool = MySqlPoolOptions::new()
		.max_connections(5)
		.connect("mysql://dbuser:12ab@localhost:3306/rust_web_test2")
		.await?;

	// Generate a ulid
	let ulid = Ulid::new();

	let sql = "INSERT INTO posts (
		ulid,
		title,
		author,
		pages,
		publisher,
		isbn13
	) VALUES (?, ?, ?, ?, ?, ?)";
	let account = sqlx::query(sql)
		.bind(ulid.to_string()) // ulid
		.bind(v1.title)
		.bind(v1.author)
		.bind(v1.pages)
		.bind(v1.publisher)
		.bind(v1.isbn13)
		.execute(&pool)
		.await?;

	Ok(())
}

fn type_of<T>(_: T) -> String {
	let a = std::any::type_name::<T>();

	return a.to_string();
}
