import 'package:code_builder/code_builder.dart';
import 'package:dart_style/dart_style.dart';
import 'package:idl_dart/src/pubspec/dependency.dart';
import 'package:pub_semver/pub_semver.dart';
import 'pubspec/pubspec.dart';

import 'json/idl_json.dart';
import 'module.dart';

class DartSpec {
  static Pubspec generate(Module module) {
    final libraryName = module.packageName;

    final environment = '>=2.12.0-0 <3.0.0';
    final description = '$libraryName library.';
    final version = Version.parse('1.0.0');

    final result = Pubspec(
      libraryName,
      version: version,
      description: description,
      environment: {'sdk': VersionConstraint.parse(environment)},
      dependencies: {
        'idl_internal':
            Dependency(version: VersionConstraint.parse('^0.1.8-2.12.0-0'))
      },
    );

    return result;
  }
}

class DartLib {
  DartLib._();

  factory DartLib.generate(Module module, PackageLibrary packageLibrary) {
    final result = DartLib._();
    final libraryName = packageLibrary.libraryName;

    final exportName = module.packageName == packageLibrary.libraryName
        ? '''export 'src/idl_types.dart';'''
        : '''export 'src/${packageLibrary.libraryName}/idl_types.dart';''';

    result._library = Code('''
      library ${libraryName};
      $exportName
      export 'package:idl_internal/idl_internal.dart' show Result, Pair;
    ''');

    return result;
  }

  @override
  String toString() {
    final emitter = DartEmitter();
    return DartFormatter().format('${_library.accept(emitter)}').trim();
  }

  late Code _library;
}
