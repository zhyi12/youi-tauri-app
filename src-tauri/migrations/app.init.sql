CREATE TABLE IF NOT EXISTS youi_desktop_config
(
    id    INTEGER      not null primary key autoincrement,
    name  VARCHAR(100) not null,
    value VARCHAR(200) not null
);