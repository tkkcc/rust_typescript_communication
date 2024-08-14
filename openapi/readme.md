# openapi

## step

1. write server code
1. generate openapi schema via builtin or third party tool
1. generate typescript client via [openapi-ts](https://github.com/hey-api/openapi-ts) or [openapi-typescript](https://github.com/openapi-ts/openapi-typescript), and others

## limit

1. axum has no official support, need third party tool, like [oasgen](https://github.com/kurtbuilds/oasgen).
poem, salvo, zino have builtin support

1. axum query works in swagger ui, but not with typescript generator. simply **don't use query** can solve this, only use path and body, or only body

1. openapi-typescript consumes response type as json, so can't decode plain text. **use json response** can solve. openapi-typescript generate query style api, force typing on path, query and body. openapi-ts generates function style api, but doesn't force typing on path and query, only body

1. specifying path, query, body adds one level complexity, compared with other communication methods

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

open <http://localhost:5173>

## how client created

```sh
npm create vite@latest client -- --template vue-ts
```





