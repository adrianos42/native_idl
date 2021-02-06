import 'dart:ffi';
import 'dart:io' show Platform;

import 'package:ffi/ffi.dart';
import 'dart:typed_data';

import 'dart:convert';

DynamicLibrary openLibrary(String name, String path) {
  final finalPath;

  if (Platform.isLinux || Platform.isAndroid || Platform.isFuchsia)
    finalPath = path + "lib" + name + ".so";
  else if (Platform.isMacOS) {
    finalPath = path + "lib" + name + ".dylib";
  } else if (Platform.isWindows)
    finalPath = path + name + ".dll";
  else {
    throw ArgumentError();
  }

  return DynamicLibrary.open(finalPath);
}

abstract class AbiInternalError {
  static const int ok = 0x0;
  static const int invalidArg = 0x1;
  static const int nullPtr = 0x2;
  static const int abort = 0x3;
  static const int callbackException = 0x4;
  static const int undefinedException = 0x5;
  static const int unimplemented = 0x6;
  static const int type = 0x7;
  static const int notAllowedOperation = 0x8;
  static const int noDefaultConstructor = 0x9;

  static String asString(int value) {
    switch (value) {
      case AbiInternalError.ok:
        return 'ok';
      case AbiInternalError.invalidArg:
        return 'invalidArg';
      case AbiInternalError.nullPtr:
        return 'nullPtr';
      case AbiInternalError.abort:
        return 'abort';
      case AbiInternalError.callbackException:
        return 'callbackException';
      case AbiInternalError.undefinedException:
        return 'undefinedException';
      case AbiInternalError.unimplemented:
        return 'unimplemented';
      case AbiInternalError.type:
        return 'type';
      case AbiInternalError.notAllowedOperation:
        return 'notAllowedOperation';
      case AbiInternalError.noDefaultConstructor:
        return 'noDefaultConstructor';
      default:
        throw ArgumentError('Unknown abi error');
    }
  }

  static void handleError(int error, String message) {
    switch (error) {
      case AbiInternalError.ok:
        return;
      case AbiInternalError.invalidArg:
        throw ArgumentError(
            '`$message`, error `${AbiInternalError.asString(error)}`');
      default:
        throw Exception();
    }
  }
}

class AbiStreamPartial extends Struct {
  @Int64()
  int? index;
  @Int64()
  int? length;
  Pointer<Void>? data;
}

class AbiStreamSize extends Struct {
  @Int64()
  int? index;
  @Int64()
  int? length;
}

abstract class AbiStreamSenderState {
  static const int ok = 0x0;
  static const int value = 0x1;
  static const int request = 0x2;
  static const int waiting = 0x3;
  static const int done = 0x4;
}

abstract class AbiStreamReceiverState {
  static const int ok = 0x0;
  static const int close = 0x1;
  static const int start = 0x2;
  static const int pause = 0x3;
  static const int resume = 0x4;
  static const int request = 0x5;
}

class AbiStream extends Struct {
  @Int64()
  int? state;
  @Int64()
  int? wakeHandle;
  Pointer<Void>? wakeObject;
  Pointer<Void>? wakeCallback;
  Pointer<Void>? data;
}

class AbiVariant extends Struct {
  @Int64()
  int? variant;
  Pointer<Void>? data;
}

class AbiPair extends Struct {
  Pointer<Void>? firstData;
  Pointer<Void>? secondData;
}

extension AbiPairEx on AbiPair {
  Pointer<AbiPair> asPtr() {
    final result = allocate<AbiPair>();
    result.ref.firstData = firstData;
    result.ref.secondData = secondData;
    return result;
  }
}

class AbiArray extends Struct {
  @Int64()
  int? length;
  Pointer<Void>? data;
}

extension AbiArrayEx on AbiArray {
  Pointer<AbiArray> asPtr() {
    final result = allocate<AbiArray>();
    result.ref.data = data;
    result.ref.length = length;
    return result;
  }
}

class AbiMap extends Struct {
  @Int64()
  int? length;
  Pointer<Void>? key;
  Pointer<Void>? data;
}

extension AbiMapEx on AbiMap {
  Pointer<AbiMap> asPtr() {
    final result = allocate<AbiMap>();
    result.ref.data = data;
    result.ref.key = data;
    result.ref.length = length;
    return result;
  }
}

class AbiBytes extends Struct {
  @Int64()
  int? length;
  Pointer<Uint8>? data;

  static void disposeWithPtr(Pointer<AbiBytes> value) {
    free(value.ref.data!);
  }
}

extension BytesPointer on Pointer<AbiBytes> {
  void dispose() {
    free(ref.data!);
    free(this);
  }

  Uint8List asUint8List() {
    return Uint8List.fromList(ref.data!.asTypedList(ref.length!));
  }
}

extension BytesInto on Uint8List {
  Pointer<AbiBytes> asAbiBytes() {
    final data = allocate<Uint8>(count: length + 0x8);
    final listSource = data.asTypedList(length);
    listSource.setAll(0, this);

    final bytes = allocate<AbiBytes>().ref
      ..length = length
      ..data = data;

    return bytes.addressOf;
  }

  void asAbiBytesWithPtr(Pointer<AbiBytes> value) {
    final data = allocate<Uint8>(count: length + 0x8);
    final listSource = data.asTypedList(length);
    listSource.setAll(0, this);

    value.ref.length = length;
    value.ref.data = data;
  }
}

class AbiString extends Struct {
  @Int64()
  int? length;
  Pointer<Uint8>? data;

  static void disposeWithPtr(Pointer<AbiString> value) {
    free(value.ref.data!);
  }
}

extension AbiStringPointer on Pointer<AbiString> {
  String asString() {
    return utf8.decode(Uint8List.view(
      ref.data!.asTypedList(ref.length!).buffer,
      0,
      ref.length,
    ));
  }

  void dispose() {
    free(ref.data!);
    free(this);
  }
}

extension AbiStringInto on String {
  Pointer<AbiString> asAbiString() {
    final dataString = utf8.encode(this);
    final resultData = allocate<Uint8>(count: dataString.length + 0x8);
    resultData.asTypedList(dataString.length).setAll(0, dataString);

    final utf8String = allocate<AbiString>().ref
      ..data = resultData
      ..length = dataString.length;

    return utf8String.addressOf;
  }

  void asAbiStringWithPtr(Pointer<AbiString> value) {
    final dataString = utf8.encode(this);
    final resultData = allocate<Uint8>(count: dataString.length + 0x8);
    resultData.asTypedList(dataString.length).setAll(0, dataString);

    value.ref.data = resultData;
    value.ref.length = dataString.length;
  }
}

class DartCObjectType {
  static const int kNull = 0x0;
  static const int kBool = 0x1;
  static const int kInt32 = 0x2;
  static const int kInt64 = 0x3;
  static const int kDouble = 0x4;
  static const int kString = 0x5;
  static const int kArray = 0x6;
  static const int kTypedData = 0x7;
  static const int kExternalTypedData = 0x8;
  static const int kSendPort = 0x9;
  static const int kCapability = 0xA;
  static const int kUnsupported = 0xB;
  static const int kNumberOfTypes = 0xC;
}

class DartCObjectInt extends Struct {
  @Int64()
  int? type;
  @Int64()
  int? value;

  static Pointer<DartCObjectInt> newObject(int value) {
    final result = allocate<DartCObjectInt>();
    result.ref.type = DartCObjectType.kInt64;
    result.ref.value = value;
    return result;
  }
}

// union {
//     bool as_bool;
//     int32_t as_int32;
//     int64_t as_int64;
//     double as_double;
//     char* as_string;
//     struct {
//       Dart_Port id;
//       Dart_Port origin_id;
//     } as_send_port;
//     struct {
//       int64_t id;
//     } as_capability;
//     struct {
//       intptr_t length;
//       struct _Dart_CObject** values;
//     } as_array;
//     struct {
//       Dart_TypedData_Type type;
//       intptr_t length;
//       uint8_t* values;
//     } as_typed_data;
//     struct {
//       Dart_TypedData_Type type;
//       intptr_t length;
//       uint8_t* data;
//       void* peer;
//       Dart_HandleFinalizer callback;
//     } as_external_typed_data;
//   };
