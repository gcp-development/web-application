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
<li><a href="https://github.com/gcp-development/web-application/blob/main/README.md#ipfs" target="_self">InterPlanetary File System</a> (IPFS)</li>
</ul>
<li><a href="https://github.com/gcp-development/web-application#conclusion" target="_self">Conclusion</a></li>
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

InterPlanetary File System [(IPFS)](http://docs.ipfs.tech/concepts/what-is-ipfs/) is a peer-to-peer distributed file system, which connect all computing devices with the same system of files.
IPFS is the combinational technology of the version controlling system and peer to peer network spreadover global namespace.

In contrast to a central storage(like PostgreSQL), IPFS works on a decentralized system in which every user in the network holds a portion of the overall data, thus creating a resilient system for storage and sharing over the globe. Any user in the network is able to share a file and it will be accessible to everyone in the network by requesting it from a node which possesses it using the Distributed Hash Table (DHT).

Three fundamental principles to understanding IPFS:
<ul>
<li><a href="https://github.com/gcp-development/web-application#unique-identification-via-content-addressing" target="_self">Unique identification via content addressing(CID)</a></li>
<li><a href="https://github.com/gcp-development/web-application#content-linking-via-directed-acyclic-graphs-dags" target="_self">Content linking via directed acyclic graphs (DAGs)</a></li>
<li><a href="https://github.com/gcp-development/web-application/blob/main/README.md#content-discovery-via-distributed-hash-tables-dhts" target="_self">Content discovery via distributed hash tables (DHTs)</a></li>
</ul>

The examples provided were developed using the [Kubo RPC API](https://docs.ipfs.tech/reference/kubo/rpc/#kubo-rpc-api-v0-reference) together with the [actix-web](https://docs.rs/actix-web/latest/actix_web/) ,[awc](https://crates.io/crates/awc) and [actix-multipart-rfc7578](https://crates.io/crates/actix-multipart-rfc7578) crates.

<hr>
      
#### Unique identification via content addressing
      
The [CID](https://docs.ipfs.tech/concepts/content-addressing/#content-identifiers-cids) is a label used to point to material in IPFS. Import the [book1.json](https://github.com/gcp-development/web-application/blob/main/kubo-rpc-api/api-v0-add/book1.json) using the [(Project Import File)](https://github.com/gcp-development/web-application/tree/main/kubo-rpc-api/api-v0-add) file and copy the [CID](https://docs.ipfs.tech/concepts/content-addressing/#content-identifiers-cids).

![image](https://user-images.githubusercontent.com/76512851/227532596-327ccdf7-1790-4d8b-9dfa-898171c6b192.png)

Search for file using the CID "QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL" in the kubo node webui(http://demo:32546/webui/).

![image](https://user-images.githubusercontent.com/76512851/227533885-9083e545-a1f8-4262-845b-d955114650b7.png)

We can access the file directly via browser https://ipfs.io/ipfs/QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL using the [IPNS name](https://docs.ipfs.tech/concepts/ipns/). We can also use a [DNSLink](http://docs.ipfs.tech/concepts/dnslink/) address which looks like an IPNS address, but it uses a DNS name in place of a hashed public key.

![image](https://user-images.githubusercontent.com/76512851/227534320-5eb3df09-2658-40b5-9ddb-d94b5371ef09.png)

<hr>

#### Content linking via directed acyclic graphs (DAGs)

[Directed acyclic graphs (DAGs)](http://demo:30009/ipns/docs.ipfs.tech/concepts/merkle-dag/) are a hierarchical data structure.
      
Import the file [book2.json](https://github.com/gcp-development/web-application/blob/main/kubo-rpc-api/api-v0-add/book2.json).
      
![image](https://user-images.githubusercontent.com/76512851/227559021-67685565-4632-431a-befb-1236eae5130c.png)
      
Import the file [book3.json](https://github.com/gcp-development/web-application/blob/main/kubo-rpc-api/api-v0-add/book3.json).
      
![image](https://user-images.githubusercontent.com/76512851/227558570-393b0b19-83aa-4922-a1b6-d282482c4a39.png)

Import the DAG node file [library.json](https://github.com/gcp-development/web-application/blob/main/kubo-rpc-api/api-v0-dag-put/library.json) using the [(Project Create Dag)](https://github.com/gcp-development/web-application/tree/main/kubo-rpc-api/api-v0-dag-put).
      
Using the three CIDs of the files already imported(book1.json,book2.json and book3.json) to create a [DAG](http://demo:30009/ipns/docs.ipfs.tech/concepts/merkle-dag/#merkle-directed-acyclic-graphs-dags) in IPFS.
      
```bash
[
 { "/":"QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL"},
 { "/":"QmYqo1Ack8g2rDX6TEoPA14oNASJrXEVB4oTEKv8So6Ect"},
 {"/":"QmUfV4m2PUM559LSvDsJkoz1KofTVq25RDXwW5uMdjNb4u"}
]
```

Copy the CID.
      
![image](https://user-images.githubusercontent.com/76512851/227553593-5645614b-5208-4eaa-8158-390c5f391a29.png)

Using the CID "bafyreihw63bea7teb7araypl6sdhhhv57vohawroks4nxogorc2jx7b5oi" of the DAG node created. Search for the DAG node in the IPFS Kubo webui (http://demo:32546/webui/).

![image](https://user-images.githubusercontent.com/76512851/227563963-35175cb3-2a74-4b8a-b1b2-690d9275c934.png)

<hr>
      
#### <b>Content discovery via distributed hash tables (DHTs)</b>
      
[Distributed Hash Tables](http://docs.ipfs.tech/concepts/dht) are a form of a distributed database that can store and retrieve information associated with a key in a network of peer nodes that can join and leave the network at any time. The nodes coordinate among themselves to balance and store data in the network without any central coordinating party.

DHTs have the following properties:
<ul>
<li>Decentralised & Autonomous: Nodes collectively form the system without any central authority.</li>
<li>Fault Tolerant: System is reliable with lots of nodes joining, leaving, and failing at all times.</li>
<li>Scalable: System should function efficiently with even thousands or millions of nodes.</li>
</ul>

DHTs support the following 3 functions:
<ul>
      <li>put (key, value) Write a key/value pair to the routing system. (We are going to use the <a href="http://demo:30009/ipns/docs.ipfs.tech/reference/kubo/rpc/#api-v0-routing-put" target="_self">Kubo RPC api-v0-routing-put<a>)</li>
      <li>get (key) Given a key, query the routing system for its best value. (We are going to use the <a href="http://demo:30009/ipns/docs.ipfs.tech/reference/kubo/rpc/#api-v0-routing-get" target="_self">Kubo RPC api-v0-routing-get<a>)</li>
      <li>provide(key) Announce to the network that you are providing given values.(We are going to use the <a href="http://demo:30009/ipns/docs.ipfs.tech/reference/kubo/rpc/#api-v0-routing-provide" target="_self">Kubo RPC api-v0-routing-provide<a>)</li>
</ul>

     
      
      
      
[IPNS Name](https://github.com/ipfs/specs/blob/main/ipns/IPNS.md#ipns-name)

```bash
/ipfs/QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL
```

[IPNS Record](https://github.com/ipfs/specs/blob/main/ipns/IPNS.md#ipns-record)
```bash
value:"/ipfs/QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL" signatureV1:"\006m\2020\352 [\214\263U\301\316\233\350\355\336\233k*\323\350\224\340\310M}:\030\252\353\034\321\024\251\000\262\277\341\356\305\317\241\303\333P\377\214\200|v\337w\210}\337oi\337\226S\326\3268\274\266\177t3\360\362\2102\365\002\347\237\nL\306-\327\346\306\203%d\265\366\023o},|\007N~.3\326\354\222\325\223B!\031\035\333\352a\005`\353c\031h\027A\365@VBk\316j?\341@\265P\372\225\220\006\223\035\007e\2307O\330\306\007\340\303\2275.\346\333\003\344|BjY\3135gN$<\344\310.\360m-l\217\251\376\036\005\033\226\263u.\220\357\020BA.Wg\274\010\020\246t\376\2430\273\317\010\020\324\270\272\225\353E\233\324\345\032\370f'\004$\302b\244\333\275\033Q\263?(\031\005\212;\221)\274\356\321\213\272\036\021\"\247L\362\002H\357\332\217\321\321Y\341\262fr\017\343" validityType:EOL validity:"2023-03-26T15:14:20.243330515Z" sequence:0 ttl:86400000000000 signatureV2:"r\032#\007e\364\321\350Nt\314\013wFj,\276lIqq\367\022\200\260\316\314\365\"\370\3638F\367\252\206\346DB\000\266\207\273\345XB\245zC\251\3531P\274R\031wDu\221\352O\215\336A\034\252\241&\237 M\334\263\376\314\271\240\026E\265\233Qp\360\322\251\354\242^aN \363\374\021\333\007\257p(\2752\322\304\267\356aF\024\246\253<\307\234\226\\\221\357\227\032\242X\300\276\347\373\tT\203m\335\3126\361\270\300\033\221\313.-\334\251\340\203\342\367\374sce\233\262\250\307\370\201#\335\342\206\005\226\010a\336\n\340\211\313\013\247\326\256\263\002\214+\357\204\242\377\364\261]\177=r\2344\255#\305\362v\234\375p\350u\247\350\205y|O \271G\305\247\334dI\256\263?\261\244\324{\321\326\362\177\362TME\244\223\307\"Q\213Q\335\207\236\223,\"\346+0\202\177\264e\254(\313EE\273" data:"\245cTTL\033\000\000N\224\221O\000\000eValueX4/ipfs/QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosLhSequence\000hValidityX\0362023-03-26T15:14:20.243330515ZlValidityType\000"
```

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
[IPFS](https://docs.ipfs.tech/)
