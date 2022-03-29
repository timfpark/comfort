# gRPC in Rust

Upskilling sample for GRPC in Rust that demonstrates two use cases:

1. Simple single-shot RPC request.
2. Streaming subscription updates for a particular value on a specific update period.

## Implementation Overview

### Proto Definition

The RPC protocol is defined in [comfort.proto](./proto/comfort.proto) and is compiled into Rust stubs via [build.rs](./build.rs).

### Service

The service implementing the gRPC service is in [server.rs](./middleware/server.rs). It imports the compiled proto stubs via the macro at the top and the bulk of the file implements the imported "Comfort" trait that matches the proto definition.

`set_desired_temperature` implements a the simple single RPC request. It unpacks the inner request and then creates a typed response.

`get_cabin_temperature` implements a single request / streaming response. It unpacks the inner request that designates the period in which the client wants updates over. It next establishes an asynchronous loop that manages these updates, which exits when a send fails, indicating the the coorresponding client has closed their connection.

### Clients

[set_temp.rs](./app/set_temp.rs) demonstrates a client utilizing the single shot gRPC interface.

[temp_stream.rs](./app/temp_stream.rs) demonstrates a client utilizing the streaming gRPC interface.

## Characterization

### Latency

[request_bench.rs](./app/request_bench.rs) implements a simple latency benchmark of the single shot gRPC interface.

On a MacBook Pro M1 and over 100k iterations we observed an average request latency of 60 microseconds.

### Executable Size

[Tonic](https://github.com/hyperium/tonic) utilizes [prost](https://github.com/tokio-rs/prost) and [tokio](https://tokio.rs) under the covers. Together the three libraries add 2.8MiB to the executable size. There is probably a lot of optimization that can happen here to further reduce this size and it is also expected that a lot of this increase will be amortized over other uses of the std library and Tokio.

### Memory Usage

- TODO: Characterize high water memory usage over large number of requests to single-shot interface via Valgrind.
- TODO: Characterize high water memory usage over long subscription to streaming interface.
