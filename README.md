# Work in Progress

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

Our main objective is to buil a distributed web applications that are safe, efficient, highly performant, and do not "break the piggy bank" to operate and maintain.

With that in mind two key components were chosen to achieve those objectives:
<ul>
  <li><a href="https://actix.rs/docs" target="_self">Actix</a> is a modern, light-weight web framework written in <a href="https://github.com/rust-lang/rust" target="_self">Rust</a>.</li>
  <li><a href="https://actix.rs/docs](https://crates.io/crates/sqlx/" target="_self">SQLx</a> is a Rust crate that provides asynchronous database access in our case to postgres.</li>
</ul>

![image](https://user-images.githubusercontent.com/76512851/222997611-1fcbe954-99d5-4c68-914f-3a71b34eee6e.png)

### RESTful Web Service (library-service)

<hr>

### User Web Interface (library-ui)

<hr>
References:<br>

[Actix Web framework](https://actix.rs/docs)<br>
[SQLx](https://github.com/launchbadge/sqlx)<br>
[PostgreSQL](https://www.postgresql.org/docs/current/index.html)<br>
[Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)<br>
[Testing](https://doc.rust-lang.org/rust-by-example/testing.html)<br>
