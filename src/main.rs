use sqlx::mysql::MySqlPoolOptions;
//use uuid::Uuid;

mod models;
//mod db;

use models::book::{Book};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Create a connection pool
	let pool = MySqlPoolOptions::new()
		.max_connections(5)
		.connect("mysql://dbuser:12ab@localhost:3306/rust_web_test2").await?;

	// Make a simple query to return the given parameter
	let rows = sqlx::query_as::<_, Book>("SELECT * FROM posts")
		.fetch_all(&pool)
		.await?;

//		assert_eq!(row.0, 464);
		println!("{:?}", rows.len());

		Ok(())
}