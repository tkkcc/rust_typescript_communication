# rspc

## step

1. write server code
2. generate typescript bindings, and use in client

## limit

1. docs incomplete

## run

start server
```sh
cd server
cargo run # this will generate typescript code
```

serve client(in another shell)
```sh
cd client
npm i
npm run dev
```

open <http://localhost:5173>


## how client created

```sh
npm create vite@latest client -- --template vanilla-ts
```



