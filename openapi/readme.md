# openapi

## step

1. write server code
1. generate openapi schema via builtin or third party tool
1. generate typescript client via openapi-ts or openapi-typescript, and others

## limit

1. axum has no official support, need third party tool, like [oasgen](https://github.com/kurtbuilds/oasgen).
poem, salvo, zino have builtin support

1. axum query works in swagger ui, but not with typescript generator. simply **don't use query** can solve this, only use path and body, or only body

1. openapi-typescript consume response type as json, so fail to decode plain text. **use Json response** can solve. openapi-typescript generate query style api(has code completion for method names)

1. openapi-ts generate function style api. but don't force typing on path and query, only body. specific path, query, body add more complexity, compared with grpc

## run

this example use two generators from openapi to typescript

start server

```sh
cd server
cargo run        # this will generate openapi.yaml
```

serve client(in another shell)

```sh
cd client
npm i
npm run openapi-ts  # this will generate typescript code
npm run openapi-typescript  # this will generate typescript code
npm run dev
```

## how client created

```sh
npm create vite@latest client -- --template vue-ts
```





