import 'idl_types_interface.dart' as idl_types_interface;
import 'idl_interface_constructor.dart' as idl_interface_constructor;
import 'dart:typed_data';
import 'dart:collection';
import 'package:idl_internal/idl_internal.dart';

class Names {
  static final String name = 'Adriano';

  static final String surname = 'Souza';
}

class ValuesFloat {
  static final double pi = 3.14;

  static final double sqrt = 2.55;

  static final double onedw = 122332.0;
}

class ValuesInteger {
  static final int one = 1;

  static final int two = 2;

  static final int three = 3;
}

class Types {
  factory Types.asIntType(int value) => Types._(value, Types.intType);

  factory Types.asStringType(String value) => Types._(value, Types.stringType);

  const Types._(this._value, this._variant);

  static const int intType = 0;

  static const int stringType = 1;

  final dynamic _value;

  final int _variant;

  dynamic get value => _value;
  int get variant => _variant;
  bool operator ==(dynamic other) =>
      variant == other.variant && value == other.value;
}

class Test implements Disposable {
  idl_types_interface.TestInstance? _instance =
      idl_interface_constructor.TestConstructor.createInstance();

  Future<int> testInt(int value) => _instance!.testInt(value);
  Future<bool> testBool(bool value) => _instance!.testBool(value);
  Future<double> testFloat(double value) => _instance!.testFloat(value);
  Future<String> testString(String value) => _instance!.testString(value);
  Future<Null> testNone(Null value) => _instance!.testNone(value);
  Future<Uint8List> testBytes(Uint8List value) => _instance!.testBytes(value);
  Future<String> testUuid(String value) => _instance!.testUuid(value);
  Future<List<int>> testIntArray(List<int> value) =>
      _instance!.testIntArray(value);
  Future<Point> testPointStruct(Point value) =>
      _instance!.testPointStruct(value);
  Future<Map<int, String>> testIntMap(Map<int, String> value) =>
      _instance!.testIntMap(value);
  Future<Map<int, int>> testIntIntMap(Map<int, int> value) =>
      _instance!.testIntIntMap(value);
  Future<Types> testTypes(Types value) => _instance!.testTypes(value);
  Future<int?> testOption(int? value) => _instance!.testOption(value);
  Future<int?> testNoneOption() => _instance!.testNoneOption();
  Future<Result<String, bool>> testResult(String value) =>
      _instance!.testResult(value);
  Future<Result<String, bool>> testResultErr() => _instance!.testResultErr();
  Stream<String> testIntStream() => _instance!.testIntStream();
  Future<String> testNames(String value) => _instance!.testNames(value);
  Future<double> testValues(double value) => _instance!.testValues(value);
  @override
  dispose() {
    (_instance! as Disposable).dispose();
    _instance = null;
  }
}

class Point {
  const Point(this.x, this.y);

  final int x;

  final int y;

  @override
  bool operator ==(dynamic other) => other.x == x && other.y == y;
}