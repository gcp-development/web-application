CREATE TABLE IF NOT EXISTS public.books
(
    id serial primary key,
    title varchar(140) not null,
    author  varchar(140) not null,
    record_timestamp TIMESTAMP default now()
);