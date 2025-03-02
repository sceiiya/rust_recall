Rust Archipellago

RJ Ramirez

Rust for blovkhain  CLI - clap-rs
backend  - axum-rs
smart contracts
protocols/ blckchain -

buikding a nide in rust
-> polkadot sdk 
- can do without sdk, purely in rust
- layer 2, roll up sequence



Node is an app with different layers

usual 4 layers of a node
runner - [can be a websocket, or rest api]
routing -
sequencing - 
storage - 


Node Architecture
1. monomorphic (static dispathc) vs polymorphic (dynamic dispacth)
2. fat ju arc<T>s vs Snack Snacy Arc<T>s


static dispatch - ur types should be know in compile time
dynamic dispatch - types are determine in runtime


vtable lookup





Rust for gaming

online multiplayer
future as a state machine


web server
game server - stateful, long-lived
skipped frames hurt
not just requests and responses




irc.nodejs.org

tinyurl.com/ircd-js

high concurrency and low latency I/O bound operations



actor model - rust

Tokio runtime ‘futures’ (async runtime for rust)


async fn fetchre(rul: Box<str>)->Vec<u8>{
let http = get_http_cl

initial get http
two get url
third get bodyColection

FetchRequestFuture

Tokio channels as mailboxes
MPSC channel 
request-response patern




Rust in Web Development

rust web ecosystem

tokio -> async runtime
axum


