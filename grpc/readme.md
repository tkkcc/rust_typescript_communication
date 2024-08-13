# gRPC 

## step

1. write proto
1. generate server code via tonic
1. generate client code via protobuf-ts

## limit

1. not code-first
2. in proto3, nested message must be optional. thus in rust, struct fields in struct are always wrapped in Option

## run

start server

```sh
cd server
cargo run  # this will generate rust code
```

serve client(in another shell)

```sh
cd client
npm i
npm run proto-build  # this will generate typescript code
npm run dev
```

## how client created

```sh
npm create vite@latest client -- --template vue-ts
```
