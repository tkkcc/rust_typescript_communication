# Rust Typescript Communication

examples for communication between rust server and typescript client(browser)

## [gprc](/grpc)

### step

1. write proto
1. generate server code via tonic
1. generate client code via protobuf-ts

### limit

nested message in proto3 must be optional, so in rust, struct fields in struct are always wrapped in Option

## [rspc](/rspc)

###  step

1. write server code
2. generate typescript bindings, and use in client

## [openapi](/openapi)

### step

1. write server code
1. generate openapi schema via builtin or thirty party tool
1. generate typescript client via openapi-generator

### limit

1. axum has no official support, need thirty party, like <https://github.com/kurtbuilds/oasgen>. poem, salvo, zino support

2. tried poem, oai macro and underlying function can be unmatched


## [graphql](/graphql)

TODO

## [json](/json)

TODO
