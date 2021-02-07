import 'dart:convert';

import 'package:json_annotation/json_annotation.dart';

part 'ids_types_json.g.dart';

class ItemType<T> {
  final T itemType;

  const ItemType({required this.itemType});

  factory ItemType.fromJson(Map<String, dynamic> json) {
    if (json.containsKey('NatInt')) {
      return ItemType(itemType: NatInt.fromJson(json) as T);
    }

    if (json.containsKey('NatFloat')) {
      return ItemType(itemType: NatFloat.fromJson(json) as T);
    }

    if (json.containsKey('NatString')) {
      return ItemType(itemType: NatString.fromJson(json) as T);
    }

    if (json.containsKey('NatBool')) {
      return ItemType(itemType: NatBool.fromJson(json) as T);
    }

    if (json.containsKey('LayerTypeName')) {
      return ItemType(itemType: LayerTypeName.fromJson(json) as T);
    }

    if (json.containsKey('ClientTypeName')) {
      return ItemType(itemType: ClientTypeName.fromJson(json) as T);
    }

    if (json.containsKey('ServerTypeName')) {
      return ItemType(itemType: ServerTypeName.fromJson(json) as T);
    }

    if (json.containsKey('Values')) {
      return ItemType(itemType: Values.fromJson(json) as T);
    }

    if (json.containsKey('Identifier')) {
      return ItemType(itemType: Identifier.fromJson(json) as T);
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data `${json}`');
  }
}

@JsonSerializable(createToJson: false)
class Identifier {
  @JsonKey(name: 'Identifier')
  final String value;

  const Identifier({required this.value});

  factory Identifier.fromJson(Map<String, dynamic> json) => _$IdentifierFromJson(json);
}

@JsonSerializable(createToJson: false)
class Values {
  @JsonKey(name: 'Values')
  final List<ItemType> values;

  const Values({required this.values});

  factory Values.fromJson(Map<String, dynamic> json) => _$ValuesFromJson(json);
}

@JsonSerializable(createToJson: false)
class NatInt {
  @JsonKey(name: 'NatInt')
  final int value;

  const NatInt({required this.value});

  factory NatInt.fromJson(Map<String, dynamic> json) => _$NatIntFromJson(json);
}

@JsonSerializable(createToJson: false)
class NatFloat {
  @JsonKey(name: 'NatFloat')
  final double value;

  const NatFloat({required this.value});

  factory NatFloat.fromJson(Map<String, dynamic> json) =>
      _$NatFloatFromJson(json);
}

@JsonSerializable(createToJson: false)
class NatString {
  @JsonKey(name: 'NatString')
  final String value;

  const NatString({required this.value});

  factory NatString.fromJson(Map<String, dynamic> json) =>
      _$NatStringFromJson(json);
}

@JsonSerializable(createToJson: false)
class NatBool {
  @JsonKey(name: 'NatBool')
  final bool value;

  const NatBool({required this.value});

  factory NatBool.fromJson(Map<String, dynamic> json) =>
      _$NatBoolFromJson(json);
}

@JsonSerializable(createToJson: false)
class LayerTypeName {
  @JsonKey(name: 'LayerTypeName')
  final String layerTypeName;

  const LayerTypeName({required this.layerTypeName});

  factory LayerTypeName.fromJson(Map<String, dynamic> json) =>
      _$LayerTypeNameFromJson(json);
}

@JsonSerializable(createToJson: false)
class ClientTypeName {
  @JsonKey(name: 'ClientTypeName')
  final String clientTypeName;

  const ClientTypeName({required this.clientTypeName});

  factory ClientTypeName.fromJson(Map<String, dynamic> json) =>
      _$ClientTypeNameFromJson(json);
}

@JsonSerializable(createToJson: false)
class ServerTypeName {
  @JsonKey(name: 'ServerTypeName')
  final String serverTypeName;

  const ServerTypeName({required this.serverTypeName});

  factory ServerTypeName.fromJson(Map<String, dynamic> json) =>
      _$ServerTypeNameFromJson(json);
}

class IdsNode<T> {
  final T node;

  const IdsNode({required this.node});

  factory IdsNode.fromJson(Map<String, dynamic> json) {
    if (json.containsKey('Package')) {
      return IdsNode(node: Package.fromJson(json['Package']) as T);
    }

    if (json.containsKey('Layer')) {
      return IdsNode(node: Layer.fromJson(json['Layer']) as T);
    }

    if (json.containsKey('Server')) {
      return IdsNode(node: Server.fromJson(json['Server']) as T);
    }

    if (json.containsKey('Client')) {
      return IdsNode(node: Client.fromJson(json['Client']) as T);
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data `${json}`');
  }
}

@JsonSerializable(createToJson: false)
class Package {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'nodes')
  final List<PackageNode> nodes;

  const Package({required this.ident, required this.nodes});

  factory Package.fromJson(Map<String, dynamic> json) =>
      _$PackageFromJson(json);
}

class PackageNode<T> {
  final T node;

  const PackageNode({required this.node});

  factory PackageNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('PackageField')) {
        return PackageNode(
            node: PackageField.fromJson(json['PackageField']) as T);
      }

      if (json.containsKey('Comment')) {
        return PackageNode(node: Comment.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data `${json}`');
  }
}

@JsonSerializable(createToJson: false)
class PackageField {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'value')
  final ItemType value;

  const PackageField({
    required this.ident,
    required this.value,
  });

  factory PackageField.fromJson(Map<String, dynamic> json) =>
      _$PackageFieldFromJson(json);
}

@JsonSerializable(createToJson: false)
class Layer {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'nodes')
  final List<LayerNode> nodes;

  const Layer({required this.ident, required this.nodes});

  factory Layer.fromJson(Map<String, dynamic> json) => _$LayerFromJson(json);
}

class LayerNode<T> {
  final T node;

  const LayerNode({required this.node});

  factory LayerNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('LayerField')) {
        return LayerNode(node: LayerField.fromJson(json['LayerField']) as T);
      }

      if (json.containsKey('Comment')) {
        return LayerNode(node: Comment.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data `${json}`');
  }
}

@JsonSerializable(createToJson: false)
class LayerField {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'value')
  final ItemType value;

  const LayerField({
    required this.ident,
    required this.value,
  });

  factory LayerField.fromJson(Map<String, dynamic> json) =>
      _$LayerFieldFromJson(json);
}

@JsonSerializable(createToJson: false)
class Server {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'nodes')
  final List<ServerNode> nodes;

  const Server({required this.ident, required this.nodes});

  factory Server.fromJson(Map<String, dynamic> json) => _$ServerFromJson(json);
}

class ServerNode<T> {
  final T node;

  const ServerNode({required this.node});

  factory ServerNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('ServerField')) {
        return ServerNode(node: ServerField.fromJson(json['ServerField']) as T);
      }

      if (json.containsKey('Comment')) {
        return ServerNode(node: Comment.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data `${json}`');
  }
}

@JsonSerializable(createToJson: false)
class ServerField {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'value')
  final ItemType value;

  const ServerField({
    required this.ident,
    required this.value,
  });

  factory ServerField.fromJson(Map<String, dynamic> json) =>
      _$ServerFieldFromJson(json);
}

@JsonSerializable(createToJson: false)
class Client {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'nodes')
  final List<ClientNode> nodes;

  const Client({required this.ident, required this.nodes});

  factory Client.fromJson(Map<String, dynamic> json) => _$ClientFromJson(json);
}

class ClientNode<T> {
  final T node;

  const ClientNode({required this.node});

  factory ClientNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('ClientField')) {
        return ClientNode(node: ClientField.fromJson(json['ClientField']) as T);
      }

      if (json.containsKey('Comment')) {
        return ClientNode(node: Comment.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data `${json}`');
  }
}

@JsonSerializable(createToJson: false)
class ClientField {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'value')
  final ItemType value;

  const ClientField({
    required this.ident,
    required this.value,
  });

  factory ClientField.fromJson(Map<String, dynamic> json) =>
      _$ClientFieldFromJson(json);
}

@JsonSerializable(createToJson: false)
class Comment {
  @JsonKey(name: 'Comment')
  final List<String> comment;

  const Comment({required this.comment});

  factory Comment.fromJson(Map<String, dynamic> json) =>
      _$CommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class PackageComment {
  @JsonKey(name: 'PackageComment')
  final List<String> interfaceComment;

  const PackageComment({required this.interfaceComment});

  factory PackageComment.fromJson(Map<String, dynamic> json) =>
      _$PackageCommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class LayerComment {
  @JsonKey(name: 'LayerComment')
  final List<String> structComment;

  const LayerComment({required this.structComment});

  factory LayerComment.fromJson(Map<String, dynamic> json) =>
      _$LayerCommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class ClientComment {
  @JsonKey(name: 'ClientComment')
  final List<String> enumComment;

  const ClientComment({required this.enumComment});

  factory ClientComment.fromJson(Map<String, dynamic> json) =>
      _$ClientCommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class ServerComment {
  @JsonKey(name: 'ServerComment')
  final List<String> typeListComment;

  const ServerComment({required this.typeListComment});

  factory ServerComment.fromJson(Map<String, dynamic> json) =>
      _$ServerCommentFromJson(json);
}
