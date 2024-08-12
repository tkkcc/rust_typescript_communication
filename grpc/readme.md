# gRPC 

step: first define proto, then use tonic to generate server code, and protobuf-ts to generate client code.

issue: nested message in proto3 must be optional

## run

start server

```sh
cd tonic_server
cargo run
```

serve client(in another shell)

```sh
cd vue_client
npm i
npm run dev
```

## how client created

```sh
npm create vite@latest vue_client -- --template vue-ts
```





