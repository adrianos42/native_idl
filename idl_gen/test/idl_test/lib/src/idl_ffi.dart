import 'idl_types_interface.dart' as idl_types_interface;
import 'idl_types.dart' as idl_types;
import 'dart:ffi';
import 'dart:async';
import 'dart:typed_data';
import 'dart:isolate';
import 'dart:collection';
import 'package:ffi/ffi.dart';
import 'package:idl_internal/idl_internal.dart';

final DynamicLibrary _$kLib = openLibrary('idl_test', '');

class Types {
  static idl_types.Types _$asValue(Pointer<AbiVariant> $value) {
    switch ($value.ref.variant!) {
      case 0:
        return idl_types.Types.asIntType($value.ref.data!.cast<Int64>().value);
      case 1:
        return idl_types.Types.asStringType(
            $value.ref.data!.cast<AbiString>().asString());
      default:
        throw ArgumentError();
    }
  }

  static void _$fromWithPtr(
      idl_types.Types $value, Pointer<AbiVariant> $result) {
    $result.ref.variant = $value.variant;
    switch ($value.variant) {
      case 0:
        final $tValue = $value.value as int;
        $result.ref.data = (int $value) {
          final $result = allocate<Int64>();
          $result.value = $value;
          return $result;
        }($tValue)
            .cast<Void>();
        break;
      case 1:
        final $tValue = $value.value as String;
        $result.ref.data = $tValue.asAbiString().cast<Void>();
        break;
      default:
        throw ArgumentError();
    }
  }

  static Pointer<AbiVariant> _$from(idl_types.Types $value) {
    final $result = allocate<AbiVariant>();
    _$fromWithPtr($value, $result);
    return $result;
  }

  static _$dispose(Pointer<AbiVariant> $value) {
    _$disposeWithPtr($value);
    free($value);
  }

  static _$disposeWithPtr(Pointer<AbiVariant> $value) {
    switch ($value.ref.variant!) {
      case 0:
        free($value.ref.data!.cast<Int64>());
        break;
      case 1:
        $value.ref.data!.cast<AbiString>().dispose();
        break;
      default:
        throw ArgumentError();
    }
  }
}

class Test implements idl_types_interface.TestInstance, Disposable {
  ReceivePort? _$toMainTestIntStreamPort;

  SendPort? _$toIsolatePort;

  ReceivePort? _$toMainPort;

  Isolate? _$isolate;

  final HashMap<int, Completer> _$completers = HashMap();

  static final _$InstanceCreateTestFunc _$instanceCreate = _$kLib
      .lookupFunction<_$InstanceCreateTestNative, _$InstanceCreateTestFunc>(
          'idl_test_create_test');

  static final _$InstanceDisposeTestFunc _$instanceDispose = _$kLib
      .lookupFunction<_$InstanceDisposeTestNative, _$InstanceDisposeTestFunc>(
          'idl_test_dispose_test');

  static final _$MethodTestTestIntFunc _$methodTestInt =
      _$kLib.lookupFunction<_$MethodTestTestIntNative, _$MethodTestTestIntFunc>(
          'idl_test_method_test_test_int');

  static final _$MethodTestTestBoolFunc _$methodTestBool = _$kLib
      .lookupFunction<_$MethodTestTestBoolNative, _$MethodTestTestBoolFunc>(
          'idl_test_method_test_test_bool');

  static final _$MethodTestTestFloatFunc _$methodTestFloat = _$kLib
      .lookupFunction<_$MethodTestTestFloatNative, _$MethodTestTestFloatFunc>(
          'idl_test_method_test_test_float');

  static final _$MethodTestTestStringFunc _$methodTestString = _$kLib
      .lookupFunction<_$MethodTestTestStringNative, _$MethodTestTestStringFunc>(
          'idl_test_method_test_test_string');

  static final _$DisposeTestTestStringFunc _$disposeTestString =
      _$kLib.lookupFunction<_$DisposeTestTestStringNative,
          _$DisposeTestTestStringFunc>('idl_test_dispose_test_test_string');

  static final _$MethodTestTestNoneFunc _$methodTestNone = _$kLib
      .lookupFunction<_$MethodTestTestNoneNative, _$MethodTestTestNoneFunc>(
          'idl_test_method_test_test_none');

  static final _$MethodTestTestBytesFunc _$methodTestBytes = _$kLib
      .lookupFunction<_$MethodTestTestBytesNative, _$MethodTestTestBytesFunc>(
          'idl_test_method_test_test_bytes');

  static final _$DisposeTestTestBytesFunc _$disposeTestBytes = _$kLib
      .lookupFunction<_$DisposeTestTestBytesNative, _$DisposeTestTestBytesFunc>(
          'idl_test_dispose_test_test_bytes');

  static final _$MethodTestTestUuidFunc _$methodTestUuid = _$kLib
      .lookupFunction<_$MethodTestTestUuidNative, _$MethodTestTestUuidFunc>(
          'idl_test_method_test_test_uuid');

  static final _$DisposeTestTestUuidFunc _$disposeTestUuid = _$kLib
      .lookupFunction<_$DisposeTestTestUuidNative, _$DisposeTestTestUuidFunc>(
          'idl_test_dispose_test_test_uuid');

  static final _$MethodTestTestIntArrayFunc _$methodTestIntArray =
      _$kLib.lookupFunction<_$MethodTestTestIntArrayNative,
          _$MethodTestTestIntArrayFunc>('idl_test_method_test_test_int_array');

  static final _$DisposeTestTestIntArrayFunc _$disposeTestIntArray =
      _$kLib.lookupFunction<_$DisposeTestTestIntArrayNative,
              _$DisposeTestTestIntArrayFunc>(
          'idl_test_dispose_test_test_int_array');

  static final _$MethodTestTestPointStructFunc _$methodTestPointStruct =
      _$kLib.lookupFunction<_$MethodTestTestPointStructNative,
              _$MethodTestTestPointStructFunc>(
          'idl_test_method_test_test_point_struct');

  static final _$DisposeTestTestPointStructFunc _$disposeTestPointStruct =
      _$kLib.lookupFunction<_$DisposeTestTestPointStructNative,
              _$DisposeTestTestPointStructFunc>(
          'idl_test_dispose_test_test_point_struct');

  static final _$MethodTestTestIntMapFunc _$methodTestIntMap = _$kLib
      .lookupFunction<_$MethodTestTestIntMapNative, _$MethodTestTestIntMapFunc>(
          'idl_test_method_test_test_int_map');

  static final _$DisposeTestTestIntMapFunc _$disposeTestIntMap =
      _$kLib.lookupFunction<_$DisposeTestTestIntMapNative,
          _$DisposeTestTestIntMapFunc>('idl_test_dispose_test_test_int_map');

  static final _$MethodTestTestIntIntMapFunc _$methodTestIntIntMap =
      _$kLib.lookupFunction<_$MethodTestTestIntIntMapNative,
              _$MethodTestTestIntIntMapFunc>(
          'idl_test_method_test_test_int_int_map');

  static final _$DisposeTestTestIntIntMapFunc _$disposeTestIntIntMap =
      _$kLib.lookupFunction<_$DisposeTestTestIntIntMapNative,
              _$DisposeTestTestIntIntMapFunc>(
          'idl_test_dispose_test_test_int_int_map');

  static final _$MethodTestTestTypesFunc _$methodTestTypes = _$kLib
      .lookupFunction<_$MethodTestTestTypesNative, _$MethodTestTestTypesFunc>(
          'idl_test_method_test_test_types');

  static final _$DisposeTestTestTypesFunc _$disposeTestTypes = _$kLib
      .lookupFunction<_$DisposeTestTestTypesNative, _$DisposeTestTestTypesFunc>(
          'idl_test_dispose_test_test_types');

  static final _$MethodTestTestOptionFunc _$methodTestOption = _$kLib
      .lookupFunction<_$MethodTestTestOptionNative, _$MethodTestTestOptionFunc>(
          'idl_test_method_test_test_option');

  static final _$DisposeTestTestOptionFunc _$disposeTestOption =
      _$kLib.lookupFunction<_$DisposeTestTestOptionNative,
          _$DisposeTestTestOptionFunc>('idl_test_dispose_test_test_option');

  static final _$MethodTestTestNoneOptionFunc _$methodTestNoneOption =
      _$kLib.lookupFunction<_$MethodTestTestNoneOptionNative,
              _$MethodTestTestNoneOptionFunc>(
          'idl_test_method_test_test_none_option');

  static final _$DisposeTestTestNoneOptionFunc _$disposeTestNoneOption =
      _$kLib.lookupFunction<_$DisposeTestTestNoneOptionNative,
              _$DisposeTestTestNoneOptionFunc>(
          'idl_test_dispose_test_test_none_option');

  static final _$MethodTestTestResultFunc _$methodTestResult = _$kLib
      .lookupFunction<_$MethodTestTestResultNative, _$MethodTestTestResultFunc>(
          'idl_test_method_test_test_result');

  static final _$DisposeTestTestResultFunc _$disposeTestResult =
      _$kLib.lookupFunction<_$DisposeTestTestResultNative,
          _$DisposeTestTestResultFunc>('idl_test_dispose_test_test_result');

  static final _$MethodTestTestResultErrFunc _$methodTestResultErr =
      _$kLib.lookupFunction<_$MethodTestTestResultErrNative,
              _$MethodTestTestResultErrFunc>(
          'idl_test_method_test_test_result_err');

  static final _$DisposeTestTestResultErrFunc _$disposeTestResultErr =
      _$kLib.lookupFunction<_$DisposeTestTestResultErrNative,
              _$DisposeTestTestResultErrFunc>(
          'idl_test_dispose_test_test_result_err');

  static final _$MethodTestTestIntStreamFunc _$methodTestIntStream =
      _$kLib.lookupFunction<_$MethodTestTestIntStreamNative,
              _$MethodTestTestIntStreamFunc>(
          'idl_test_method_test_test_int_stream');

  static final _$DisposeTestTestIntStreamFunc _$disposeStreamTestIntStream =
      _$kLib.lookupFunction<_$DisposeTestTestIntStreamNative,
              _$DisposeTestTestIntStreamFunc>(
          'idl_test_dispose_stream_test_test_int_stream');

  final _$streamControllersTestIntStream =
      HashMap<int, StreamController<String>>();

  int _$handleTestIntStream = 1;

  static final _$StreamTestTestIntStreamFunc _$streamTestIntStream =
      _$kLib.lookupFunction<_$StreamTestTestIntStreamNative,
              _$StreamTestTestIntStreamFunc>(
          'idl_test_stream_test_test_int_stream');

  static final _$MethodTestTestNamesFunc _$methodTestNames = _$kLib
      .lookupFunction<_$MethodTestTestNamesNative, _$MethodTestTestNamesFunc>(
          'idl_test_method_test_test_names');

  static final _$DisposeTestTestNamesFunc _$disposeTestNames = _$kLib
      .lookupFunction<_$DisposeTestTestNamesNative, _$DisposeTestTestNamesFunc>(
          'idl_test_dispose_test_test_names');

  static final _$MethodTestTestValuesFunc _$methodTestValues = _$kLib
      .lookupFunction<_$MethodTestTestValuesNative, _$MethodTestTestValuesFunc>(
          'idl_test_method_test_test_values');

  @override
  Future<int> testInt(int value) async {
    final $completer = Completer<int>();
    (await _$toIsolate()).send([
      'testInt',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<bool> testBool(bool value) async {
    final $completer = Completer<bool>();
    (await _$toIsolate()).send([
      'testBool',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<double> testFloat(double value) async {
    final $completer = Completer<double>();
    (await _$toIsolate()).send([
      'testFloat',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<String> testString(String value) async {
    final $completer = Completer<String>();
    (await _$toIsolate()).send([
      'testString',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<Null> testNone(Null value) async {
    final $completer = Completer<Null>();
    (await _$toIsolate()).send([
      'testNone',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<Uint8List> testBytes(Uint8List value) async {
    final $completer = Completer<Uint8List>();
    (await _$toIsolate()).send([
      'testBytes',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<String> testUuid(String value) async {
    final $completer = Completer<String>();
    (await _$toIsolate()).send([
      'testUuid',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<List<int>> testIntArray(List<int> value) async {
    final $completer = Completer<List<int>>();
    (await _$toIsolate()).send([
      'testIntArray',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<idl_types.Point> testPointStruct(idl_types.Point value) async {
    final $completer = Completer<idl_types.Point>();
    (await _$toIsolate()).send([
      'testPointStruct',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<Map<int, String>> testIntMap(Map<int, String> value) async {
    final $completer = Completer<Map<int, String>>();
    (await _$toIsolate()).send([
      'testIntMap',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<Map<int, int>> testIntIntMap(Map<int, int> value) async {
    final $completer = Completer<Map<int, int>>();
    (await _$toIsolate()).send([
      'testIntIntMap',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<idl_types.Types> testTypes(idl_types.Types value) async {
    final $completer = Completer<idl_types.Types>();
    (await _$toIsolate()).send([
      'testTypes',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<int?> testOption(int? value) async {
    final $completer = Completer<int?>();
    (await _$toIsolate()).send([
      'testOption',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<int?> testNoneOption() async {
    final $completer = Completer<int?>();
    (await _$toIsolate()).send([
      'testNoneOption',
      $completer.hashCode,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<Result<String, bool>> testResult(String value) async {
    final $completer = Completer<Result<String, bool>>();
    (await _$toIsolate()).send([
      'testResult',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<Result<String, bool>> testResultErr() async {
    final $completer = Completer<Result<String, bool>>();
    (await _$toIsolate()).send([
      'testResultErr',
      $completer.hashCode,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Stream<String> testIntStream() {
    while (
        _$streamControllersTestIntStream.containsKey(_$handleTestIntStream)) {
      _$handleTestIntStream += 1;
    }
    final $handle = _$handleTestIntStream;
    final $controller = StreamController<String>(onListen: () {
      _$toIsolate().then(($value) => $value.send([
            'testIntStream',
            [_$toMainTestIntStreamPort!.sendPort.nativePort, $handle],
          ]));
    }, onCancel: () {
      _$toIsolatePort?.send([
        StreamReceiverState.close,
        $handle,
        'testIntStream',
        _$toMainTestIntStreamPort!.sendPort.nativePort,
        _$toMainTestIntStreamPort!.sendPort,
      ]);
    }, onPause: () {
      _$toIsolatePort?.send([
        StreamReceiverState.pause,
        $handle,
        'testIntStream',
        _$toMainTestIntStreamPort!.sendPort.nativePort,
        _$toMainTestIntStreamPort!.sendPort,
      ]);
    }, onResume: () {
      _$toIsolatePort?.send([
        StreamReceiverState.resume,
        $handle,
        'testIntStream',
        _$toMainTestIntStreamPort!.sendPort.nativePort,
        _$toMainTestIntStreamPort!.sendPort,
      ]);
    });
    _$streamControllersTestIntStream[$handle] = $controller;
    return $controller.stream;
  }

  @override
  Future<String> testNames(String value) async {
    final $completer = Completer<String>();
    (await _$toIsolate()).send([
      'testNames',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  @override
  Future<double> testValues(double value) async {
    final $completer = Completer<double>();
    (await _$toIsolate()).send([
      'testValues',
      $completer.hashCode,
      value,
    ]);
    _$completers[$completer.hashCode] = $completer;
    return $completer.future;
  }

  Future<SendPort> _$toIsolate() async {
    _$toIsolatePort ??= await _$setIsolate();
    return _$toIsolatePort!;
  }

  Future<SendPort> _$setIsolate() async {
    final $completer = Completer<SendPort>();
    _$toMainPort = ReceivePort();
    _$toMainPort!.listen(($data) {
      if ($data is SendPort) {
        $completer.complete($data);
      } else if ($data is List<dynamic>) {
        if ($data[0] is int) {
          _$completers.remove($data[0])!.complete($data[1]);
        }
      }
    });
    _$toMainTestIntStreamPort = ReceivePort();
    _$toMainTestIntStreamPort!.listen(($data) {
      if ($data is int) {
        _$toIsolatePort?.send([
          StreamReceiverState.request,
          $data,
          'testIntStream',
          _$toMainTestIntStreamPort!.sendPort.nativePort,
          _$toMainTestIntStreamPort!.sendPort,
        ]);
      } else if ($data is List<dynamic>) {
        final $handleValue = $data[0] as int;
        if (!_$streamControllersTestIntStream.containsKey($handleValue)) return;
        final $state = StreamSenderState.values[$data[1] as int];
        switch ($state) {
          case StreamSenderState.value:
            _$streamControllersTestIntStream[$handleValue]!
                .add($data[2] as String);
            break;
          case StreamSenderState.done:
            final $controller =
                _$streamControllersTestIntStream.remove($handleValue)!;
            $controller.close();
            break;
          case StreamSenderState.waiting:
            break;
          case StreamSenderState.ok:
            break;
          default:
            throw ArgumentError('Invalid stream state `${$state}`');
        }
      }
    });
    _$isolate = await Isolate.spawn(_$runIsolate, _$toMainPort!.sendPort);
    return $completer.future;
  }

  static void _$runIsolate(SendPort $sendPort) async {
    final $receivePort = ReceivePort();
    final $instance = _$create();
    $sendPort.send($receivePort.sendPort);
    await for (var $data in $receivePort) {
      if ($data is List<dynamic>) {
        if ($data[0] is String) {
          switch ($data[0]) {
            case 'testInt':
              $sendPort.send([
                $data[1] as int,
                (int value) {
                  final $fValue = allocate<Int64>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestInt($instance, value, $fValue),
                        'Test.testInt');
                    final $result = $fValue.value;
                    ;
                    return $result;
                  } finally {
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testBool':
              $sendPort.send([
                $data[1] as int,
                (bool value) {
                  final $fValue = allocate<Int64>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestBool($instance, value ? 1 : 0, $fValue),
                        'Test.testBool');
                    final $result = $fValue.value == 0 ? false : true;
                    ;
                    return $result;
                  } finally {
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testFloat':
              $sendPort.send([
                $data[1] as int,
                (double value) {
                  final $fValue = allocate<Double>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestFloat($instance, value, $fValue),
                        'Test.testFloat');
                    final $result = $fValue.value;
                    ;
                    return $result;
                  } finally {
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testString':
              $sendPort.send([
                $data[1] as int,
                (String value) {
                  final $fValueValue = value.asAbiString();
                  final $fValue = allocate<Pointer<AbiString>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestString($instance, $fValueValue, $fValue),
                        'Test.testString');
                    try {
                      final $result = $fValue.value.asString();
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestString($instance, $fValue.value),
                          'Test.testString');
                    }
                  } finally {
                    $fValueValue.dispose();
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testNone':
              $sendPort.send([
                $data[1] as int,
                (Null value) {
                  final $fValue = allocate<Int64>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestNone($instance, 0, $fValue),
                        'Test.testNone');
                    final $result = null;
                    ;
                    return $result;
                  } finally {
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testBytes':
              $sendPort.send([
                $data[1] as int,
                (Uint8List value) {
                  final $fValueValue = value.asAbiBytes();
                  final $fValue = allocate<Pointer<AbiBytes>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestBytes($instance, $fValueValue, $fValue),
                        'Test.testBytes');
                    try {
                      final $result = $fValue.value.asUint8List();
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestBytes($instance, $fValue.value),
                          'Test.testBytes');
                    }
                  } finally {
                    $fValueValue.dispose();
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testUuid':
              $sendPort.send([
                $data[1] as int,
                (String value) {
                  final $fValueValue = value.asAbiUuid();
                  final $fValue = allocate<Pointer<AbiUuid>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestUuid($instance, $fValueValue, $fValue),
                        'Test.testUuid');
                    try {
                      final $result = $fValue.value.asUuid();
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestUuid($instance, $fValue.value),
                          'Test.testUuid');
                    }
                  } finally {
                    $fValueValue.dispose();
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testIntArray':
              $sendPort.send([
                $data[1] as int,
                (List<int> value) {
                  final $fValueValue = (List<int> $value) {
                    final $result = allocate<AbiArray>();
                    final $data = allocate<Int64>(count: $value.length);
                    final $listSource = $data.asTypedList($value.length);
                    $listSource.setAll(0, $value);
                    $result.ref.data = $data.cast<Void>();
                    $result.ref.length = $value.length;
                    return $result;
                  }(value);
                  final $fValue = allocate<Pointer<AbiArray>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestIntArray($instance, $fValueValue, $fValue),
                        'Test.testIntArray');
                    try {
                      final $result = (Pointer<AbiArray> $value) {
                        final $data = $value.ref.data!;
                        final $length = $value.ref.length!;
                        return $data
                            .cast<Int64>()
                            .asTypedList($length)
                            .toList();
                      }($fValue.value);
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestIntArray($instance, $fValue.value),
                          'Test.testIntArray');
                    }
                  } finally {
                    final $data = $fValueValue.ref.data!.cast<Int64>();
                    free($data);
                    free($fValueValue);
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testPointStruct':
              $sendPort.send([
                $data[1] as int,
                (idl_types.Point value) {
                  final $fValueValue = Point._$from(value);
                  final $fValue = allocate<Pointer<Point>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestPointStruct(
                            $instance, $fValueValue, $fValue),
                        'Test.testPointStruct');
                    try {
                      final $result = Point._$asValue($fValue.value);
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestPointStruct($instance, $fValue.value),
                          'Test.testPointStruct');
                    }
                  } finally {
                    Point._$dispose($fValueValue);
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testIntMap':
              $sendPort.send([
                $data[1] as int,
                (Map<int, String> value) {
                  final $fValueValue = (Map<int, String> $value) {
                    final $result = allocate<AbiMap>();
                    final $entries = $value.entries.toList();
                    final $length = $entries.length;
                    final $values = $entries.map(($m) => $m.value).toList();
                    final $keyValues = $entries.map(($m) => $m.key).toList();
                    final $data = allocate<AbiString>(count: $length);
                    final $key = allocate<Int64>(count: $length);
                    $result.ref.data = $data.cast<Void>();
                    $result.ref.key = $key.cast<Void>();
                    $result.ref.length = $length;
                    for (var $i = 0; $i < $length; $i += 1) {
                      $values[$i].asAbiStringWithPtr($data.elementAt($i));
                    }
                    final $listSourceKey = $key.asTypedList($length);
                    $listSourceKey.setAll(0, $keyValues);
                    return $result;
                  }(value);
                  final $fValue = allocate<Pointer<AbiMap>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestIntMap($instance, $fValueValue, $fValue),
                        'Test.testIntMap');
                    try {
                      final $result = (Pointer<AbiMap> $value) {
                        final $length = $value.ref.length!;
                        final $data = (Pointer<Void> $data, int $length) {
                          final $listSource = $data.cast<AbiString>();
                          final $result = <String>[];
                          for (var $i = 0; $i < $length; $i += 1) {
                            $result.add($listSource.elementAt($i).asString());
                          }
                          return $result;
                        }($value.ref.data!, $length);
                        final $keys = (Pointer<Void> $data, int $length) {
                          return $data
                              .cast<Int64>()
                              .asTypedList($length)
                              .toList();
                        }($value.ref.key!, $length);
                        final $result = <int, String>{};
                        for (var $i = 0; $i < $length; $i += 1) {
                          $result[$keys[$i]] = $data[$i];
                        }
                        return $result;
                      }($fValue.value);
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestIntMap($instance, $fValue.value),
                          'Test.testIntMap');
                    }
                  } finally {
                    final $data = $fValueValue.ref.data!.cast<AbiString>();
                    final $key = $fValueValue.ref.key!.cast<Int64>();
                    for (var $i = 0; $i < $fValueValue.ref.length!; $i += 1) {
                      AbiString.disposeWithPtr($data.elementAt($i));
                    }
                    free($data);
                    free($key);
                    free($fValueValue);
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testIntIntMap':
              $sendPort.send([
                $data[1] as int,
                (Map<int, int> value) {
                  final $fValueValue = (Map<int, int> $value) {
                    final $result = allocate<AbiMap>();
                    final $entries = $value.entries.toList();
                    final $length = $entries.length;
                    final $values = $entries.map(($m) => $m.value).toList();
                    final $keyValues = $entries.map(($m) => $m.key).toList();
                    final $data = allocate<Int64>(count: $length);
                    final $key = allocate<Int64>(count: $length);
                    $result.ref.data = $data.cast<Void>();
                    $result.ref.key = $key.cast<Void>();
                    $result.ref.length = $length;
                    final $listSource = $data.asTypedList($length);
                    $listSource.setAll(0, $values);
                    final $listSourceKey = $key.asTypedList($length);
                    $listSourceKey.setAll(0, $keyValues);
                    return $result;
                  }(value);
                  final $fValue = allocate<Pointer<AbiMap>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestIntIntMap($instance, $fValueValue, $fValue),
                        'Test.testIntIntMap');
                    try {
                      final $result = (Pointer<AbiMap> $value) {
                        final $length = $value.ref.length!;
                        final $data = (Pointer<Void> $data, int $length) {
                          return $data
                              .cast<Int64>()
                              .asTypedList($length)
                              .toList();
                        }($value.ref.data!, $length);
                        final $keys = (Pointer<Void> $data, int $length) {
                          return $data
                              .cast<Int64>()
                              .asTypedList($length)
                              .toList();
                        }($value.ref.key!, $length);
                        final $result = <int, int>{};
                        for (var $i = 0; $i < $length; $i += 1) {
                          $result[$keys[$i]] = $data[$i];
                        }
                        return $result;
                      }($fValue.value);
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestIntIntMap($instance, $fValue.value),
                          'Test.testIntIntMap');
                    }
                  } finally {
                    final $data = $fValueValue.ref.data!.cast<Int64>();
                    final $key = $fValueValue.ref.key!.cast<Int64>();
                    free($data);
                    free($key);
                    free($fValueValue);
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testTypes':
              $sendPort.send([
                $data[1] as int,
                (idl_types.Types value) {
                  final $fValueValue = Types._$from(value);
                  final $fValue = allocate<Pointer<AbiVariant>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestTypes($instance, $fValueValue, $fValue),
                        'Test.testTypes');
                    try {
                      final $result = Types._$asValue($fValue.value);
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestTypes($instance, $fValue.value),
                          'Test.testTypes');
                    }
                  } finally {
                    Types._$dispose($fValueValue);
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testOption':
              $sendPort.send([
                $data[1] as int,
                (int? value) {
                  final $fValueValue = (int? $value) {
                    final $result = allocate<AbiVariant>();
                    if ($value != null) {
                      $result.ref.variant = 0;
                      $result.ref.data = (int $value) {
                        final $result = allocate<Int64>();
                        $result.value = $value;
                        return $result;
                      }($value)
                          .cast<Void>();
                    } else {
                      $result.ref.variant = 1;
                      $result.ref.data = null;
                    }
                    return $result;
                  }(value);
                  final $fValue = allocate<Pointer<AbiVariant>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestOption($instance, $fValueValue, $fValue),
                        'Test.testOption');
                    try {
                      final $result = (Pointer<AbiVariant> $value) {
                        switch ($value.ref.variant!) {
                          case 0:
                            return $value.ref.data!.cast<Int64>().value;
                          case 1:
                            return null;
                          default:
                            throw ArgumentError();
                        }
                      }($fValue.value);
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestOption($instance, $fValue.value),
                          'Test.testOption');
                    }
                  } finally {
                    if ($fValueValue.ref.variant == 0) {
                      free($fValueValue.ref.data!.cast<Int64>());
                    }
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testNoneOption':
              $sendPort.send([
                $data[1] as int,
                () {
                  final $fValue = allocate<Pointer<AbiVariant>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestNoneOption($instance, $fValue),
                        'Test.testNoneOption');
                    try {
                      final $result = (Pointer<AbiVariant> $value) {
                        switch ($value.ref.variant!) {
                          case 0:
                            return $value.ref.data!.cast<Int64>().value;
                          case 1:
                            return null;
                          default:
                            throw ArgumentError();
                        }
                      }($fValue.value);
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestNoneOption($instance, $fValue.value),
                          'Test.testNoneOption');
                    }
                  } finally {
                    free($fValue);
                  }
                }()
              ]);
              break;
            case 'testResult':
              $sendPort.send([
                $data[1] as int,
                (String value) {
                  final $fValueValue = value.asAbiString();
                  final $fValue = allocate<Pointer<AbiVariant>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestResult($instance, $fValueValue, $fValue),
                        'Test.testResult');
                    try {
                      final $result = (Pointer<AbiVariant> $value) {
                        switch ($value.ref.variant!) {
                          case 0:
                            return Result<String, bool>.ok(
                                $value.ref.data!.cast<AbiString>().asString());
                          case 1:
                            return Result<String, bool>.err(
                                $value.ref.data!.cast<Int64>().value == 0
                                    ? false
                                    : true);
                          default:
                            throw ArgumentError();
                        }
                      }($fValue.value);
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestResult($instance, $fValue.value),
                          'Test.testResult');
                    }
                  } finally {
                    $fValueValue.dispose();
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testResultErr':
              $sendPort.send([
                $data[1] as int,
                () {
                  final $fValue = allocate<Pointer<AbiVariant>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestResultErr($instance, $fValue),
                        'Test.testResultErr');
                    try {
                      final $result = (Pointer<AbiVariant> $value) {
                        switch ($value.ref.variant!) {
                          case 0:
                            return Result<String, bool>.ok(
                                $value.ref.data!.cast<AbiString>().asString());
                          case 1:
                            return Result<String, bool>.err(
                                $value.ref.data!.cast<Int64>().value == 0
                                    ? false
                                    : true);
                          default:
                            throw ArgumentError();
                        }
                      }($fValue.value);
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestResultErr($instance, $fValue.value),
                          'Test.testResultErr');
                    }
                  } finally {
                    free($fValue);
                  }
                }()
              ]);
              break;
            case 'testIntStream':
              ($wakePort, $wakeObject) {
                final $streamValue = allocate<AbiStream>();
                try {
                  $streamValue.ref.wakeHandle = $wakePort;
                  $streamValue.ref.wakeObject =
                      DartCObjectInt.newObject($wakeObject).cast();
                  $streamValue.ref.wakeCallback = NativeApi.postCObject.cast();
                  $streamValue.ref.state = AbiStreamReceiverState.start;
                  AbiInternalError.handleError(
                      _$methodTestIntStream($instance, $streamValue),
                      'Test.testIntStream');
                } finally {
                  free($streamValue);
                }
              }($data[1][0] as int, $data[1][1] as int);
              break;
            case 'testNames':
              $sendPort.send([
                $data[1] as int,
                (String value) {
                  final $fValueValue = value.asAbiString();
                  final $fValue = allocate<Pointer<AbiString>>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestNames($instance, $fValueValue, $fValue),
                        'Test.testNames');
                    try {
                      final $result = $fValue.value.asString();
                      return $result;
                    } finally {
                      AbiInternalError.handleError(
                          _$disposeTestNames($instance, $fValue.value),
                          'Test.testNames');
                    }
                  } finally {
                    $fValueValue.dispose();
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            case 'testValues':
              $sendPort.send([
                $data[1] as int,
                (double value) {
                  final $fValue = allocate<Double>();
                  try {
                    AbiInternalError.handleError(
                        _$methodTestValues($instance, value, $fValue),
                        'Test.testValues');
                    final $result = $fValue.value;
                    ;
                    return $result;
                  } finally {
                    free($fValue);
                  }
                }($data[2])
              ]);
              break;
            default:
              throw ArgumentError('Invalid port args');
          }
        } else if ($data[0] is StreamReceiverState) {
          switch ($data[2] as String) {
            case 'testIntStream':
              (int $streamState, int $wakeObject, int $wakePort,
                      SendPort $sendPort) {
                final $streamValue = allocate<AbiStream>();
                final $fValue = allocate<Pointer<AbiStream>>();
                try {
                  $streamValue.ref.wakeHandle = $wakePort;
                  $streamValue.ref.wakeObject =
                      DartCObjectInt.newObject($wakeObject).cast();
                  $streamValue.ref.wakeCallback = NativeApi.postCObject.cast();
                  $streamValue.ref.state = $streamState;
                  AbiInternalError.handleError(
                      _$streamTestIntStream($instance, $streamValue, $fValue),
                      'Test.testIntStream');
                  try {
                    final $stream = $fValue.value;
                    switch ($stream.ref.state!) {
                      case AbiStreamSenderState.ok:
                        break;
                      case AbiStreamSenderState.done:
                        $sendPort.send([$wakeObject, $stream.ref.state!, 0]);
                        break;
                      case AbiStreamSenderState.value:
                        final $result = $stream.ref.data!
                            .cast<Pointer<AbiString>>()
                            .value
                            .asString();
                        $sendPort
                            .send([$wakeObject, $stream.ref.state!, $result]);
                        break;
                      default:
                        throw ArgumentError();
                    }
                  } finally {
                    AbiInternalError.handleError(
                        _$disposeStreamTestIntStream($instance, $fValue.value),
                        'Test.testIntStream');
                  }
                } finally {
                  free($streamValue);
                  free($fValue);
                }
              }(($data[0] as StreamReceiverState).index, $data[1] as int,
                  $data[3] as int, $data[4] as SendPort);
              break;
            default:
              throw ArgumentError('Invalid port args');
          }
        }
      } else {
        break;
      }
    }
    AbiInternalError.handleError(_$instanceDispose($instance), 'Test');
  }

  static Pointer<_Test> _$create() {
    final $result = allocate<Pointer<_Test>>();
    try {
      AbiInternalError.handleError(_$instanceCreate($result), 'Test');
      return $result.value;
    } finally {
      free($result);
    }
  }

  @override
  void dispose() {
    _$toIsolatePort?.send(false);
    _$toMainPort?.close();
    _$isolate?.kill();
    _$toMainTestIntStreamPort?.close();
    _$toMainTestIntStreamPort = null;
    _$toIsolatePort = null;
    _$toMainPort = null;
    _$isolate = null;
  }
}

class _Test extends Struct {}

class Point extends Struct {
  @Int64()
  int? x;

  @Int64()
  int? y;

  static idl_types.Point _$asValue(Pointer<Point> $value) =>
      idl_types.Point($value.ref.x!, $value.ref.y!);
  static void _$fromWithPtr(idl_types.Point $value, Pointer<Point> $result) {
    $result.ref.x = $value.x;
    $result.ref.y = $value.y;
  }

  static Pointer<Point> _$from(idl_types.Point $value) {
    final $result = allocate<Point>();
    _$fromWithPtr($value, $result);
    return $result;
  }

  static _$dispose(Pointer<Point> $value) {
    _$disposeWithPtr($value);
    free($value);
  }

  static _$disposeWithPtr(Pointer<Point> $value) {}
}

typedef _$InstanceCreateTestNative = Int64 Function(Pointer<Pointer<_Test>>);
typedef _$InstanceCreateTestFunc = int Function(Pointer<Pointer<_Test>>);
typedef _$InstanceDisposeTestNative = Int64 Function(Pointer<_Test>);
typedef _$InstanceDisposeTestFunc = int Function(Pointer<_Test>);
typedef _$MethodTestTestIntNative = Int64 Function(
    Pointer<_Test>, Int64, Pointer<Int64>);
typedef _$MethodTestTestIntFunc = int Function(
    Pointer<_Test>, int, Pointer<Int64>);
typedef _$MethodTestTestBoolNative = Int64 Function(
    Pointer<_Test>, Int64, Pointer<Int64>);
typedef _$MethodTestTestBoolFunc = int Function(
    Pointer<_Test>, int, Pointer<Int64>);
typedef _$MethodTestTestFloatNative = Int64 Function(
    Pointer<_Test>, Double, Pointer<Double>);
typedef _$MethodTestTestFloatFunc = int Function(
    Pointer<_Test>, double, Pointer<Double>);
typedef _$DisposeTestTestStringNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiString>);
typedef _$DisposeTestTestStringFunc = int Function(
    Pointer<_Test>, Pointer<AbiString>);
typedef _$MethodTestTestStringNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiString>, Pointer<Pointer<AbiString>>);
typedef _$MethodTestTestStringFunc = int Function(
    Pointer<_Test>, Pointer<AbiString>, Pointer<Pointer<AbiString>>);
typedef _$MethodTestTestNoneNative = Int64 Function(
    Pointer<_Test>, Int64, Pointer<Int64>);
typedef _$MethodTestTestNoneFunc = int Function(
    Pointer<_Test>, int, Pointer<Int64>);
typedef _$DisposeTestTestBytesNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiBytes>);
typedef _$DisposeTestTestBytesFunc = int Function(
    Pointer<_Test>, Pointer<AbiBytes>);
typedef _$MethodTestTestBytesNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiBytes>, Pointer<Pointer<AbiBytes>>);
typedef _$MethodTestTestBytesFunc = int Function(
    Pointer<_Test>, Pointer<AbiBytes>, Pointer<Pointer<AbiBytes>>);
typedef _$DisposeTestTestUuidNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiUuid>);
typedef _$DisposeTestTestUuidFunc = int Function(
    Pointer<_Test>, Pointer<AbiUuid>);
typedef _$MethodTestTestUuidNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiUuid>, Pointer<Pointer<AbiUuid>>);
typedef _$MethodTestTestUuidFunc = int Function(
    Pointer<_Test>, Pointer<AbiUuid>, Pointer<Pointer<AbiUuid>>);
typedef _$DisposeTestTestIntArrayNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiArray>);
typedef _$DisposeTestTestIntArrayFunc = int Function(
    Pointer<_Test>, Pointer<AbiArray>);
typedef _$MethodTestTestIntArrayNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiArray>, Pointer<Pointer<AbiArray>>);
typedef _$MethodTestTestIntArrayFunc = int Function(
    Pointer<_Test>, Pointer<AbiArray>, Pointer<Pointer<AbiArray>>);
typedef _$DisposeTestTestPointStructNative = Int64 Function(
    Pointer<_Test>, Pointer<Point>);
typedef _$DisposeTestTestPointStructFunc = int Function(
    Pointer<_Test>, Pointer<Point>);
typedef _$MethodTestTestPointStructNative = Int64 Function(
    Pointer<_Test>, Pointer<Point>, Pointer<Pointer<Point>>);
typedef _$MethodTestTestPointStructFunc = int Function(
    Pointer<_Test>, Pointer<Point>, Pointer<Pointer<Point>>);
typedef _$DisposeTestTestIntMapNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiMap>);
typedef _$DisposeTestTestIntMapFunc = int Function(
    Pointer<_Test>, Pointer<AbiMap>);
typedef _$MethodTestTestIntMapNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiMap>, Pointer<Pointer<AbiMap>>);
typedef _$MethodTestTestIntMapFunc = int Function(
    Pointer<_Test>, Pointer<AbiMap>, Pointer<Pointer<AbiMap>>);
typedef _$DisposeTestTestIntIntMapNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiMap>);
typedef _$DisposeTestTestIntIntMapFunc = int Function(
    Pointer<_Test>, Pointer<AbiMap>);
typedef _$MethodTestTestIntIntMapNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiMap>, Pointer<Pointer<AbiMap>>);
typedef _$MethodTestTestIntIntMapFunc = int Function(
    Pointer<_Test>, Pointer<AbiMap>, Pointer<Pointer<AbiMap>>);
typedef _$DisposeTestTestTypesNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$DisposeTestTestTypesFunc = int Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$MethodTestTestTypesNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiVariant>, Pointer<Pointer<AbiVariant>>);
typedef _$MethodTestTestTypesFunc = int Function(
    Pointer<_Test>, Pointer<AbiVariant>, Pointer<Pointer<AbiVariant>>);
typedef _$DisposeTestTestOptionNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$DisposeTestTestOptionFunc = int Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$MethodTestTestOptionNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiVariant>, Pointer<Pointer<AbiVariant>>);
typedef _$MethodTestTestOptionFunc = int Function(
    Pointer<_Test>, Pointer<AbiVariant>, Pointer<Pointer<AbiVariant>>);
typedef _$DisposeTestTestNoneOptionNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$DisposeTestTestNoneOptionFunc = int Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$MethodTestTestNoneOptionNative = Int64 Function(
    Pointer<_Test>, Pointer<Pointer<AbiVariant>>);
typedef _$MethodTestTestNoneOptionFunc = int Function(
    Pointer<_Test>, Pointer<Pointer<AbiVariant>>);
typedef _$DisposeTestTestResultNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$DisposeTestTestResultFunc = int Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$MethodTestTestResultNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiString>, Pointer<Pointer<AbiVariant>>);
typedef _$MethodTestTestResultFunc = int Function(
    Pointer<_Test>, Pointer<AbiString>, Pointer<Pointer<AbiVariant>>);
typedef _$DisposeTestTestResultErrNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$DisposeTestTestResultErrFunc = int Function(
    Pointer<_Test>, Pointer<AbiVariant>);
typedef _$MethodTestTestResultErrNative = Int64 Function(
    Pointer<_Test>, Pointer<Pointer<AbiVariant>>);
typedef _$MethodTestTestResultErrFunc = int Function(
    Pointer<_Test>, Pointer<Pointer<AbiVariant>>);
typedef _$DisposeTestTestIntStreamNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiStream>);
typedef _$DisposeTestTestIntStreamFunc = int Function(
    Pointer<_Test>, Pointer<AbiStream>);
typedef _$MethodTestTestIntStreamNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiStream>);
typedef _$MethodTestTestIntStreamFunc = int Function(
    Pointer<_Test>, Pointer<AbiStream>);
typedef _$StreamTestTestIntStreamNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiStream>, Pointer<Pointer<AbiStream>>);
typedef _$StreamTestTestIntStreamFunc = int Function(
    Pointer<_Test>, Pointer<AbiStream>, Pointer<Pointer<AbiStream>>);
typedef _$DisposeTestTestNamesNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiString>);
typedef _$DisposeTestTestNamesFunc = int Function(
    Pointer<_Test>, Pointer<AbiString>);
typedef _$MethodTestTestNamesNative = Int64 Function(
    Pointer<_Test>, Pointer<AbiString>, Pointer<Pointer<AbiString>>);
typedef _$MethodTestTestNamesFunc = int Function(
    Pointer<_Test>, Pointer<AbiString>, Pointer<Pointer<AbiString>>);
typedef _$MethodTestTestValuesNative = Int64 Function(
    Pointer<_Test>, Double, Pointer<Double>);
typedef _$MethodTestTestValuesFunc = int Function(
    Pointer<_Test>, double, Pointer<Double>);