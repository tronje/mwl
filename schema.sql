drop table if exists users;
create table users (
    id integer unique primary key autoincrement,
    name text unique not null,
    password text not null,
    session_token text unique
);
