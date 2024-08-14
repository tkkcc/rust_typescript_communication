# Rust TypeScript Communication

different methods for communication between rust server and typescript client(browser)

all methods except grpc, support code-first approach: rust -> (optional)generate schema -> generate typescript

all methods support server streaming, but openapi's is more low level, via server-sent events


[gprc](/grpc)

[openapi](/openapi)

[rspc](/rspc)

[qubit](https://github.com/andogq/qubit)

[graphql](/graphql)
