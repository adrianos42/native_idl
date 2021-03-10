# Layers

## Bytes

This is only a partial layer, which means it depends on another layer to be implemented and a protocol to be defined.
It's the basis of protocols that requires a byte stream, instead of using a ffi, for example.

### Marshaling

For any method, the `Read` and `Write` traits are used to convert all data between rust code and the protocol used. In case of receiving a request, the data is read and converted to rust type, calling the implemented function. If the method returns anything, it will be written using the argument that implements `Write`. With only the exception of streams that will require a callback. Nonetheless, the callback will use the same approach used for methods.

### Request

All hashs are 16 bytes long, while enumerations are a 64-bit integer.

* Header

1. package `hash` -> `hash`
2. library `hash` -> `hash`
3. interface `hash` -> `hash`
4. `MethodType`

* create Instance

    1. *empty* -> `MethodType`, `uuid`, then `error code`

* dispose Instance

    1. instance `uuid` -> `MethodType`, then `error code`

* method call

    1. intance `uuid` -> `uuid`
    2. method `hash` ->  `hash`
    3. `MethodCallType` -> `MethodCallType`

* Method

    1. `args` -> `result`, then `error code`


## FFI

Layer that uses foreigh function interface as communication. For now, only 64-bit is supported. If 32-bit or 128-bit is used in the future, all other layers will also have to be the same type as to avoid data loss.

### Layout

Since it uses the x64 calling convention, call between language would be the same as any call using the languague supported ffi calls. In Rust, this would be the `extern "C"` function attribute, and in Dart, the `ffi` library to annotate functions used for interfacing with this calling convention.

### Stream support

The main problem with stream is that it needs to use a callback to send an event to the client to acknowledge for new data, and then the need to raise a signal to the thread that is listening to that event. This is the reason that there's a function that sends a handle and id, so that the client can call the server to retrive data or do something else with the stream.

## Web Sockets
