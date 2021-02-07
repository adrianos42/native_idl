// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'idl_json.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Map<String, dynamic> _$SourceItemToJson(SourceItem instance) =>
    <String, dynamic>{
      'Source': instance.source,
    };

Map<String, dynamic> _$BinarySourceItemToJson(BinarySourceItem instance) =>
    <String, dynamic>{
      'Source': instance.binarySource,
    };

Map<String, dynamic> _$FolderItemToJson(FolderItem instance) =>
    <String, dynamic>{
      'Folder': instance.folder,
    };

Map<String, dynamic> _$SourceToJson(Source instance) => <String, dynamic>{
      'name': instance.name,
      'txt': instance.txt,
    };

Map<String, dynamic> _$BinarySourceToJson(BinarySource instance) =>
    <String, dynamic>{
      'name': instance.name,
      'data': instance.data,
    };

Map<String, dynamic> _$FolderToJson(Folder instance) => <String, dynamic>{
      'name': instance.name,
      'items': instance.items,
    };

Map<String, dynamic> _$GeneratedToJson(Generated instance) => <String, dynamic>{
      'Generated': instance.generated,
    };

Map<String, dynamic> _$UndefinedToJson(Undefined instance) => <String, dynamic>{
      'Undefined': instance.undefined,
    };

Map<String, dynamic> _$LanguageResponseToJson(LanguageResponse instance) =>
    <String, dynamic>{
      'gen_response': instance.genResponse,
      'response_messages': instance.responseMessages,
    };

Server _$ServerFromJson(Map<String, dynamic> json) {
  return Server(
    server: json['Server'] as String,
  );
}

Client _$ClientFromJson(Map<String, dynamic> json) {
  return Client(
    client: json['Client'] as String,
  );
}

LibraryItem _$LibraryItemFromJson(Map<String, dynamic> json) {
  return LibraryItem(
    nodes: _idlNodesFromJson(json['nodes'] as List),
  );
}

LanguageRequest _$LanguageRequestFromJson(Map<String, dynamic> json) {
  return LanguageRequest(
    libraries: _libraryItemFromJson(json['libraries'] as List),
    idsNodes: _idsNodesFromJson(json['ids_nodes'] as List),
    requestType:
        RequestType.fromJson(json['request_type'] as Map<String, dynamic>),
  );
}
