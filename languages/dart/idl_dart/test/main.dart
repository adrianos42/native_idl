// @dart=2.9
import 'package:idl_dart/src/pubspec/dependency.dart';
import 'package:idl_dart/src/pubspec/pubspec.dart';
import 'package:pub_semver/pub_semver.dart';
import 'package:yaml/yaml.dart';

void main() {
  final libraryName = 'calc_manager';

  final environment = '>=2.12.0-0 <3.0.0';
  final description = '$libraryName library.';
  final version = '1.0.0';

  final result = Pubspec(
    libraryName,
    version: Version.parse(version),
    description: description,
    environment: {'sdk': VersionConstraint.parse(environment)},
    dependencies: {
      'ffi':
          Dependency(version: VersionConstraint.parse('^0.2.0-nullsafety.1')),
      'idl_internal':
          Dependency(version: VersionConstraint.parse('^0.1.2-3.12.0-0')),
      'pfpf': GitDependency(
          url: Uri.parse('https://github.com/alexei-sintotski/pubspec_yaml'))
    },
  );

final txtTest = '''
name: calc_manager
description: A new flutter plugin project.
version: 0.0.1
author:
homepage:

environment:
  sdk: ">=2.7.0 <3.0.0"
  flutter: ">=1.20.0"

dependencies:
  flutter:
    sdk: flutter

dev_dependencies:
  flutter_test:
    sdk: flutter

flutter:
  plugin:
    platforms:
      ios:
        pluginClass: CalcManagerPlugin
      linux:
        pluginClass: CalcManagerPlugin
  assets:
    - images/a_dot_burr.jpeg
    - images/a_dot_ham.jpeg
  
''';


  print(Pubspec.parse(txtTest).toString());

}
