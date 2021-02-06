import 'dart:convert';

import 'package:json_annotation/json_annotation.dart';

part 'idl_types_json.g.dart';

enum Types {
  natInt,
  natFloat,
  natString,
  natBytes,
  natBool,
  natNone,
}

Types _typesFromJson(String json) {
  switch (json) {
    case 'NatInt':
      return Types.natInt;
    case 'NatFloat':
      return Types.natFloat;
    case 'NatString':
      return Types.natString;
    case 'NatBytes':
      return Types.natBytes;
    case 'NatBool':
      return Types.natBool;
    case 'NatNone':
      return Types.natNone;
  }

  throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
}

class IdlNode<T> {
  final T node;

  const IdlNode({required this.node});

  factory IdlNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('LibraryName')) {
        return IdlNode(node: LibraryName.fromJson(json) as T);
      }

      if (json.containsKey('Imports')) {
        return IdlNode(node: Imports.fromJson(json) as T);
      }

      if (json.containsKey('Comment')) {
        return IdlNode(node: Comment.fromJson(json) as T);
      }

      if (json.containsKey('InterfaceComment')) {
        return IdlNode(node: InterfaceComment.fromJson(json) as T);
      }

      if (json.containsKey('StructComment')) {
        return IdlNode(node: StructComment.fromJson(json) as T);
      }

      if (json.containsKey('EnumCommment')) {
        return IdlNode(node: EnumComment.fromJson(json) as T);
      }

      if (json.containsKey('ConstComment')) {
        return IdlNode(node: ConstComment.fromJson(json) as T);
      }

      if (json.containsKey('TypeListComment')) {
        return IdlNode(node: TypeListComment.fromJson(json) as T);
      }

      if (json.containsKey('TypeStruct')) {
        return IdlNode(node: TypeStruct.fromJson(json['TypeStruct']) as T);
      }

      if (json.containsKey('TypeEnum')) {
        return IdlNode(node: TypeEnum.fromJson(json['TypeEnum']) as T);
      }

      if (json.containsKey('TypeList')) {
        return IdlNode(node: TypeList.fromJson(json['TypeList']) as T);
      }

      if (json.containsKey('TypeConst')) {
        return IdlNode(node: TypeConst.fromJson(json['TypeConst']) as T);
      }

      if (json.containsKey('TypeInterface')) {
        return IdlNode(
            node: TypeInterface.fromJson(json['TypeInterface']) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
  }
}

class InterfaceNode<T> {
  final T node;

  const InterfaceNode({required this.node});

  factory InterfaceNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('InterfaceField')) {
        return InterfaceNode(
            node: InterfaceField.fromJson(json['InterfaceField']) as T);
      }

      if (json.containsKey('Comment')) {
        return InterfaceNode(node: Comment.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
  }
}

class StructNode<T> {
  final T node;

  const StructNode({required this.node});

  factory StructNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('StructField')) {
        return StructNode(node: StructField.fromJson(json['StructField']) as T);
      }

      if (json.containsKey('Comment')) {
        return StructNode(node: Comment.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
  }
}

class TypeListNode<T> {
  final T node;

  const TypeListNode({required this.node});

  factory TypeListNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('TypeListField')) {
        return TypeListNode(
            node: TypeListField.fromJson(json['TypeListField']) as T);
      }

      if (json.containsKey('Comment')) {
        return TypeListNode(node: Comment.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
  }
}

class EnumNode<T> {
  final T node;

  const EnumNode({required this.node});

  factory EnumNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('EnumField')) {
        return EnumNode(node: EnumField.fromJson(json['EnumField']) as T);
      }

      if (json.containsKey('Comment')) {
        return EnumNode(node: Comment.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
  }
}

class ConstNode<T> {
  final T node;

  const ConstNode({required this.node});

  factory ConstNode.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('ConstField')) {
        return ConstNode(node: ConstField.fromJson(json['ConstField']) as T);
      }

      if (json.containsKey('Comment')) {
        return ConstNode(node: Comment.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
  }
}

class TypeName<T> {
  final T typeName;

  const TypeName({required this.typeName});

  factory TypeName.fromJson(Map<String, dynamic> json) {
    if (json is Map<String, dynamic>) {
      if (json.containsKey('Types')) {
        return TypeName(typeName: _typesFromJson(json['Types']) as T);
      }

      if (json.containsKey('TypeFunction')) {
        return TypeName(
            typeName: TypeFunction.fromJson(json['TypeFunction']) as T);
      }

      if (json.containsKey('TypeTuple')) {
        return TypeName(typeName: TypeTuple.fromJson(json['TypeTuple']) as T);
      }

      if (json.containsKey('TypeArray')) {
        return TypeName(typeName: TypeArray.fromJson(json['TypeArray']) as T);
      }

      if (json.containsKey('TypeMap')) {
        return TypeName(typeName: TypeMap.fromJson(json['TypeMap']) as T);
      }

      if (json.containsKey('TypePair')) {
        return TypeName(typeName: TypePair.fromJson(json['TypePair']) as T);
      }

      if (json.containsKey('TypeOption')) {
        return TypeName(typeName: TypeOption.fromJson(json['TypeOption']) as T);
      }

      if (json.containsKey('TypeResult')) {
        return TypeName(typeName: TypeResult.fromJson(json['TypeResult']) as T);
      }

      if (json.containsKey('TypeStream')) {
        return TypeName(typeName: TypeStream.fromJson(json['TypeStream']) as T);
      }

      if (json.containsKey('ListTypeName')) {
        return TypeName(typeName: ListTypeName.fromJson(json) as T);
      }

      if (json.containsKey('EnumTypeName')) {
        return TypeName(typeName: EnumTypeName.fromJson(json) as T);
      }

      if (json.containsKey('StructTypeName')) {
        return TypeName(typeName: StructTypeName.fromJson(json) as T);
      }

      if (json.containsKey('InterfaceTypeName')) {
        return TypeName(typeName: InterfaceTypeName.fromJson(json) as T);
      }

      if (json.containsKey('ConstTypeName')) {
        return TypeName(typeName: ConstTypeName.fromJson(json) as T);
      }
    }

    throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
  }
}

@JsonSerializable(createToJson: false)
class ListTypeName {
  @JsonKey(name: 'ListTypeName')
  final String listTypeName;

  const ListTypeName({
    required this.listTypeName,
  });

  factory ListTypeName.fromJson(Map<String, dynamic> json) =>
      _$ListTypeNameFromJson(json);
}

@JsonSerializable(createToJson: false)
class EnumTypeName {
  @JsonKey(name: 'EnumTypeName')
  final String enumTypeName;

  const EnumTypeName({
    required this.enumTypeName,
  });

  factory EnumTypeName.fromJson(Map<String, dynamic> json) =>
      _$EnumTypeNameFromJson(json);
}

@JsonSerializable(createToJson: false)
class StructTypeName {
  @JsonKey(name: 'StructTypeName')
  final String structTypeName;

  const StructTypeName({
    required this.structTypeName,
  });

  factory StructTypeName.fromJson(Map<String, dynamic> json) =>
      _$StructTypeNameFromJson(json);
}

@JsonSerializable(createToJson: false)
class InterfaceTypeName {
  @JsonKey(name: 'InterfaceTypeName')
  final String interfaceTypeName;

  const InterfaceTypeName({
    required this.interfaceTypeName,
  });

  factory InterfaceTypeName.fromJson(Map<String, dynamic> json) =>
      _$InterfaceTypeNameFromJson(json);
}

@JsonSerializable(createToJson: false)
class ConstTypeName {
  @JsonKey(name: 'ConstTypeName')
  final String constTypeName;

  const ConstTypeName({
    required this.constTypeName,
  });

  factory ConstTypeName.fromJson(Map<String, dynamic> json) =>
      _$ConstTypeNameFromJson(json);
}

@JsonSerializable(createToJson: false)
class LibraryName {
  @JsonKey(name: 'LibraryName')
  final String libraryName;

  const LibraryName({
    required this.libraryName,
  });

  factory LibraryName.fromJson(Map<String, dynamic> json) =>
      _$LibraryNameFromJson(json);
}

@JsonSerializable(createToJson: false)
class Imports {
  @JsonKey(name: 'Imports')
  final List<String> imports;

  const Imports({
    required this.imports,
  });

  factory Imports.fromJson(Map<String, dynamic> json) =>
      _$ImportsFromJson(json);
}

@JsonSerializable(createToJson: false)
class Comment {
  @JsonKey(name: 'Comment')
  final List<String> comment;

  const Comment({
    required this.comment,
  });

  factory Comment.fromJson(Map<String, dynamic> json) =>
      _$CommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class InterfaceComment {
  @JsonKey(name: 'InterfaceComment')
  final List<String> interfaceComment;

  const InterfaceComment({
    required this.interfaceComment,
  });

  factory InterfaceComment.fromJson(Map<String, dynamic> json) =>
      _$InterfaceCommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class StructComment {
  @JsonKey(name: 'StructComment')
  final List<String> structComment;

  const StructComment({
    required this.structComment,
  });

  factory StructComment.fromJson(Map<String, dynamic> json) =>
      _$StructCommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class EnumComment {
  @JsonKey(name: 'EnumComment')
  final List<String> enumComment;

  const EnumComment({
    required this.enumComment,
  });

  factory EnumComment.fromJson(Map<String, dynamic> json) =>
      _$EnumCommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class ConstComment {
  @JsonKey(name: 'ConstComment')
  final List<String> constComment;

  const ConstComment({
    required this.constComment,
  });

  factory ConstComment.fromJson(Map<String, dynamic> json) =>
      _$ConstCommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeListComment {
  @JsonKey(name: 'TypeListComment')
  final List<String> typeListComment;

  const TypeListComment({
    required this.typeListComment,
  });

  factory TypeListComment.fromJson(Map<String, dynamic> json) =>
      _$TypeListCommentFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeInterface {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'fields')
  final List<InterfaceNode> fields;

  const TypeInterface({
    required this.ident,
    required this.fields,
  });

  factory TypeInterface.fromJson(Map<String, dynamic> json) =>
      _$TypeInterfaceFromJson(json);
}

@JsonSerializable(createToJson: false)
class InterfaceField {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'is_static')
  final bool isStatic;

  @JsonKey(name: 'ty')
  final TypeName ty;

  const InterfaceField({
    required this.ident,
    required this.isStatic,
    required this.ty,
  });

  factory InterfaceField.fromJson(Map<String, dynamic> json) =>
      _$InterfaceFieldFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeStruct {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'fields')
  final List<StructNode> fields;

  const TypeStruct({
    required this.ident,
    required this.fields,
  });

  factory TypeStruct.fromJson(Map<String, dynamic> json) =>
      _$TypeStructFromJson(json);
}

@JsonSerializable(createToJson: false)
class StructField {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'ty')
  final TypeName ty;

  const StructField({
    required this.ident,
    required this.ty,
  });

  factory StructField.fromJson(Map<String, dynamic> json) =>
      _$StructFieldFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeList {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'fields')
  final List<TypeListNode> fields;

  const TypeList({
    required this.ident,
    required this.fields,
  });

  factory TypeList.fromJson(Map<String, dynamic> json) =>
      _$TypeListFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeListField {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'ty')
  final TypeName ty;

  const TypeListField({
    required this.ident,
    required this.ty,
  });

  factory TypeListField.fromJson(Map<String, dynamic> json) =>
      _$TypeListFieldFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeEnum {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'fields')
  final List<EnumNode> fields;

  const TypeEnum({
    required this.ident,
    required this.fields,
  });

  factory TypeEnum.fromJson(Map<String, dynamic> json) =>
      _$TypeEnumFromJson(json);
}

@JsonSerializable(createToJson: false)
class EnumField {
  @JsonKey(name: 'ident')
  final String ident;

  const EnumField({
    required this.ident,
  });

  factory EnumField.fromJson(Map<String, dynamic> json) =>
      _$EnumFieldFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeConst {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'fields')
  final List<ConstNode> fields;

  @JsonKey(name: 'const_type', fromJson: _constTypeFromJson)
  final ConstTypes constType;

  const TypeConst({
    required this.ident,
    required this.fields,
    required this.constType,
  });

  factory TypeConst.fromJson(Map<String, dynamic> json) =>
      _$TypeConstFromJson(json);
}

enum ConstTypes {
  natInt,
  natFloat,
  natString,
}

ConstTypes _constTypeFromJson(String json) {
  switch (json) {
    case 'NatInt':
      return ConstTypes.natInt;
    case 'NatFloat':
      return ConstTypes.natFloat;
    case 'NatString':
      return ConstTypes.natString;
  }

  throw ArgumentError.value(json, 'json', 'Invalid convertion input data.');
}

@JsonSerializable(createToJson: false)
class ConstField {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'value')
  final String value;

  const ConstField({
    required this.ident,
    required this.value,
  });

  factory ConstField.fromJson(Map<String, dynamic> json) =>
      _$ConstFieldFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeFunction {
  @JsonKey(name: 'args')
  final TypeName args;

  @JsonKey(name: 'return_ty')
  final TypeName returnTy;

  const TypeFunction({
    required this.args,
    required this.returnTy,
  });

  factory TypeFunction.fromJson(Map<String, dynamic> json) =>
      _$TypeFunctionFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeArray {
  @JsonKey(name: 'ty')
  final TypeName ty;

  const TypeArray({
    required this.ty,
  });

  factory TypeArray.fromJson(Map<String, dynamic> json) =>
      _$TypeArrayFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeMap {
  @JsonKey(name: 'map_ty')
  final TypeName mapTy;

  @JsonKey(name: 'index_ty')
  final TypeName indexTy;

  const TypeMap({
    required this.mapTy,
    required this.indexTy,
  });

  factory TypeMap.fromJson(Map<String, dynamic> json) =>
      _$TypeMapFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypePair {
  @JsonKey(name: 'first_ty')
  final TypeName firstTy;

  @JsonKey(name: 'second_ty')
  final TypeName secondTy;

  const TypePair({
    required this.firstTy,
    required this.secondTy,
  });

  factory TypePair.fromJson(Map<String, dynamic> json) =>
      _$TypePairFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeStream {
  @JsonKey(name: 's_ty')
  final TypeName sTy;

  const TypeStream({
    required this.sTy,
  });

  factory TypeStream.fromJson(Map<String, dynamic> json) =>
      _$TypeStreamFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeTuple {
  @JsonKey(name: 'fields')
  final List<TupleEntry> fields;

  const TypeTuple({
    required this.fields,
  });

  factory TypeTuple.fromJson(Map<String, dynamic> json) =>
      _$TypeTupleFromJson(json);
}

@JsonSerializable(createToJson: false)
class TupleEntry {
  @JsonKey(name: 'ident')
  final String ident;

  @JsonKey(name: 'ty')
  final TypeName ty;

  const TupleEntry({
    required this.ident,
    required this.ty,
  });

  factory TupleEntry.fromJson(Map<String, dynamic> json) =>
      _$TupleEntryFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeResult {
  @JsonKey(name: 'ok_ty')
  final TypeName okTy;

  @JsonKey(name: 'err_ty')
  final TypeName errTy;

  const TypeResult({
    required this.okTy,
    required this.errTy,
  });

  factory TypeResult.fromJson(Map<String, dynamic> json) =>
      _$TypeResultFromJson(json);
}

@JsonSerializable(createToJson: false)
class TypeOption {
  @JsonKey(name: 'some_ty')
  final TypeName someTy;

  const TypeOption({
    required this.someTy,
  });

  factory TypeOption.fromJson(Map<String, dynamic> json) =>
      _$TypeOptionFromJson(json);
}
