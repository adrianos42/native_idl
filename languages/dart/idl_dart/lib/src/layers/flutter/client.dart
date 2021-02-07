import 'dart:convert';

import 'package:idl_dart/src/pubspec/dependency.dart';
import 'package:idl_dart/src/pubspec/pubspec.dart';
import 'package:pub_semver/pub_semver.dart';

import 'dart:io';
import 'package:path/path.dart';
import 'package:tuple/tuple.dart';

import '../../json/ids_types_json.dart' as ids_types;
import '../../json/idl_json.dart';
import '../../module.dart';
import '../../dart_layer.dart';
import '../../string_pos.dart';

const _kFlutterCreateWhitelist = [
  'linux',
  'android',
  'ios',
  'web',
];

class FlutterLayer implements DartIdlLayer {
  @override
  LayerItem generate(
    Module module,
    bool endPoint,
    List<DartIdlLayer> subLayers,
  ) {
    if (subLayers.isEmpty) {
      throw ArgumentError('Need sublayer for flutter.');
    }

    final result = List<StorageItem>.empty(growable: true);

   // result.addAll([createFolderItem('lib', [])]);

    final flutterCreate = _createPluginFiles(module);
    final flutterPubspec = flutterCreate.item2;
    result.addAll(flutterCreate.item1);

    return LayerItem(
        storageItems: result,
        pubspec: Pubspec(
          flutterPubspec.name,
          flutter: flutterPubspec.flutter,
          dependencies: flutterPubspec.dependencies,
          devDependencies: flutterPubspec.devDependencies,
          environment: flutterPubspec.environment,
        )).generateNext(module, subLayers, true);
  }

  Tuple2<List<StorageItem>, Pubspec> _createPluginFiles(Module module) {
    // If a `pubspec` is not found, then an exception will be thrown when trying to merge.
    var pubspec = Pubspec('');
    final result = <StorageItem>[];
    final tmpDir = Directory.systemTemp.createTempSync('nativeidl');
    final platformList =
        ['linux'].fold('', (String p, String v) => p.isNotEmpty ? '$p,$v' : v);

    final args = <String>[
      'create',
      '--platforms=$platformList',
      '--template=plugin',
      module.packageName,
    ];

    final output = Process.runSync(
      'flutter',
      args,
      workingDirectory: tmpDir.path,
    );

    if (output.stderr.isNotEmpty) {
      throw Exception(output.stderr);
    } else {
      module.addResponseMessage(output.stdout);
    }

    final targetFolder = Directory.fromUri(tmpDir.listSync().first.uri);
    final folderList = targetFolder.listSync();

    for (var item in folderList) {
      final name = basename(item.path);
      if (name == 'pubspec.yaml') {
        pubspec = Pubspec.parse(File.fromUri(item.uri).readAsStringSync());
      } else if (name == 'linux' &&
          FileSystemEntity.isDirectorySync(item.path)) {
        result.add(createLinuxFolder(module, Directory.fromUri(item.uri)));
      } else if (_kFlutterCreateWhitelist.contains(name)) {
        if (FileSystemEntity.isDirectorySync(item.path)) {
          result.add(createFolderItemFromPath(Directory.fromUri(item.uri)));
        } else if (FileSystemEntity.isFileSync(item.path)) {
          result.add(createSourceItemFromPath(File.fromUri(item.uri)));
        }
      }
    }

    targetFolder.deleteSync(recursive: true);
    tmpDir.deleteSync();

    final newVersion = VersionConstraint.parse('>=2.12.0-0 <3.0.0');
    pubspec.environment
            ?.update('sdk', (_) => newVersion, ifAbsent: () => newVersion) ??
        {'sdk': newVersion};

    return Tuple2(result, pubspec);
  }

  StorageItem createLinuxFolder(Module module, Directory dir) {
    final folderList = dir.listSync();
    final result = <StorageItem>[];

    for (var item in folderList) {
      if (FileSystemEntity.isDirectorySync(item.path)) {
        result.add(createFolderItemFromPath(Directory.fromUri(item.uri)));
      } else if (FileSystemEntity.isFileSync(item.path)) {
        final cmakeName = 'CMakeLists.txt';
        if (basename(item.path) == cmakeName) {
          var cmakeSource = File.fromUri(item.uri).readAsStringSync();
          cmakeSource += '''
target_link_directories(\${PLUGIN_NAME} PRIVATE "\${CMAKE_CURRENT_SOURCE_DIR}/../build/idl/")
target_link_libraries(\${PLUGIN_NAME} PRIVATE ${module.packageName})''';
          result.add(createSourceItem(cmakeName, cmakeSource));
        } else {
          result.add(createSourceItemFromPath(File.fromUri(item.uri)));
        }
      }
    }
    return createFolderItem('linux', result);
  }

  @override
  ids_types.Layer get layer => _layer;

  final ids_types.Layer _layer;

  FlutterLayer(ids_types.Layer layer) : _layer = layer;
}
