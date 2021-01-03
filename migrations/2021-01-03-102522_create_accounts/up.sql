-- Your SQL goes here
create table accounts(
    id varchar(32) primary key,
    items jsonb[] not null,
    debtors jsonb[] not null,
    name_ varchar(64) not null
);