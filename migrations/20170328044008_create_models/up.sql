-- Your SQL goes here
create table users (
       id integer primary key not null, -- must use integer to autoincrement
       username text not null
);

create table threads (
       id integer primary key not null,
       slug text not null
);

create table comments (
       id integer primary key not null,
       thread int not null,
       author int not null,
       content text not null
);
