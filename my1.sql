USE rust_web_test2;

INSERT INTO posts(
uuid, title, author, pages, publisher, isbn13
)
VALUES(
UNHEX(REPLACE(UUID(), '-', '')),
"薔薇の名前〈上〉",
"ウンベルト エーコ",
413,
"東京創元社",
"978-4488013516"
);
