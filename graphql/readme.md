# graphql

## step

1. write rust code with [juniper](https://github.com/graphql-rust/juniper)
2. generate graphql schema: [export](https://graphql-rust.github.io/juniper/schema/index.html#export)
3. generate client code follow [apollo client guide](https://www.apollographql.com/docs/react/get-started/)

## limit

1. gql tag autocompletion need additional care to setup, for neovim, vscode is easy
2. wasting time follow this [guide](https://the-guild.dev/graphql/codegen/docs/guides/react-vue), but it's fake

## run

start server

```sh
cd server
cargo run  # this will generate graphql schema
```

serve client(in another shell)

```sh
cd client
npm i
npx graphql-codegen  # this will generate typescript code
npm run dev
```

open <http://localhost:5173>


## how client created

```sh
npm create vite@latest client -- --template react-ts
```

