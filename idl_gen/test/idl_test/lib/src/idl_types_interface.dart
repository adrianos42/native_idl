import 'idl_types.dart' as idl_types;
import 'dart:typed_data';
import 'dart:collection';
import 'package:idl_internal/idl_internal.dart';

abstract class TestInstance {
  Future<int> testInt(int value);
  Future<bool> testBool(bool value);
  Future<double> testFloat(double value);
  Future<String> testString(String value);
  Future<Null> testNone(Null value);
  Future<Uint8List> testBytes(Uint8List value);
  Future<String> testUuid(String value);
  Future<List<int>> testIntArray(List<int> value);
  Future<idl_types.Point> testPointStruct(idl_types.Point value);
  Future<Map<int, String>> testIntMap(Map<int, String> value);
  Future<Map<int, int>> testIntIntMap(Map<int, int> value);
  Future<idl_types.Types> testTypes(idl_types.Types value);
  Future<int?> testOption(int? value);
  Future<int?> testNoneOption();
  Future<Result<String, bool>> testResult(String value);
  Future<Result<String, bool>> testResultErr();
  Stream<String> testIntStream();
  Future<String> testNames(String value);
  Future<double> testValues(double value);
}