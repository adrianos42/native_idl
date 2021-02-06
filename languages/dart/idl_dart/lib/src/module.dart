import 'dart:io';
import 'package:idl_dart/native_idl.dart';
import 'package:path/path.dart';
import 'json/idl_json.dart';
import 'json/idl_types_json.dart' as idl_types;
import 'json/ids_types_json.dart' as ids_types;
import 'dart_client.dart';

class Module {
  idl_types.TypeConst findTypeConst(idl_types.ConstTypeName name) =>
      _request.idlNodes
          .map((f) => f.node)
          .whereType<idl_types.TypeConst>()
          .firstWhere((node) => node.ident == name.constTypeName);

  idl_types.TypeStruct findTypeStruct(idl_types.StructTypeName name) =>
      _request.idlNodes
          .map((f) => f.node)
          .whereType<idl_types.TypeStruct>()
          .firstWhere((node) => node.ident == name.structTypeName);

  idl_types.TypeInterface findTypeInterface(idl_types.InterfaceTypeName name) =>
      _request.idlNodes
          .map((f) => f.node)
          .whereType<idl_types.TypeInterface>()
          .firstWhere((node) => node.ident == name.interfaceTypeName);

  idl_types.TypeEnum findTypeEnum(idl_types.EnumTypeName name) =>
      _request.idlNodes
          .map((f) => f.node)
          .whereType<idl_types.TypeEnum>()
          .firstWhere((node) => node.ident == name.enumTypeName);

  idl_types.TypeList findTypeList(idl_types.ListTypeName name) =>
      _request.idlNodes
          .map((f) => f.node)
          .whereType<idl_types.TypeList>()
          .firstWhere((node) => node.ident == name.listTypeName);

  ids_types.Layer findLayer(ids_types.LayerTypeName name) => _request.idsNodes
      .map((f) => f.node)
      .whereType<ids_types.Layer>()
      .firstWhere((node) => node.ident == name.layerTypeName);

  ids_types.Server findServer(ids_types.ServerTypeName name) =>
      _request.idsNodes
          .map((f) => f.node)
          .whereType<ids_types.Server>()
          .firstWhere((node) => node.ident == name.serverTypeName);

  ids_types.Client findClient(ids_types.ClientTypeName name) =>
      _request.idsNodes
          .map((f) => f.node)
          .whereType<ids_types.Client>()
          .firstWhere((node) => node.ident == name.clientTypeName);

  static bool interfaceHasStaticField(idl_types.TypeInterface tyInterface) {
    return tyInterface.fields
        .map((m) => m.node)
        .whereType<idl_types.InterfaceField>()
        .any((field) => field.isStatic);
  }

  static bool interfaceHasNonStaticField(idl_types.TypeInterface tyInterface) {
    return tyInterface.fields
        .map((m) => m.node)
        .whereType<idl_types.InterfaceField>()
        .any((field) => !field.isStatic);
  }

  static bool interfaceReturnsStream(idl_types.TypeInterface tyInterface,
      {bool? isStatic}) {
    return tyInterface.fields
        .map((m) => m.node)
        .where((node) =>
            node is idl_types.InterfaceField &&
            (isStatic == null || node.isStatic == isStatic))
        .any((field) => fieldReturnsStream(field));
  }

  static bool interfaceSendsStream(idl_types.TypeInterface tyInterface, // TODO
      {bool? isStatic}) {
    return tyInterface.fields
        .map((m) => m.node)
        .where((node) =>
            node is idl_types.InterfaceField &&
            (isStatic == null || node.isStatic == isStatic))
        .any((field) => fieldSendsStream(field));
  }

  static bool fieldSendsStream(idl_types.InterfaceField field) {
    final ty = field.ty.typeName;
    return ty is idl_types.TypeFunction &&
            (ty.args.typeName as idl_types.TypeTuple).fields.last.ty.typeName
                is idl_types.TypeStream ||
        ty is idl_types.TypeTuple &&
            ty.fields.last.ty.typeName is idl_types.TypeStream;
  }

  static bool fieldReturnsStream(idl_types.InterfaceField field) {
    final ty = field.ty.typeName;
    return ty is idl_types.TypeStream ||
        ty is idl_types.TypeFunction &&
            ty.returnTy.typeName is idl_types.TypeStream;
  }

  static bool interfaceReturnsValue(idl_types.TypeInterface tyInterface,
      {bool? isStatic}) {
    return tyInterface.fields
        .map((m) => m.node)
        .where((node) =>
            node is idl_types.InterfaceField &&
            (isStatic == null || node.isStatic == isStatic))
        .any((field) => fieldReturnsValue(field));
  }

  static bool fieldReturnsValue(idl_types.InterfaceField field) {
    return !fieldReturnsStream(field);
  }

  static idl_types.TypeName fieldStreamReturnTy(
      idl_types.InterfaceField field) {
    final streamTy = field.ty.typeName is idl_types.TypeFunction
        ? field.ty.typeName.returnTy
        : field.ty;
    if (streamTy.typeName is idl_types.TypeStream) {
      return streamTy.typeName.sTy;
    }
    throw ArgumentError('type `${streamTy.typeName}` does not return a stream');
  }

  static idl_types.TypeName fieldStreamSendTy(idl_types.InterfaceField field) {
    final streamTy = field.ty.typeName is idl_types.TypeFunction
        ? (field.ty.typeName.args.typeName as idl_types.TypeTuple)
            .fields
            .last
            .ty
        : (field.ty.typeName as idl_types.TypeTuple).fields.last.ty;
    if (streamTy.typeName is idl_types.TypeStream) {
      return streamTy.typeName.sTy;
    }
    throw ArgumentError(
        'type `${streamTy.typeName}` does not have stream argument');
  }

  String get libraryName => _request.idlNodes
      .singleWhere((n) => n.node is idl_types.LibraryName)
      .node
      .libraryName;

  bool isTypePrimitive(idl_types.TypeName type) {
    final ty = type.typeName;
    return ty is idl_types.Types
        ? ty == idl_types.Types.natBool ||
            ty == idl_types.Types.natInt ||
            ty == idl_types.Types.natFloat ||
            ty == idl_types.Types.natNone
        : ty is idl_types.ConstTypeName
            ? findTypeConst(ty).constType == idl_types.ConstTypes.natInt ||
                findTypeConst(ty).constType == idl_types.ConstTypes.natFloat
            : ty is idl_types.EnumTypeName;
  }

  bool get hasTests => false; // TODO Find way to specify tests

  // Returns a `Client` field with the expected `ItemType`
  static T getClientField<T>(ids_types.Client client, String fieldName) =>
      client.nodes
          .map((f) => f.node)
          .whereType<ids_types.ClientField>()
          .firstWhere((field) => field.ident == fieldName)
          .value
          .itemType as T;

  // Returns a `Server` field with the expected `ItemType`
  static T getServerField<T>(ids_types.Server server, String fieldName) =>
      server.nodes
          .map((f) => f.node)
          .whereType<ids_types.ServerField>()
          .firstWhere((field) => field.ident == fieldName)
          .value
          .itemType as T;

  // Returns a `Layer` field with the expected `ItemType`
  static T getLayerField<T>(ids_types.Layer layer, String fieldName) =>
      layer.nodes
          .map((f) => f.node)
          .whereType<ids_types.LayerField>()
          .firstWhere((field) => field.ident == fieldName)
          .value
          .itemType as T;

  Module(LanguageRequest request) : _request = request {
    final requestType = request.requestType.value;

    if (requestType is Client) {
      _targetClient = findClient(
          ids_types.ClientTypeName(clientTypeName: requestType.client));

      // TODO Resolve for which server to use
      _targetServer = findServer(
          Module.getClientField<ids_types.Values>(_targetClient!, 'servers')
              .values
              .first
              .itemType as ids_types.ServerTypeName);
    } else {}
  }

  final LanguageRequest _request;
  ids_types.Client? _targetClient;
  late ids_types.Server _targetServer;

  ids_types.Client get targetClient => _targetClient!;

  // This is both the target server used by the client, and the target for the
  // eventual server code generation, when the request is the server type.
  ids_types.Server get targetServer => _targetServer;

  LanguageRequest get request => _request;

  void addResponseMessage(String value) => _responseMessages.add(value);

  final _responseMessages = <String>[];

  LanguageResponse generateResponse() {
    final requestType = request.requestType.value;
    if (requestType is Client) {
      final listItems = DartClient.generate(this);
      return LanguageResponse(
          responseMessages: _responseMessages,
          genResponse: ResponseType(
              value: Generated(generated: listItems)));
    } else if (requestType is Server) {
      throw ArgumentError('Server file generation is not supported');
    }

    throw Exception();
  }
}

StorageItem createSourceItem(String name, String txt) =>
    StorageItem(value: SourceItem(source: Source(name: name, txt: txt)));

StorageItem createFolderItem(String name, List<StorageItem> items) =>
    StorageItem(value: FolderItem(folder: Folder(name: name, items: items)));

StorageItem createFolderItemFromPath(Directory path) {
  final folderList = path.listSync();
  final result = <StorageItem>[];

  for (var item in folderList) {
    if (FileSystemEntity.isDirectorySync(item.path)) {
      result.add(createFolderItemFromPath(Directory.fromUri(item.uri)));
    } else if (FileSystemEntity.isFileSync(item.path)) {
      result.add(createSourceItemFromPath(File.fromUri(item.uri)));
    }
  }
  return createFolderItem(basename(path.path), result);
}

StorageItem createSourceItemFromPath(File path) {
  return createSourceItem(basename(path.path), path.readAsStringSync());
}
