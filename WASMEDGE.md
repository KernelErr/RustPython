 

# WasmEdge Network Support

Currently, there is no proposal about supporting network feature for WASI. But WasmEdge has provided several network socket functions to make accessing network inside WASI module possible.

For Rust developer, there are two crates available on Crates.io:

- [wasmedge_wasi_socket](https://crates.io/crates/wasmedge_wasi_socket)
- [wasmedge_http_req](https://crates.io/crates/wasmedge_http_req)

In this fork version of RustPython, I implemented two python library for networking.

- wasmedge_socket
- wasmedge_http(WIP)

To enable WasmEdge related feature, you can compile RustPython with the following command:

```
$ cargo build --release --target wasm32-wasi --features="freeze-stdlib,wasmedge"
$ wasmedgec ./target/wasm32-wasi/release/rustpython.wasm ./target/wasm32-wasi/release/rustpython.wasm
```

## WasmEdge Socket

There is an example for create a TCP connection, in the meanwhile, we use `nc` to create a listening server.

```
$ wasmedge ./target/wasm32-wasi/release/rustpython.wasm
Welcome to the magnificent Rust Python 0.1.2 interpreter 😱 🖖
>>>>> import wasmedge_socket
>>>>> ts = wasmedge_socket.tcpstream()
>>>>> ts.connect("127.0.0.1:9999")
>>>>> ts.send(b"Hi")
2
>>>>> ts.recv(1024)
b'Hello\n'
>>>>>
```

Create a listener:

```
$ wasmedge ./target/wasm32-wasi/release/rustpython.wasm
Welcome to the magnificent Rust Python 0.1.2 interpreter 😱 🖖
>>>>> import wasmedge_socket
>>>>> server = wasmedge_socket.tcplistener()
>>>>> server.bind("127.0.0.1:9999")
>>>>> (socket, addr) = server.accept()
>>>>> socket.recv(1024)
b'Hi\n'
>>>>> socket.send(b"Hello")
5
>>>>>
```

## WasmEdge HTTP

WIP
