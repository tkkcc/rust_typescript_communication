# Rust TypeScript Communication

different methods for communication between rust server and typescript client(browser)

## summary

- all methods except grpc, support code-first approach: rust -> (optional)generate schema -> generate typescript
- all methods support server streaming, but openapi's way is more low level 
- all methods support code completion for api methods and parameters 

## example

- [gprc](/grpc)
- [openapi](/openapi)
- [graphql](/graphql)
- [rspc](/rspc)
- [qubit](https://github.com/andogq/qubit)

