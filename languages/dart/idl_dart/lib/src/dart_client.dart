import 'module.dart';
import 'json/idl_json.dart';
import 'layers/ffi/client.dart';
import 'layers/flutter/client.dart';
import 'dart_types.dart';
import 'dart_layer.dart';
import 'json/ids_types_json.dart' as ids_types;

final _kLayersBuilders = {
  'FFI': (layer) => FFILayer(layer),
  'Flutter': (layer) => FlutterLayer(layer),
};

class DartClient {
  static List<StorageItem> generate(Module module) {
    var clientLayers;
    try {
      clientLayers =
          Module.getClientField<ids_types.Values>(module.targetClient, 'layers')
              .values;
    } catch (exception) {
      // TODO
      clientLayers = ids_types.Values(values: List.empty());
    }

    final dartLayers = [
      ...clientLayers,
      Module.getServerField<ids_types.Values>(module.targetServer, 'layers')
          .values
          .last, // Only the last layer is considered to generate the client side code.
    ].map((f) {
      final name = f.itemType as ids_types.LayerTypeName;
      final layer = module.findLayer(name);

      final result = _kLayersBuilders[name.layerTypeName];

      if (result != null) {
        return result(layer);
      }

      throw ArgumentError('Layer `${name.layerTypeName}` not supported');
    }).toList();

    return ClientTypes.generate(module).withSublayer(dartLayers.first
        .generate(module, false, dartLayers.sublist(1))).merge(module);
  }
}