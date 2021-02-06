// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'ids_types_json.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Values _$ValuesFromJson(Map<String, dynamic> json) {
  return Values(
    values: (json['Values'] as List<dynamic>)
        .map((e) => ItemType.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

NatInt _$NatIntFromJson(Map<String, dynamic> json) {
  return NatInt(
    value: json['NatInt'] as int,
  );
}

NatFloat _$NatFloatFromJson(Map<String, dynamic> json) {
  return NatFloat(
    value: (json['NatFloat'] as num).toDouble(),
  );
}

NatString _$NatStringFromJson(Map<String, dynamic> json) {
  return NatString(
    value: json['NatString'] as String,
  );
}

NatBool _$NatBoolFromJson(Map<String, dynamic> json) {
  return NatBool(
    value: json['NatBool'] as bool,
  );
}

LayerTypeName _$LayerTypeNameFromJson(Map<String, dynamic> json) {
  return LayerTypeName(
    layerTypeName: json['LayerTypeName'] as String,
  );
}

ClientTypeName _$ClientTypeNameFromJson(Map<String, dynamic> json) {
  return ClientTypeName(
    clientTypeName: json['ClientTypeName'] as String,
  );
}

ServerTypeName _$ServerTypeNameFromJson(Map<String, dynamic> json) {
  return ServerTypeName(
    serverTypeName: json['ServerTypeName'] as String,
  );
}

Library _$LibraryFromJson(Map<String, dynamic> json) {
  return Library(
    ident: json['ident'] as String,
    nodes: (json['nodes'] as List<dynamic>)
        .map((e) => LibraryNode.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

LibraryField _$LibraryFieldFromJson(Map<String, dynamic> json) {
  return LibraryField(
    ident: json['ident'] as String,
    value: ItemType.fromJson(json['value'] as Map<String, dynamic>),
  );
}

Layer _$LayerFromJson(Map<String, dynamic> json) {
  return Layer(
    ident: json['ident'] as String,
    nodes: (json['nodes'] as List<dynamic>)
        .map((e) => LayerNode.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

LayerField _$LayerFieldFromJson(Map<String, dynamic> json) {
  return LayerField(
    ident: json['ident'] as String,
    value: ItemType.fromJson(json['value'] as Map<String, dynamic>),
  );
}

Server _$ServerFromJson(Map<String, dynamic> json) {
  return Server(
    ident: json['ident'] as String,
    nodes: (json['nodes'] as List<dynamic>)
        .map((e) => ServerNode.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

ServerField _$ServerFieldFromJson(Map<String, dynamic> json) {
  return ServerField(
    ident: json['ident'] as String,
    value: ItemType.fromJson(json['value'] as Map<String, dynamic>),
  );
}

Client _$ClientFromJson(Map<String, dynamic> json) {
  return Client(
    ident: json['ident'] as String,
    nodes: (json['nodes'] as List<dynamic>)
        .map((e) => ClientNode.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

ClientField _$ClientFieldFromJson(Map<String, dynamic> json) {
  return ClientField(
    ident: json['ident'] as String,
    value: ItemType.fromJson(json['value'] as Map<String, dynamic>),
  );
}

Comment _$CommentFromJson(Map<String, dynamic> json) {
  return Comment(
    comment:
        (json['Comment'] as List<dynamic>).map((e) => e as String).toList(),
  );
}

LibraryComment _$LibraryCommentFromJson(Map<String, dynamic> json) {
  return LibraryComment(
    interfaceComment: (json['LibraryComment'] as List<dynamic>)
        .map((e) => e as String)
        .toList(),
  );
}

LayerComment _$LayerCommentFromJson(Map<String, dynamic> json) {
  return LayerComment(
    structComment: (json['LayerComment'] as List<dynamic>)
        .map((e) => e as String)
        .toList(),
  );
}

ClientComment _$ClientCommentFromJson(Map<String, dynamic> json) {
  return ClientComment(
    enumComment: (json['ClientComment'] as List<dynamic>)
        .map((e) => e as String)
        .toList(),
  );
}

ServerComment _$ServerCommentFromJson(Map<String, dynamic> json) {
  return ServerComment(
    typeListComment: (json['ServerComment'] as List<dynamic>)
        .map((e) => e as String)
        .toList(),
  );
}
