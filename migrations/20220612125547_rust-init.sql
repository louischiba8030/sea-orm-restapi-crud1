CREATE TABLE posts (
	ulid VARCHAR(26) NOT NULL PRIMARY KEY,
	title VARCHAR(255) NOT NULL,
	author VARCHAR(255) NOT NULL,
	pages INT NOT NULL,
	publisher VARCHAR(255) NOT NULL,
	isbn13 CHAR(14) NOT NULL
) CHARACTER SET utf8mb4;
