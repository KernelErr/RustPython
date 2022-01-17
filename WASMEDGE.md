 

# WasmEdge Network Support

Currently, there is no proposal about supporting network feature for WASI. But WasmEdge has provided several network socket functions to make accessing network inside WASI module possible.

For Rust developer, there are two crates available on Crates.io:

- [wasmedge_wasi_socket](https://crates.io/crates/wasmedge_wasi_socket)
- [wasmedge_http_req](https://crates.io/crates/wasmedge_http_req)

In this fork version of RustPython, I implemented two python library for networking.

- wasmedge_socket
- wasmedge_http

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

HTTP Get:

```
Welcome to the magnificent Rust Python 0.1.2 interpreter 😱 🖖
>>>>> import wasmedge_http as http
>>>>> client = http.client()
>>>>> resp = client.get("http://127.0.0.1/")
>>>>> resp.status_code()
200
>>>>> resp.headers()
{'ETag': '"61a370f6-264"', 'Last-Modified': 'Sun, 28 Nov 2021 12:07:18 GMT', 'Server': 'nginx/1.20.2', 'Accept-Ranges': 'bytes', 'Content-Type': 'text/html', 'Date': 'Mon, 17 Jan 2022 09:37:03 GMT', 'Connection': 'close', 'Content-Length': '612'}
>>>>> resp.body()
b'<!DOCTYPE html>\n<html>\n<head>\n<title>Welcome to nginx!</title>\n<style>\n    body {\n        width: 35em;\n        margin: 0 auto;\n        font-family: Tahoma, Verdana, Arial, sans-serif;\n    }\n</style>\n</head>\n<body>\n<h1>Welcome to nginx!</h1>\n<p>If you see this page, the nginx web server is successfully installed and\nworking. Further configuration is required.</p>\n\n<p>For online documentation and support please refer to\n<a href="http://nginx.org/">nginx.org</a>.<br/>\nCommercial support is available at\n<a href="http://nginx.com/">nginx.com</a>.</p>\n\n<p><em>Thank you for using nginx.</em></p>\n</body>\n</html>\n'
>>>>>
```

