-- Your SQL goes here
create table users (
       id serial primary key, -- must use integer to autoincrement
       username text not null
);

create table threads (
       id serial primary key,
       slug text not null
);

create table comments (
       id serial primary key,
       thread int not null,
       author int not null,
       content text not null
);
