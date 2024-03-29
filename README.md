# Web Application

## Motivation

There is no "one-size-fits-all" for any technology and everything comes down to "what we are trying to solve?". For Web3, we are dealing with distributed applications working on distributed software infrastructure (P2P networks). In business terms this means we are going to lose money if we don't get the right trade-offs in our technology stack. For startups these technical choices and trade-offs will be crucial in their product development and ability to scale. Having a good idea is not enough for a startup to thrive.<br>
For the Web3 world, type safety (verifies and enforces type constraints at compile-time), high-performance async (good [ecosystem](https://rust-lang.github.io/async-book/08_ecosystem/00_chapter.html#the-async-ecosystem) of non-blocking  I/O libraries and runtimes), automatic memory management and memory safety without garbage collection ([ownership model](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)) were the main reasons that made [Rust](https://www.rust-lang.org/tools/install) my first choice. For instance, writing [smart contracts](https://use.ink/) that doesn't have memory bugs and consumes less storage on the blockchain is a massive advantage. On the other hand for user interface (UI) development, [React](https://reactjs.org/) is still a better choice due the rich UI features offered.<br>

The examples developed, will be focused on the back-end services and distributed software infrastructure demonstrating the key points of using [Rust](https://github.com/rust-lang/rust) with [React](https://reactjs.org/) and Rust with [IPFS](https://ipfs.tech/) for the Web3 world.

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

Our main objective is to understand how to build a distributed web application that are safe, efficient, highly performant, and do not "break the piggy bank" to operate and maintain. We are going to look at two type of storages a central storage(Postgres) and a distributed storage (IPFS).

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

The examples provided were developed using the [Kubo RPC API](https://docs.ipfs.tech/reference/kubo/rpc/#kubo-rpc-api-v0-reference) together with the [actix-web](https://docs.rs/actix-web/latest/actix_web/), [awc](https://crates.io/crates/awc) and [actix-multipart-rfc7578](https://crates.io/crates/actix-multipart-rfc7578) crates.
      
![image](https://user-images.githubusercontent.com/76512851/229612684-30c3e220-c14f-42b5-860a-36c118b0034f.png)

Note: A Rust implementation of a IPFS node ([iroh](https://iroh.computer/)) is currently being developed. But unfortunately is not mature enough for the examples needed.

<hr>
      
#### Unique identification via content addressing
      
The [CID](https://docs.ipfs.tech/concepts/content-addressing/#content-identifiers-cids) is a label used to point to material in IPFS. Import the [book1.json](https://github.com/gcp-development/web-application/blob/main/kubo-rpc-api/api-v0-add/book1.json) using the Rust project [api-v0-add](https://github.com/gcp-development/web-application/tree/main/kubo-rpc-api/api-v0-add) file and copy the [CID](https://docs.ipfs.tech/concepts/content-addressing/#content-identifiers-cids).

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

Import the DAG node file [library.json](https://github.com/gcp-development/web-application/blob/main/kubo-rpc-api/api-v0-dag-put/library.json) using the the Rust project [api-v0-dag-put](https://github.com/gcp-development/web-application/tree/main/kubo-rpc-api/api-v0-dag-put).
      
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
      
How does DHT work ?

As a simple example the diagram below represents a ring overlay network (logical network implemented on top of some underlying network) with 6 nodes. To find out which node (peer) will get to store a specific key we just need to hash that key. Each node will have a hash value (Peer Id) and will store 5 keys. Making each node an independent hash table bucket in that ring overlay network.
      
![image](https://user-images.githubusercontent.com/76512851/229342962-9ba3226b-e5ed-4c88-a7d9-c2dd405b8b40.png)
      
For this example we are using the DHT to map a data identifier to a peer; this is a "Provider records" type of key-value pairing. It's used to find and advertise content.
There are other two main types, "IPNS records" (map an [IPNS key](https://specs.ipfs.tech/ipns/ipns-record/#ipns-keys) to a [IPNS record](https://specs.ipfs.tech/ipns/ipns-record/#ipns-record)) and "Peer records" (map a Peer Id to a set of multi addresses at which the peer may be reach)      
      
Most of the DHTs implementations support the following 3 basic functions:
<ul>
      <li>put (key, value) We are going to use the <a href="http://docs.ipfs.tech/reference/kubo/rpc/#api-v0-routing-put" target="_self">Kubo RPC api-v0-routing-put<a> (Write a key/value pair to the <a href="http://docs.ipfs.tech/concepts/dht/#routing-tables" target="_self">routing system</a>.)</li>
      <li>get (key) We are going to use the <a href="http://docs.ipfs.tech/reference/kubo/rpc/#api-v0-routing-get" target="_self">Kubo RPC api-v0-routing-get<a> (Given a key, query the <a href="http://docs.ipfs.tech/concepts/dht/#routing-tables" target="_self">routing system</a> for its best value.)</li>
      <li>provide (key) We are going to use the <a href="http://docs.ipfs.tech/reference/kubo/rpc/#api-v0-routing-provide" target="_self">Kubo RPC api-v0-routing-provide<a> (Announce to the network that we are providing given values.)</li>
</ul>

How to create an [IPNS record](http://docs.ipfs.tech/concepts/dht/#ipns-records) ?

Execute the [GO(go1.20.2)](https://go.dev/doc/install) script [ipnsRecord.go](https://github.com/gcp-development/web-application/blob/main/kubo-rpc-api/create-ipns-record/ipnsRecord.go).
      
![image](https://user-images.githubusercontent.com/76512851/229357924-1fcdaa96-d6f8-4bcc-ad34-ac29e338cd08.png)

Copy the private-key-ipns-record.bin file to the src path and run the Rust project [api-v0-key-import](https://docs.ipfs.tech/reference/kubo/rpc/#api-v0-key-import).

![image](https://user-images.githubusercontent.com/76512851/229357726-c3395f78-a078-4e6c-b2f9-5a1c3576d821.png)

```bash
/ipns/k2k4r8lpp59iv154i7dfnd5m99tke25rqhqaybpssnk3ds5h5t5boe8j
```

Copy the signed-ipns-record.bin file to the src path and run the Rust project [api-v0-name-inspect](https://github.com/gcp-development/web-application/tree/main/kubo-rpc-api/api-v0-name-inspect) against the IPNS name /ipns/k2k4r8lpp59iv154i7dfnd5m99tke25rqhqaybpssnk3ds5h5t5boe8j.
      
![image](https://user-images.githubusercontent.com/76512851/229359402-e00f5a0e-c78d-418f-8de0-8ff396261ca0.png)

[IPNS Name](https://specs.ipfs.tech/ipns/ipns-record/#ipns-name)

```bash
/ipns/k2k4r8lpp59iv154i7dfnd5m99tke25rqhqaybpssnk3ds5h5t5boe8j
```

[IPNS Record](https://specs.ipfs.tech/ipns/ipns-record/#ipns-record)
```bash
{
    "Entry": {
        "Value": "/ipfs/QmUfV4m2PUM559LSvDsJkoz1KofTVq25RDXwW5uMdjNb4u",
        "ValidityType": 0,
        "Validity": "2023-04-03T13:29:58.128901162Z",
        "Sequence": 0,
        "TTL": 60000000000,
        "PublicKey": "mCAASpgIwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQDSVhekFyk3/EIdJW530Zip/MeliHGDumXpbT6dBB/BWTP8wv7ioRXAiH0fs9v6Uflglw1VqN+08gs4ScbWkeRaVP1Q2d+lzffeTwiG4eZm/O9OGmdPQfXP/nSOVsKjDITxrVnYLoLLXnHpRcSMuueqpULMpxKj1aT3tSetn7YgrC1TuT3A40RVtwi8ODly9dMkzYH1lqzqMNtqHFumAyPN1hgEDHN2awbQs7KDLsvHF/LFUVlq/xZb6mbmiZBhIfv/YpvTNA9sYZVvL02Q1NqqOvcg0T5SY/lLNv8w1PZuePV58ZMJOHYHgP22MR5hnmrSYOlpebbU8A3EJCgG9FGvAgMBAAE",
        "SignatureV1": "meWDwdTYzNsaCrn/rP6IVaa5cnmqs5tYA6OKN1p647UcoGGDrurSgVXvIKmjIlKr6JigJB6NL2cnRksDdX+vYQdSNejb2bqtne+bG7jmc2hpj4xQHMWpVdgKdN1GwG1rJdcrEJnzqt0eNhiwKOMmhC8ynlx7B/8vzw+70unxCyBaWeXnumJYKXzymSRIe2UdI87WRzu6NEJh7QJuIBOuwlGjAyRepCAodUCVqr/Fl2hAtdlyRGmavtwGda6TmVp9Xi2F+o2qOwLJEvA6fMiR5uH0YoXnMdN7DY3IVr0+BmKnPyCsoCx+X5OsVofCkWpEKFvJKfY9ILeqxEp/7DbvabQ",
        "SignatureV2": "modrb+wC4mBtUahM/KhSm1tYWhUj73fENvmceLAklbYYVfWwnmAHl1VGrlOmDVrdJv40+7gQgQcMUTwfk1S9XWqwopbBHhfKZxb0RFIoGy/Fmpay0rHpYBzLJBM5ah+Xcz6rnfo0FsNSjLz4PFl/dG4gNPNL1Pv9W01G3abjTEcZoTm8BOh7wRkbagG1eO/UayKaXeCRVgd/9xfKF9XJs0jo3RIbbf4lXbWme1j7k8vO861OCEp+bky9P5eQ/dnYnjxhysE6HkvbofqJxg9VVfyhe7DC46ZXu0FVmw2PznrsHz+mPRogZwawVrqxh+6YBkbJTJhd4YoEZdd5MEnZUaA",
        "Data": {
            "Sequence": 0,
            "TTL": 60000000000,
            "Validity": {
                "/": {
                    "bytes": "MjAyMy0wNC0wM1QxMzoyOTo1OC4xMjg5MDExNjJa"
                }
            },
            "ValidityType": 0,
            "Value": {
                "/": {
                    "bytes": "L2lwZnMvUW1VZlY0bTJQVU01NTlMU3ZEc0prb3oxS29mVFZxMjVSRFh3VzV1TWRqTmI0dQ"
                }
            }
        }
    },
    "Validation": {
        "Valid": true,
        "Reason": "",
        "PublicKey": "QmUYNURRiUgNo1YevHW7XH8dDTqw3ywKdUZN6PiVP1deJe"
    }
}
```
      
Copy the signed-ipns-record.bin file to the src path and run the Rust project [api-v0-routing-put](https://github.com/gcp-development/web-application/tree/main/kubo-rpc-api/api-v0-routing-put).

![image](https://user-images.githubusercontent.com/76512851/229452909-4e9da6cc-1944-4a0e-8e81-18c55f68269a.png)
      
To get the new route created just, run the Rust project [api-v0-routing-get](https://github.com/gcp-development/web-application/tree/main/kubo-rpc-api/api-v0-routing-get).
      
![image](https://user-images.githubusercontent.com/76512851/229458025-43cc1879-b034-43c3-aad9-5f1f37263d6c.png)
      
To resolve the IPNS name k2k4r8lpp59iv154i7dfnd5m99tke25rqhqaybpssnk3ds5h5t5boe8j we will use the Rust project [api-v0-name-resolve](https://github.com/gcp-development/web-application/tree/main/kubo-rpc-api/api-v0-name-resolve).
      
![image](https://user-images.githubusercontent.com/76512851/229470868-b3ac421d-e89e-45c4-a3f9-f96c8b181158.png)

IPFS CID
```bash
/ipfs/QmUfV4m2PUM559LSvDsJkoz1KofTVq25RDXwW5uMdjNb4u
```
Note: IPNS names are mutable pointers to immutable pointers IPFS CIDs (immutable because they're derived from the content).

Resolve the IPNS Name in the browser (https://ipfs.io/ipns/k2k4r8lpp59iv154i7dfnd5m99tke25rqhqaybpssnk3ds5h5t5boe8j)
![image](https://user-images.githubusercontent.com/76512851/229567669-c73abb7f-6715-4b1c-bad9-49f5f9e8a01b.png)

Resolve the IPFS CID in the browser (https://ipfs.io/ipfs/QmUfV4m2PUM559LSvDsJkoz1KofTVq25RDXwW5uMdjNb4u)
![image](https://user-images.githubusercontent.com/76512851/229567352-fcae6bd0-2fd2-473e-ae4c-f6122c92e30e.png)

To announce the new IPFS CID to the overlay network just, run the Rust project [api-v0-routing-provide](https://github.com/gcp-development/web-application/tree/main/kubo-rpc-api/api-v0-routing-provide).

List of the peer Ids.

![image](https://user-images.githubusercontent.com/76512851/229569317-c39f2a62-4b5d-42e2-b6a4-a5cf81edd9b6.png)
      
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
