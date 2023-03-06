# Web Application (Work in Progress)

## Motivation

There is no "one-size-fits-all" for any technology and everything comes down to "what we are trying to solve?". For Web3, we are dealing with distributed applications working on distributed software infrastructure (P2P networks). In business terms this means we are going to lose money if we don't get the right trade-offs in our technology stack. For startups these technical choices and trade-offs will be crucial in their product development and ability to scale. Having a good idea is not enough for a startup to thrive.<br>
For the Web3 world, type safety (verifies and enforces type constraints at compile-time), high-performance async (good [ecosystem](https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html#the-async-ecosystem) of non-blocking  I/O libraries and runtimes), automatic memory management and memory safety without garbage collection ([ownership model](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)) were the main reasons that made [Rust](https://www.rust-lang.org/tools/install) my first choice. For instance, writing [smart contracts](https://use.ink/) that doesn't have memory bugs and consumes less storage on the blockchain is a massive advantage. On the other hand for UI building [React](https://reactjs.org/) is a better choice due the rich UI features offered.<br>

The example developed, will be focused on the back-end services and distributed software infrastructure demonstrating the advantages of using [Rust](https://github.com/rust-lang/rust) for the Web3 world.

<hr>

## Table of Contents<br>
<ul>
<li><a href="" target="_self">Web Application</a></li>
<ul>
<li><a href="https://github.com/gcp-development/web-application/blob/main/README.md#restful-web-service-library-service" target="_self">RESTful Web Service</a> (library-service)</li>
<li><a href="https://github.com/gcp-development/web-application#user-web-interface-library-ui" target="_self">User Web Interface</a> (library-ui)</li>
</ul>
</ul>
<hr>

## Web Application

Our main objective is to build a distributed web application that are safe, efficient, highly performant, and do not "break the piggy bank" to operate and maintain.

With that in mind two key components were chosen to achieve those objectives:
<ul>
  <li><a href="https://actix.rs/docs" target="_self">Actix</a> is a modern, light-weight web framework written in <a href="https://github.com/rust-lang/rust" target="_self">Rust</a>.</li>
  <li><a href="https://actix.rs/docs](https://crates.io/crates/sqlx/" target="_self">SQLx</a> is a Rust crate that provides asynchronous database access in our case to <a href="https://www.postgresql.org/download/" target="_self">postgres</a>.</li>
</ul>

When considering open-source software my first look is on the team building the software and the community using and maintaining it. (Makes no point selecting a state of the art software that is supported by a one man show. Believe Me or not, this is one of the most common errors startups do.)
After saying that, having asynchronous support, ability to scale and maturity is essential in any startup product development. Nevertheless, there are some very good alternatives options to the ones used for this demo. For a Rust web framework [axum](https://docs.rs/axum/latest/axum/) is also a sound choice and for the database access [SeaORM](https://www.sea-ql.org/SeaORM/docs/introduction/sea-orm/) (SeaORM is an object-relational mapper which SQLx is not).

We have the following environment setup using [kubernetes](https://github.com/gcp-development/web-application/tree/main/kubernetes-setup) for our web application:

![image](https://user-images.githubusercontent.com/76512851/223080940-6f847eb7-8cff-41eb-8571-3e5419391618.png)

Our web application is composed of a Rust Rest API connected to a postgres database which forms a [Microservice](https://microservices.io/) and a [React](https://reactjs.org/docs/getting-started.html) user web interface served by the Rust Rest API.

### RESTful Web Service (library-service)

The project code is organized with separate and clearly marked areas to store code for [handlers](https://github.com/gcp-development/web-application/tree/main/library-service/src/handlers), database access functions [(dal)](https://github.com/gcp-development/web-application/tree/main/library-service/src/dal), [data models](https://github.com/gcp-development/web-application/tree/main/library-service/src/model) and [database scripts](https://github.com/gcp-development/web-application/tree/main/library-service/src/model/sql_scripts). 

![image](https://user-images.githubusercontent.com/76512851/223094021-910d4695-d224-43bb-aade-ee255a0da1ce.png)

Each route has a handler function and normally a database access function. The main purpose of structuring our code is to make it easier for other people to read and support a CI/CD pipeline. For instance for this project all [tests](https://actix.rs/docs/testing/) are concentrated in the [handler section](https://github.com/gcp-development/web-application/tree/main/library-service/src/handlers). 

![image](https://user-images.githubusercontent.com/76512851/223129476-a0458994-0178-47d9-b978-c3429f548ad5.png)

Key points:

<ul>
  <li>Actix uses Async I/O, which enables an Actix web application to perform other tasks while waiting on I/O on a single thread. Actix has its own Async runtime that is based on <a href="https://tokio.rs/" target="_self">Tokio</a>(async library in Rust).</li>
  <li>Actix allows to define custom application <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/state.rs" target="_self">state</a>, and provides a mechanism to safely access this state from each handler function. Since each application instance of Actix runs in a separate thread, Actix provides a <a href="https://actix.rs/docs/application/#shared-mutable-state" target="_self">safe mechanism</a> to access and mutate this shared state without conflicts or data races.</li>
  <li>SQLx is built from the ground-up using async/await for maximum concurrency.</li>
  <li>The Postgres driver is written in pure Rust using zero unsafe code.</li>
</ul>

Service Handlers:
<ul>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L7" target="_self">post_add_book</a>Insert one book into the table <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql" target="_self">book<a/>.</li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L14" target="_self">post_bulk_insert</a>Insert books in bulk mode into the table <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql" target="_self">book<a/>.</li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L21" target="_self">get_books</a>Read all books from the table <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql" target="_self">book<a/>.</li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L28" target="_self">get_book_by_id</a></li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L39" target="_self">put_book_by_id</a></li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L51" target="_self">delete_book_by_id</a></li>
</ul>
  
<hr>

### User Web Interface (library-ui)

<hr>
References:<br>

[Actix Web framework](https://actix.rs/docs)<br>
[SQLx](https://github.com/launchbadge/sqlx)<br>
[PostgreSQL](https://www.postgresql.org/docs/current/index.html)<br>
[Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)<br>
[Testing](https://doc.rust-lang.org/rust-by-example/testing.html)<br>
