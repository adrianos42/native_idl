import 'pubspec/pubspec.dart';

import 'json/idl_json.dart';
import 'json/ids_types_json.dart' as ids_types;
import 'module.dart';
import 'string_pos.dart';
import 'dart_types.dart';

abstract class DartIdlLayer {
  // Generates all the files, including the pubspec and library file.
  LayerItem generate(Module module, bool endPoint, List<DartIdlLayer> subLayer);
  ids_types.Layer get layer;
}

class LayerItem {
  final List<StorageItem> storageItems;
  final Pubspec? pubspec;
  final LayerItem? sublayer;

  LayerItem({required this.storageItems, required this.pubspec, this.sublayer});

  LayerItem withSublayer(LayerItem sublayer) {
    if (this.sublayer != null) {
      throw Exception('`sublayer` already defined');
    }

    return LayerItem(
        storageItems: storageItems, pubspec: pubspec, sublayer: sublayer);
  }

  LayerItem generateNext(
    Module module,
    List<DartIdlLayer> subLayers,
    bool endPoint,
  ) {
    if (subLayers.isEmpty) return this;
    final nextLayer = subLayers.removeAt(0);
    return withSublayer(nextLayer.generate(module, endPoint, subLayers));
  }

  List<StorageItem> merge(Module module) {
    final pubspecResult = mergePubspec(Pubspec(module.libraryName));

    final result = List<StorageItem>.empty(growable: true);

    result.addAll(mergeStorageItems([]));
    result.add(createSourceItem('pubspec.yaml', pubspecResult.toString()));

    return result;
  }

  List<StorageItem> mergeStorageItems(List<StorageItem> value) {
    final result = storageItems + value;
    return sublayer?.mergeStorageItems(result) ?? result;
  }

  Pubspec mergePubspec(Pubspec value) {
    final result = pubspec?.merge(value) ?? value;
    return sublayer?.mergePubspec(result) ?? result;
  }
}

List<StorageItem> createItemsForLayer(
  Module module,
  String layerName,
  bool endPoint, {
  required String interfaceContructors,
  required String layerTypes,
}) {
  final result = List<StorageItem>.empty(growable: true);

  final layerReferenceName = layerName.toLowerCase();

  if (endPoint) {
    result.addAll([
      createFolderItem('lib', [
        createFolderItem('src', [
          createSourceItem(
              'idl_interface_constructor.dart', interfaceContructors),
        ]),
      ])
    ]);
  } else {
    throw Exception('fpfp');
    final dartTypes =
        DartTypes.generateInterfaceOnlyLayer(module, layerReferenceName);

    result.addAll([
      createFolderItem('lib', [
        createFolderItem('src', [
          createSourceItem(
              'idl_${layerReferenceName}_types.dart', dartTypes.toString()),
          createSourceItem(
              'idl_${layerReferenceName}_interface_constructor.dart',
              interfaceContructors),
        ]),
      ]),
    ]);
  }

  result.addAll([
    createFolderItem('lib', [
      createFolderItem('src', [
        createSourceItem('idl_${layerReferenceName}.dart', layerTypes),
      ]),
    ])
  ]);

  return result;
}
