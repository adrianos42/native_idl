import 'dart:convert';
import 'dart:typed_data';

import 'package:json_annotation/json_annotation.dart';

import 'idl_types_json.dart' as idl_types;
import 'ids_types_json.dart' as ids_types;

part 'idl_json.g.dart';

@JsonSerializable(createFactory: false)
class SourceItem {
  @JsonKey(name: 'Source')
  final Source source;

  const SourceItem({required this.source});

  Map<String, dynamic> toJson() => _$SourceItemToJson(this);
}

@JsonSerializable(createFactory: false)
class BinarySourceItem {
  @JsonKey(name: 'Source')
  final BinarySource binarySource;

  const BinarySourceItem({required this.binarySource});

  Map<String, dynamic> toJson() => _$BinarySourceItemToJson(this);
}

@JsonSerializable(createFactory: false)
class FolderItem {
  @JsonKey(name: 'Folder')
  final Folder folder;

  const FolderItem({required this.folder});

  Map<String, dynamic> toJson() => _$FolderItemToJson(this);
}

class StorageItem<T> {
  final T value;

  const StorageItem({required this.value});

  Map<String, dynamic> toJson() => (value as dynamic).toJson();
}

@JsonSerializable(createFactory: false)
class Source {
  @JsonKey(name: 'name')
  final String name;

  @JsonKey(name: 'txt')
  final String txt;

  const Source({required this.name, required this.txt});

  Map<String, dynamic> toJson() => _$SourceToJson(this);
}

@JsonSerializable(createFactory: false)
class BinarySource {
  @JsonKey(name: 'name')
  final Uint8List name;

  @JsonKey(name: 'data')
  final Uint8List data;

  const BinarySource({required this.name, required this.data});

  Map<String, dynamic> toJson() => _$BinarySourceToJson(this);
}

@JsonSerializable(createFactory: false)
class Folder {
  @JsonKey(name: 'name')
  final String name;

  @JsonKey(name: 'items')
  final List<StorageItem> items;

  const Folder({required this.name, required this.items});

  Map<String, dynamic> toJson() => _$FolderToJson(this);
}

@JsonSerializable(createFactory: false)
class Generated {
  @JsonKey(name: 'Generated')
  final List<StorageItem> generated;

  const Generated({required this.generated});

  Map<String, dynamic> toJson() => _$GeneratedToJson(this);
}

@JsonSerializable(createFactory: false)
class Undefined {
  @JsonKey(name: 'Undefined')
  final String undefined;

  const Undefined({required this.undefined});

  Map<String, dynamic> toJson() => _$UndefinedToJson(this);
}

class ResponseType<T> {
  final T value;

  const ResponseType({required this.value});

  Map<String, dynamic> toJson() => (value as dynamic).toJson();
}

@JsonSerializable(createFactory: false)
class LanguageResponse {
  @JsonKey(name: 'gen_response')
  final ResponseType genResponse;

  @JsonKey(name: 'response_messages')
  final List<String> responseMessages;

  const LanguageResponse({
    required this.genResponse,
    required this.responseMessages,
  });

  Map<String, dynamic> toJson() => _$LanguageResponseToJson(this);
}

@JsonSerializable(createToJson: false)
class Server {
  @JsonKey(name: 'Server')
  final String server;

  const Server({required this.server});

  factory Server.fromJson(Map<String, dynamic> json) => _$ServerFromJson(json);
}

@JsonSerializable(createToJson: false)
class Client {
  @JsonKey(name: 'Client')
  final String client;

  const Client({required this.client});

  factory Client.fromJson(Map<String, dynamic> json) => _$ClientFromJson(json);
}

class RequestType<T> {
  final T value;

  const RequestType({required this.value});

  factory RequestType.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('Server')) {
        return RequestType(value: Server.fromJson(json) as T);
      }

      if (json.containsKey('Client')) {
        return RequestType(value: Client.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
  }
}

@JsonSerializable(createToJson: false)
class LanguageRequest {
  @JsonKey(name: 'idl_nodes', fromJson: _idlNodesFromJson)
  final List<idl_types.IdlNode> idlNodes;

  @JsonKey(name: 'ids_nodes', fromJson: _idsNodesFromJson)
  final List<ids_types.IdsNode> idsNodes;

  @JsonKey(name: 'request_type')
  final RequestType requestType;

  const LanguageRequest({
    required this.idlNodes,
    required this.idsNodes,
    required this.requestType,
  });

  factory LanguageRequest.fromJson(Map<String, dynamic> json) =>
      _$LanguageRequestFromJson(json);
}

List<idl_types.IdlNode> _idlNodesFromJson(List<dynamic> json) {
  return json
      .map((value) => idl_types.IdlNode.fromJson(value as Map<String, dynamic>))
      .toList();
}

List<ids_types.IdsNode> _idsNodesFromJson(List<dynamic> json) {
  return json
      .map((value) => ids_types.IdsNode.fromJson(value as Map<String, dynamic>))
      .toList();
}
