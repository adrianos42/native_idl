import 'idl_ffi.dart' as idl_ffi;
import 'idl_types_interface.dart' as idl_types_interface;

class TestConstructor {
  static idl_types_interface.TestInstance createInstance() => idl_ffi.Test();
}