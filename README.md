# Web Application (Work in Progress)

## Motivation

There is no "one-size-fits-all" for any technology and everything comes down to "what we are trying to solve?". For Web3, we are dealing with distributed applications working on distributed software infrastructure (P2P networks). In business terms this means we are going to lose money if we don't get the right trade-offs in our technology stack. For startups these technical choices and trade-offs will be crucial in their product development and ability to scale. Having a good idea is not enough for a startup to thrive.<br>
For the Web3 world, type safety (verifies and enforces type constraints at compile-time), high-performance async (good [ecosystem](https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html#the-async-ecosystem) of non-blocking  I/O libraries and runtimes), automatic memory management and memory safety without garbage collection ([ownership model](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)) were the main reasons that made [Rust](https://www.rust-lang.org/tools/install) my first choice. For instance, writing [smart contracts](https://use.ink/) that doesn't have memory bugs and consumes less storage on the blockchain is a massive advantage. On the other hand for user interface (UI) development, [React](https://reactjs.org/) is still a better choice due the rich UI features offered.<br>

The example developed, will be focused on the back-end services and distributed software infrastructure demonstrating the key points of using [Rust](https://github.com/rust-lang/rust) and [React](https://reactjs.org/) for the Web3 world.

<hr>

## Table of Contents<br>
<ul>
<li><a href="" target="_self">Web Application</a></li>
<ul>
<li><a href="https://github.com/gcp-development/web-application/blob/main/README.md#restful-web-service-library-service" target="_self">RESTful Web Service</a> (library-service)</li>
<li><a href="https://github.com/gcp-development/web-application#user-web-interface-library-ui" target="_self">User Web Interface</a> (library-ui)</li>
<li><a href="https://github.com/gcp-development/web-application#conclusion" target="_self">Conclusion</li></li>
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

Each route has a handler function and normally a database access function. The main purpose of structuring our code is to make it easier for other people to read and support a CI/CD pipeline. For instance for this project all [integration tests](https://actix.rs/docs/testing/) are concentrated in the [handler section](https://github.com/gcp-development/web-application/tree/main/library-service/src/handlers). 

![image](https://user-images.githubusercontent.com/76512851/223129476-a0458994-0178-47d9-b978-c3429f548ad5.png)

Key points:

<ul>
  <li>Actix uses async I/O, which enables an Actix web application to perform other tasks while waiting on I/O on a single thread. Actix has its own Async runtime that is based on <a href="https://tokio.rs/" target="_self">Tokio</a>(async library in Rust).</li>
  <li>Actix allows to define custom application <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/state.rs" target="_self">state</a>, and provides a mechanism to safely access this state from each handler function. Since each application instance of Actix runs in a separate thread, Actix provides a <a href="https://actix.rs/docs/application/#shared-mutable-state" target="_self">safe mechanism</a> to access and mutate this shared state without conflicts or data races.</li>
  <li>SQLx is built from the ground-up using async/await for maximum concurrency.</li>
  <li>The Postgres driver is written in pure Rust using zero unsafe code.</li>
</ul>

Service Handlers:
<ul>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L7" target="_self">post_add_book</a> Insert one book into the table <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql" target="_self">books<a/>.</li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L14" target="_self">post_bulk_insert</a> Insert books in bulk mode into the table <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql" target="_self">books<a/>.</li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L21" target="_self">get_books</a> Read all books from the table <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql" target="_self">books<a/>.</li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L28" target="_self">get_book_by_id</a> Read a book by id from the table <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql" target="_self">books<a/>.</li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L39" target="_self">put_book_by_id</a> Update a book by id from the table <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql" target="_self">books<a/>.</li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/handlers/book.rs#L51" target="_self">delete_book_by_id</a> Delete a book by id from table <a href="https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql" target="_self">books<a/>.</li>
</ul>

Before runing the integration tests we need to create the table [books](https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql) and run the script [test data](https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/2_testData.sql) using [pgAdmin](https://www.pgadmin.org/download/) for instance.

![image](https://user-images.githubusercontent.com/76512851/224683234-a3b08cd4-beda-4a4d-8dab-bec152056677.png)

<hr>

### User Web Interface (library-ui)

The project code is organized with separate and clearly marked areas to store code for [API functions](https://github.com/gcp-development/web-application/tree/main/library-ui/src/api), [components](https://github.com/gcp-development/web-application/tree/main/library-ui/src/components) and [types](https://github.com/gcp-development/web-application/tree/main/library-ui/src/types).

![image](https://user-images.githubusercontent.com/76512851/224685345-476db57f-017b-4d33-817f-c82abddf1916.png)

The [React version 18](https://reactjs.org/versions) was used to build this user interface(UI), because of the [concurrent mode](https://reactjs.org/blog/2022/03/29/react-v18.html#what-is-concurrent-react) that comes with this version. This new feature allows React to work on several state updates concurrently. 
For providing asynchronous state management, server-state utilities and data fetching, [TanStack Query](https://tanstack.com/query/latest) was used. Together with [React Router](https://reactrouter.com/en/main), which allow this UI to update the URL from a link click without making another request for another document from the server.

By integrating these two libraries and the use of the React V18, we get the following key benefits:
<ul>
  <li>Ability to prepare multiple versions of our UI at the same time.</li>
  <li>React Router’s data loader prevents an unnecessary re-render when data is loaded onto a page.</li>
  <li>React Query’s cache prevents unnecessary calls to the REST API.</li>
</ul>

SolidJS vs [React](https://www.solidjs.com/guides/comparison#react)

[SolidJS](https://www.solidjs.com/) is an open source, reactive declarative JavaScript library with an API similar to React. There is no such thing "one better than the other". Its always about tradeoff [(js-framework-benchmark)](https://krausest.github.io/js-framework-benchmark/current.html), we have to give up something to gain something else. In every decision made, as always, we should select two paths chose one and keep an eye on both.

Comparison Table
| Feature| SolidJS (2021) | React (2016) |
| ------------- | ------------- | ------------- |
| TypeScript support | = | = | 
| Declarative nature | = | = | 
| JSX Support | = | = |
| Highly performant | + | - |
| Direct manipulation of the DOM | Yes | No |
| Server-side rendering  | = | = |
| Conditional rendering | = | = |
| Concurrent rendering | = | = |
| Community and ecosystem | - | + |

Key points:
<ul>
  <li>
    React is a popular library for creating component-based frontends. React has the largest ecosystem out of any UI library, with very <a href="https://reactjs.org/blog/2022/06/15/react-labs-what-we-have-been-working-on-june-2022.html" target="_self">talent people</a> supporting it.
  </li>
  <li>
    TypeScript provides a much richer type system on top of JavaScript. TypeScript uses the type system to allow <a href="https://code.visualstudio.com/docs/languages/typescript target="_self">code editors</a> to catch type errors as developers write code. 
  </li>
  <li>
     <a href="https://reactjs.org/blog/2022/03/08/react-18-upgrade-guide.html#updates-to-client-rendering-apis" target="_self">Concurrent React</a>. Until React 18, the React render process was synchronous and non-interruptable. As a result, the UI would lock during long render processes and not respond immediately to user input.
  </li>
  <li>
     React Router reduces the number of re-renders.
  </li>
  <li>
     React Query(TanStack Query) provides a client-side cache of the data.
  </li>
  <li>
     <a href="https://www.solidjs.com/" target="_self">SolidJS</a> is a very solid alternative to React.
  </li>
</ul>

Running the React UI

The table [books](https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql) needs to be create and run the script [test data](https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/2_testData.sql) using [pgAdmin](https://www.pgadmin.org/download/) for instance.

![image](https://user-images.githubusercontent.com/76512851/224684760-c92e380b-d1b6-487b-9379-872a772b907f.png)

<hr>

### IPFS

<hr>

### Conclusion

In a life span of 5 years, [90%](https://www.forbes.com/sites/neilpatel/2015/01/16/90-of-startups-will-fail-heres-what-you-need-to-know-about-the-10/?sh=39fb5cda6679) of [technology startups fail](https://www.rocketspace.com/tech-startups/why-tech-startups-fail-and-how-founders-can-bounce-back). Despite of not being the only reason, the common denominator present in all of those, was/is the wrong technical choices made. Those choices limited the time to market, the ability to scale and most important time to innovate. 
The balancing between profitability and growth is extremely difficult to managed by itself. Without having to be limited in our actions by a software that does not scale, was poorly implemented and have an expensive operational cost. 

<hr>
References:<br>

[Actix Web framework](https://actix.rs/docs)<br>
[SQLx](https://github.com/launchbadge/sqlx)<br>
[PostgreSQL](https://www.postgresql.org/docs/current/index.html)<br>
[Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)<br>
[Testing](https://doc.rust-lang.org/rust-by-example/testing.html)<br>
[React TypeScript Cheatsheets](https://react-typescript-cheatsheet.netlify.app/)<br>
[Type Script Language](https://www.typescriptlang.org/)<br>

