// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'idl_types_json.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

ListTypeName _$ListTypeNameFromJson(Map<String, dynamic> json) {
  return ListTypeName(
    listTypeName: json['ListTypeName'] as String,
  );
}

EnumTypeName _$EnumTypeNameFromJson(Map<String, dynamic> json) {
  return EnumTypeName(
    enumTypeName: json['EnumTypeName'] as String,
  );
}

StructTypeName _$StructTypeNameFromJson(Map<String, dynamic> json) {
  return StructTypeName(
    structTypeName: json['StructTypeName'] as String,
  );
}

InterfaceTypeName _$InterfaceTypeNameFromJson(Map<String, dynamic> json) {
  return InterfaceTypeName(
    interfaceTypeName: json['InterfaceTypeName'] as String,
  );
}

ConstTypeName _$ConstTypeNameFromJson(Map<String, dynamic> json) {
  return ConstTypeName(
    constTypeName: json['ConstTypeName'] as String,
  );
}

LibraryName _$LibraryNameFromJson(Map<String, dynamic> json) {
  return LibraryName(
    libraryName: json['LibraryName'] as String,
  );
}

Imports _$ImportsFromJson(Map<String, dynamic> json) {
  return Imports(
    imports:
        (json['Imports'] as List<dynamic>).map((e) => e as String).toList(),
  );
}

Comment _$CommentFromJson(Map<String, dynamic> json) {
  return Comment(
    comment:
        (json['Comment'] as List<dynamic>).map((e) => e as String).toList(),
  );
}

InterfaceComment _$InterfaceCommentFromJson(Map<String, dynamic> json) {
  return InterfaceComment(
    interfaceComment: (json['InterfaceComment'] as List<dynamic>)
        .map((e) => e as String)
        .toList(),
  );
}

StructComment _$StructCommentFromJson(Map<String, dynamic> json) {
  return StructComment(
    structComment: (json['StructComment'] as List<dynamic>)
        .map((e) => e as String)
        .toList(),
  );
}

EnumComment _$EnumCommentFromJson(Map<String, dynamic> json) {
  return EnumComment(
    enumComment:
        (json['EnumComment'] as List<dynamic>).map((e) => e as String).toList(),
  );
}

ConstComment _$ConstCommentFromJson(Map<String, dynamic> json) {
  return ConstComment(
    constComment: (json['ConstComment'] as List<dynamic>)
        .map((e) => e as String)
        .toList(),
  );
}

TypeListComment _$TypeListCommentFromJson(Map<String, dynamic> json) {
  return TypeListComment(
    typeListComment: (json['TypeListComment'] as List<dynamic>)
        .map((e) => e as String)
        .toList(),
  );
}

TypeInterface _$TypeInterfaceFromJson(Map<String, dynamic> json) {
  return TypeInterface(
    ident: json['ident'] as String,
    fields: (json['fields'] as List<dynamic>)
        .map((e) => InterfaceNode.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

InterfaceField _$InterfaceFieldFromJson(Map<String, dynamic> json) {
  return InterfaceField(
    ident: json['ident'] as String,
    isStatic: json['is_static'] as bool,
    ty: TypeName.fromJson(json['ty'] as Map<String, dynamic>),
  );
}

TypeStruct _$TypeStructFromJson(Map<String, dynamic> json) {
  return TypeStruct(
    ident: json['ident'] as String,
    fields: (json['fields'] as List<dynamic>)
        .map((e) => StructNode.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

StructField _$StructFieldFromJson(Map<String, dynamic> json) {
  return StructField(
    ident: json['ident'] as String,
    ty: TypeName.fromJson(json['ty'] as Map<String, dynamic>),
  );
}

TypeList _$TypeListFromJson(Map<String, dynamic> json) {
  return TypeList(
    ident: json['ident'] as String,
    fields: (json['fields'] as List<dynamic>)
        .map((e) => TypeListNode.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

TypeListField _$TypeListFieldFromJson(Map<String, dynamic> json) {
  return TypeListField(
    ident: json['ident'] as String,
    ty: TypeName.fromJson(json['ty'] as Map<String, dynamic>),
  );
}

TypeEnum _$TypeEnumFromJson(Map<String, dynamic> json) {
  return TypeEnum(
    ident: json['ident'] as String,
    fields: (json['fields'] as List<dynamic>)
        .map((e) => EnumNode.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

EnumField _$EnumFieldFromJson(Map<String, dynamic> json) {
  return EnumField(
    ident: json['ident'] as String,
  );
}

TypeConst _$TypeConstFromJson(Map<String, dynamic> json) {
  return TypeConst(
    ident: json['ident'] as String,
    fields: (json['fields'] as List<dynamic>)
        .map((e) => ConstNode.fromJson(e as Map<String, dynamic>))
        .toList(),
    constType: _constTypeFromJson(json['const_type'] as String),
  );
}

ConstField _$ConstFieldFromJson(Map<String, dynamic> json) {
  return ConstField(
    ident: json['ident'] as String,
    value: json['value'] as String,
  );
}

TypeFunction _$TypeFunctionFromJson(Map<String, dynamic> json) {
  return TypeFunction(
    args: TypeName.fromJson(json['args'] as Map<String, dynamic>),
    returnTy: TypeName.fromJson(json['return_ty'] as Map<String, dynamic>),
  );
}

TypeArray _$TypeArrayFromJson(Map<String, dynamic> json) {
  return TypeArray(
    ty: TypeName.fromJson(json['ty'] as Map<String, dynamic>),
  );
}

TypeMap _$TypeMapFromJson(Map<String, dynamic> json) {
  return TypeMap(
    mapTy: TypeName.fromJson(json['map_ty'] as Map<String, dynamic>),
    indexTy: TypeName.fromJson(json['index_ty'] as Map<String, dynamic>),
  );
}

TypePair _$TypePairFromJson(Map<String, dynamic> json) {
  return TypePair(
    firstTy: TypeName.fromJson(json['first_ty'] as Map<String, dynamic>),
    secondTy: TypeName.fromJson(json['second_ty'] as Map<String, dynamic>),
  );
}

TypeStream _$TypeStreamFromJson(Map<String, dynamic> json) {
  return TypeStream(
    sTy: TypeName.fromJson(json['s_ty'] as Map<String, dynamic>),
  );
}

TypeTuple _$TypeTupleFromJson(Map<String, dynamic> json) {
  return TypeTuple(
    fields: (json['fields'] as List<dynamic>)
        .map((e) => TupleEntry.fromJson(e as Map<String, dynamic>))
        .toList(),
  );
}

TupleEntry _$TupleEntryFromJson(Map<String, dynamic> json) {
  return TupleEntry(
    ident: json['ident'] as String,
    ty: TypeName.fromJson(json['ty'] as Map<String, dynamic>),
  );
}

TypeResult _$TypeResultFromJson(Map<String, dynamic> json) {
  return TypeResult(
    okTy: TypeName.fromJson(json['ok_ty'] as Map<String, dynamic>),
    errTy: TypeName.fromJson(json['err_ty'] as Map<String, dynamic>),
  );
}

TypeOption _$TypeOptionFromJson(Map<String, dynamic> json) {
  return TypeOption(
    someTy: TypeName.fromJson(json['some_ty'] as Map<String, dynamic>),
  );
}
