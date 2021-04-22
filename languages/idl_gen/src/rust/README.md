# Layers - *for use in specification*

## Bytes

This is only a partial layer, which means it depends on another layer to be implemented and a protocol to be defined.
It's the basis of protocols that requires a byte stream, instead of using a ffi, for example.

### Marshaling

For any method, the `Read` and `Write` traits are used to convert all data between rust code and the protocol provided. In case of receiving a request, the data is read and converted to rust type, calling the implemented function. If the method returns anything, it will be written using the argument that implements `Write`, with only the exception of streams that will require a callback. Nonetheless, the callback will use the same approach used for methods.

### Request

All hashs are 16 bytes long, meanwhile enumerations - all types with pascal case here - are a 64-bit integers and uuids are 128-bit, big endian integers.

1. *package* `hash` -> `hash`
2. *library* `hash` -> `hash`
3. *interface* `hash` -> `hash`
4. *call id* `int` -> `MethodType`
5. *method type* `MethodType` -> *call id* `int`, *error* `InternalError`
    1. CreateInstance
        1. *nothing* -> *instance* `uuid`
    2. DisposeInstance
        1. *instance* `uuid` -> *nothing*
    3. MethodCall
        1. *intance?* `uuid` -> `uuid`
        2. *method* `hash` ->  `hash`
        3. *stream?* `MethodCallType`
            1. Method
                1. *args* -> *result*
            2. Stream
                1. *args*
                2. *object id* `int`
                3. *state* `StreamReceiver`
                    1. Ok | Close | Start | Pause | Resume | Request
                        1. *nothing*
    4. StreamValue
        1. *instance?* `uuid` -> `uuid`
        2. *object id* `int`
        3. *state* `StreamSender`
            1. Ok | Request | Awaiting | Done
                1. *nothing*
            2. Value
                1. *stream value* `any`

### Stream response

1. *package* `hash`
2. *library* `hash`
3. *interface* `hash`
4. *method type* `MethodType.StreamValue`
5. *instance?* `uuid`
6. *object id* `int`
7. *state* `StreamSender`
    1. Ok | Request | Awaiting | Done
        1. *nothing*
    2. Value
        1. *stream value* `any`

## FFI

Layer that uses foreigh function interface as communication. For now, only 64-bit is supported. If 32-bit or 128-bit is used in the future, all other layers will also have to be the same type as to avoid data loss.

### Layout

Since it uses the x64 calling convention, call between language would be the same as any call using the languague supported ffi calls. In Rust, this would be the `extern "C"` function attribute, and in Dart, the `ffi` library to annotate functions used for interfacing with this calling convention.

### Stream support

The main problem with stream is that it needs to use a callback to send an event to the client to acknowledge for new data, and then the need to raise a signal to the thread that is listening to that event. This is the reason that there's a function that sends a handle and id, so that the client can call the server to retrive data or do something else with the stream.

## Web Sockets
