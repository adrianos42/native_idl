import 'package:tuple/tuple.dart';

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
    final pubspecResult = mergePubspec(Pubspec(module.packageName));

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

class LayerItemCreation {
  final PackageLibrary packageLibrary;
  final String interfaceConstructors;
  final String layerTypes;
  const LayerItemCreation({
    required this.interfaceConstructors,
    required this.layerTypes,
    required this.packageLibrary,
  });
}

List<StorageItem> createItemsForLayer(
  Module module,
  String layerName,
  bool endPoint,
  List<LayerItemCreation> items,
) {
  final result = List<StorageItem>.empty(growable: true);

  final layerReferenceName = layerName.toLowerCase();

  for (var item in items) {
    if (item.packageLibrary.hasInterface) {
      // TODO here??
      final interfaceConstructorsSources =
          List<StorageItem>.empty(growable: true);

      if (endPoint) {
        interfaceConstructorsSources.addAll([
          createSourceItem(
              'idl_interface_constructor.dart', item.interfaceConstructors),
          createSourceItem('idl_${layerReferenceName}.dart', item.layerTypes),
        ]);
      } else {
        final dartTypes = DartTypes.generateInterfaceOnlyLayer(
            item.packageLibrary, layerReferenceName);

        interfaceConstructorsSources.addAll([
          createSourceItem(
              'idl_${layerReferenceName}_types.dart', dartTypes.toString()),
          createSourceItem(
            'idl_${layerReferenceName}_interface_constructor.dart',
            item.interfaceConstructors,
          ),
          createSourceItem('idl_${layerReferenceName}.dart', item.layerTypes),
        ]);
      }

      if (item.packageLibrary.libraryName == module.packageName) {
        result.add(createFolderItem('lib', [
          createFolderItem(
            'src',
            interfaceConstructorsSources,
          )
        ]));
      } else {
        result.add(createFolderItem('lib', [
          createFolderItem('src', [
            createFolderItem(
              item.packageLibrary.libraryName,
              interfaceConstructorsSources,
            )
          ])
        ]));
      }
    }
  }

  return result;
}
