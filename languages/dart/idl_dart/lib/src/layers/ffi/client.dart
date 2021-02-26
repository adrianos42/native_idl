import 'package:code_builder/code_builder.dart';
import 'package:dart_style/dart_style.dart';
import '../../pubspec/pubspec.dart';
import '../../string_pos.dart';
import '../../dart_types.dart';
import '../../json/idl_types_json.dart';
import '../../json/ids_types_json.dart' as ids_types;
import '../../dart_layer.dart';
import '../../module.dart';

class _FFIDartTypes {
  Class _addTypeList(TypeList tyList) {
    final fields = <Field>[];

    for (var field_node in tyList.fields) {
      final node = field_node.node;

      if (node is TypeListField) {
      } else if (node is Comment) {}
    }

    return Class((b) => b
      ..name = tyList.ident
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$asValue'
        ..body = Code(_getTypeValueBodyConversionForTypeList(tyList))
        ..returns = refer('$kTypesLib.${tyList.ident}')
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('Pointer<AbiVariant>')))))
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$fromWithPtr'
        ..returns = refer('void')
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('$kTypesLib.${tyList.ident}')))
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$result'
          ..type = refer('Pointer<AbiVariant>')))
        ..body = Code(_getTypeffiConversionWithPtrBodyForTypeList(tyList))))
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$from'
        ..returns = refer('Pointer<AbiVariant>')
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('$kTypesLib.${tyList.ident}')))
        ..body = Code(_getTypeffiConversionBodyForPtr('AbiVariant'))))
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$dispose'
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('Pointer<AbiVariant>')))
        ..body = Code(_getTypeffiDisposeBodyForType())))
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$disposeWithPtr'
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('Pointer<AbiVariant>')))
        ..body = Code(_getTypeffiDisposeBodyWithPtrForTypeList(tyList))))
      ..fields.addAll(fields));
  }

  String _getTypeValueBodyConversionForTypeList(TypeList tyList) {
    var variantBody = '';

    final fields = tyList.fields
        .where((m) => m.node is TypeListField)
        .map((m) => m.node as TypeListField)
        .toList();

    for (var i = 0; i < fields.length; i += 1) {
      final ffiName =
          '\$value.ref.data!.cast<${_getffiTypeArray(fields[i].ty)}>()';
      variantBody += ''' 
        case $i:
          return $kTypesLib.${tyList.ident}.as${fields[i].ident}(${_ffiToTypeForFieldBoxed(fields[i].ty, ffiName)});''';
    }

    return ''' 
      switch (\$value.ref.variant!) {
        $variantBody
        default:
          throw ArgumentError();
      }''';
  }

  String _getTypeffiConversionWithPtrBodyForTypeList(TypeList tyList) {
    var variantBody = '';
    final fields = tyList.fields
        .where((m) => m.node is TypeListField)
        .map((m) => m.node as TypeListField)
        .toList();

    for (var i = 0; i < fields.length; i += 1) {
      variantBody += ''' 
        case $i:
          final \$tValue = \$value.value as ${DartTypes.getDartType(fields[i].ty, _packageLibrary, kTypesLib)};
          \$result.ref.data = ${_typeToffiForFieldBoxed(fields[i].ty, '\$tValue')}.cast<Void>();
          break;''';
    }

    return ''' 
      \$result.ref.variant = \$value.variant;
      switch (\$value.variant) {
        $variantBody
        default:
         throw ArgumentError();
      }''';
  }

  // ffi structure constructor.
  String _getTypeffiConversionBodyForPtr(String name) {
    return '''
      final \$result=allocate<$name>();
      _\$fromWithPtr(\$value, \$result);
      return \$result;''';
  }

  String _getTypeffiDisposeBodyWithPtrForTypeList(TypeList tyList) {
    var variantBody = '';
    final fields = tyList.fields
        .where((m) => m.node is TypeListField)
        .map((m) => m.node as TypeListField)
        .toList();

    for (var i = 0; i < fields.length; i += 1) {
      final ffiTy = _getffiTypeArray(fields[i].ty);
      variantBody += ''' 
        case $i:
          ${_typeffiForFieldBoxedDispose(fields[i].ty, '\$value.ref.data!.cast<$ffiTy>()')}
          break;''';
    }

    return variantBody.isNotEmpty
        ? ''' 
      switch (\$value.ref.variant!) {
        $variantBody
        default:
          throw ArgumentError();
      }'''
        : '';
  }

  String _getTypeffiDisposeBodyForType() {
    return '''
      _\$disposeWithPtr(\$value);
      free(\$value);''';
  }

  // ffi structure constructor with ptr as argument.
  String _getTypeffiConversionWithPtrBodyForStruct(TypeStruct tyStruct) {
    return tyStruct.fields
        .where((field_node) => field_node.node is StructField)
        .map((f) => f.node as StructField)
        .fold('', (p, field) {
      final name = toCamelCase(field.ident);
      final valueName = '\$value.$name';
      return '$p\$result.ref.$name=${_typeToffiForField(field.ty, valueName)};';
    });
  }

  String _ffiToTypeForField(TypeName tyName, String ffiName) {
    final ty =
        (DartTypes.resolveTypeWithConst(tyName, _packageLibrary)).typeName;

    if (ty is Types) {
      switch (ty) {
        case Types.natString:
          return '$ffiName.asString()';
        case Types.natBytes:
          return '$ffiName.asUint8List()';
        case Types.natUUID:
          return '$ffiName.asUuid()';
        case Types.natBool:
          return '$ffiName == 0 ? false : true';
        case Types.natInt:
        case Types.natFloat:
          return '$ffiName';
        case Types.natNone:
          return 'null';
      }
    } else if (ty is EnumTypeName) {
      return '$kTypesLib.${ty.enumTypeName}.values[$ffiName]';
    } else if (ty is ListTypeName) {
      return '${ty.listTypeName}._\$asValue($ffiName)';
    } else if (ty is StructTypeName) {
      return '${ty.structTypeName}._\$asValue($ffiName)';
    } else if (ty is TypeArray) {
      return _getArrayDataForDart(ty.ty, ffiName);
    } else if (ty is TypeMap) {
      return _getMapDataForDart(ty, ffiName);
    } else if (ty is TypePair) {
      final pairTyName = '''
          Pair<${DartTypes.getDartType(ty.firstTy, _packageLibrary, kTypesLib)}, 
            ${DartTypes.getDartType(ty.secondTy, _packageLibrary, kTypesLib)}>''';
      final firstffiName =
          '$ffiName.ref.firstData!.cast<${_getffiTypeArray(ty.firstTy)}>()';
      final secondffiName =
          '$ffiName.ref.secondData!.cast<${_getffiTypeArray(ty.secondTy)}>()';
      return '''
          $pairTyName(${_ffiToTypeForFieldBoxed(ty.firstTy, firstffiName)}, 
            ${_ffiToTypeForFieldBoxed(ty.secondTy, secondffiName)});''';
    } else if (ty is TypeOption) {
      final someffiName =
          '\$value.ref.data!.cast<${_getffiTypeArray(ty.someTy)}>()';
      return ''' 
        (Pointer<AbiVariant> \$value) {
          switch (\$value.ref.variant!) {
            case 0:
              return ${_ffiToTypeForFieldBoxed(ty.someTy, someffiName)};
            case 1:
              return null;
            default:
              throw ArgumentError();
          }
        } ($ffiName)''';
    } else if (ty is TypeResult) {
      final resultTyName = '''
          Result<${DartTypes.getDartType(ty.okTy, _packageLibrary, kTypesLib)}, 
            ${DartTypes.getDartType(ty.errTy, _packageLibrary, kTypesLib)}>''';
      final okffiName =
          '\$value.ref.data!.cast<${_getffiTypeArray(ty.okTy)}>()';
      final errffiName =
          '\$value.ref.data!.cast<${_getffiTypeArray(ty.errTy)}>()';
      return ''' 
        (Pointer<AbiVariant> \$value) {
          switch (\$value.ref.variant!) {
            case 0:
              return $resultTyName.ok(${_ffiToTypeForFieldBoxed(ty.okTy, okffiName)});
            case 1:
              return $resultTyName.err(${_ffiToTypeForFieldBoxed(ty.errTy, errffiName)});
            default:
              throw ArgumentError();
          }
        } ($ffiName)''';
    }

    throw Exception('Invalid type for dart code: ``');
  }

  String _ffiToTypeForFieldBoxed(TypeName tyName, String ffiName) {
    if (_packageLibrary.isTypePrimitive(tyName)) {
      final name = '$ffiName.value';
      return '${_ffiToTypeForField(tyName, name)}';
    } else {
      return _ffiToTypeForField(tyName, ffiName);
    }
  }

  String _typeToffiForFieldBoxed(TypeName tyName, String valueName) {
    if (_packageLibrary.isTypePrimitive(tyName)) {
      return ''' 
        (${DartTypes.getDartType(tyName, _packageLibrary, kTypesLib)} \$value) {
          final \$result = allocate<${_getffiTypeArray(tyName)}>();
          \$result.value = ${_typeToffiForField(tyName, '\$value')};
          return \$result;
        }($valueName)''';
    } else {
      return _typeToffiForField(tyName, valueName);
    }
  }

  String _typeToffiForField(TypeName tyName, String valueName) {
    final ty =
        (DartTypes.resolveTypeWithConst(tyName, _packageLibrary)).typeName;

    if (ty is Types) {
      switch (ty) {
        case Types.natString:
          return '$valueName.asAbiString()';
        case Types.natBytes:
          return '$valueName.asAbiBytes()';
        case Types.natUUID:
          return '$valueName.asAbiUuid()';
        case Types.natBool:
          return '$valueName ? 1 : 0';
        case Types.natInt:
        case Types.natFloat:
          return '$valueName';
        case Types.natNone:
          return '0';
      }
    } else if (ty is EnumTypeName) {
      return '$kTypesLib.${ty.enumTypeName}.values.indexOf($valueName)';
    } else if (ty is ListTypeName) {
      return '${ty.listTypeName}._\$from($valueName)';
    } else if (ty is StructTypeName) {
      return '${ty.structTypeName}._\$from($valueName)';
    } else if (ty is TypeArray) {
      return '${_setArrayDataForffi(ty.ty, valueName)}';
    } else if (ty is TypeMap) {
      return '${_setMapDataForffi(ty, valueName)}';
    } else if (ty is TypePair) {
      return ''' 
        (Pair<${DartTypes.getDartType(ty.firstTy, _packageLibrary, kTypesLib)}, ${DartTypes.getDartType(ty.secondTy, _packageLibrary, kTypesLib)}> \$value) {
          final \$result = allocate<AbiPair>();
          \$result.ref.firstData = ${_typeToffiForFieldBoxed(ty.firstTy, '\$value.first')}.cast<Void>();
          \$result.ref.secondData = ${_typeToffiForFieldBoxed(ty.secondTy, '\$value.second')}.cast<Void>();
          return \$result;
        }($valueName)''';
    } else if (ty is TypeOption) {
      return ''' 
        (${DartTypes.getDartType(ty.someTy, _packageLibrary, kTypesLib)}? \$value) {
          final \$result = allocate<AbiVariant>();
          if (\$value != null) {
            \$result.ref.variant = 0;
            \$result.ref.data = ${_typeToffiForFieldBoxed(ty.someTy, '\$value')}.cast<Void>();
          } else {
            \$result.ref.variant = 1;
            \$result.ref.data = null;
          }
          return \$result;
        }($valueName)''';
    } else if (ty is TypeResult) {
      return ''' 
        (Result<${DartTypes.getDartType(ty.okTy, _packageLibrary, kTypesLib)}, ${DartTypes.getDartType(ty.errTy, _packageLibrary, kTypesLib)}> \$value) {
          final \$result = allocate<AbiVariant>();
          if (\$value.ok != null) {
            \$result.ref.variant = 0;
            \$result.ref.data = ${_typeToffiForFieldBoxed(ty.okTy, '\$value.ok')}.cast<Void>();
          } else if (\$value.err != null) {
            \$result.ref.variant = 1;
            \$result.ref.data = ${_typeToffiForFieldBoxed(ty.errTy, '\$value.err')}.cast<Void>();
          } else {
            throw ArgumentError();
          }
          return \$result;
        }($valueName)''';
    }

    throw Exception('Invalid type for dart code: ``');
  }

  String _typeToffiForFieldWithPtr(
      TypeName tyName, String valueName, String ptrName) {
    final ty =
        (DartTypes.resolveTypeWithConst(tyName, _packageLibrary)).typeName;

    if (ty is Types) {
      switch (ty) {
        case Types.natString:
          return '$valueName.asAbiStringWithPtr($ptrName)';
        case Types.natBytes:
          return '$valueName.asAbiBytesWithPtr($ptrName)';
        case Types.natUUID:
          return '$valueName.asAbiUuidWithPtr($ptrName)';
        default:
          throw ArgumentError();
      }
    } else if (ty is ListTypeName) {
      return '${ty.listTypeName}._\$fromWithPtr($valueName, $ptrName)';
    } else if (ty is StructTypeName) {
      return '${ty.structTypeName}._\$fromWithPtr($valueName, $ptrName)';
    } else if (ty is TypeArray) {
      return '${_setArrayDataForffiWithPtr(ty.ty, valueName, ptrName)}';
    }

    throw Exception('Invalid type for dart code: `${ty}`');
  }

  String _typeffiForFieldBoxedDispose(TypeName tyName, String valueName) {
    if (_packageLibrary.isTypePrimitive(tyName)) {
      return 'free($valueName);';
    } else {
      return '${_typeffiForFieldDispose(tyName, '$valueName')}';
    }
  }

  String _typeffiForFieldDispose(TypeName tyName, String valueName) {
    final ty =
        (DartTypes.resolveTypeWithConst(tyName, _packageLibrary)).typeName;

    if (ty is Types) {
      switch (ty) {
        case Types.natString:
        case Types.natBytes:
        case Types.natUUID:
          return '$valueName.dispose();';
        default:
      }
    } else if (ty is ListTypeName) {
      return '${ty.listTypeName}._\$dispose($valueName);';
    } else if (ty is StructTypeName) {
      return '${ty.structTypeName}._\$dispose($valueName);';
    } else if (ty is TypeArray) {
      return '${_setArrayDataDisposeForffi(ty.ty, valueName)}';
    } else if (ty is TypeMap) {
      return '${_setMapDataDisposeForffi(ty, valueName)}';
    } else if (ty is TypePair) {
      final firstffiTy = _getffiTypeArray(ty.firstTy);
      final secondffiTy = _getffiTypeArray(ty.secondTy);
      final dispose = '''
        ${_typeffiForFieldBoxedDispose(ty.firstTy, '$valueName.ref.firstData!.cast<$firstffiTy>()')}
        ${_typeffiForFieldBoxedDispose(ty.secondTy, '$valueName.ref.secondData!.cast<$secondffiTy>()')}''';
      return '$dispose';
    } else if (ty is TypeOption) {
      final sffiTy = _getffiTypeArray(ty.someTy);
      return ''' 
        if ($valueName.ref.variant == 0) {
          ${_typeffiForFieldBoxedDispose(ty.someTy, '$valueName.ref.data!.cast<$sffiTy>()')}
        }''';
    } else if (ty is TypeResult) {
      final sffiTy = _getffiTypeArray(ty.okTy);
      final sffiErrTy = _getffiTypeArray(ty.errTy);

      final okDispose = '''
        if ($valueName.ref.variant == 0) {
          ${_typeffiForFieldBoxedDispose(ty.okTy, '$valueName.ref.data!.cast<$sffiTy>()')}
        }''';

      final errDispose = '''
        else if ($valueName.ref.variant == 1) {
          ${_typeffiForFieldBoxedDispose(ty.errTy, '$valueName.ref.data!.cast<$sffiErrTy>()')}
        }''';

      return '$okDispose$errDispose';
    }

    return '';
  }

  String _setMapDataForffi(TypeMap map, String valueName) {
    var body = '''
      (Map<${DartTypes.getDartType(map.indexTy, _packageLibrary, kTypesLib)},
        ${DartTypes.getDartType(map.mapTy, _packageLibrary, kTypesLib)}> \$value) {
        final \$result = allocate<AbiMap>();
        final \$entries = \$value.entries.toList();
        final \$length = \$entries.length;
        final \$values = \$entries.map((\$m) => \$m.value).toList();
        final \$keyValues = \$entries.map((\$m) => \$m.key).toList();
        final \$data = allocate<${_getffiTypeArray(map.mapTy)}>(count: \$length);
        final \$key = allocate<${_getffiTypeArray(map.indexTy)}>(count: \$length);
        \$result.ref.data = \$data.cast<Void>();
        \$result.ref.key = \$key.cast<Void>();
        \$result.ref.length = \$length;''';

    final mTy =
        (DartTypes.resolveTypeWithConst(map.mapTy, _packageLibrary)).typeName;

    if (mTy is Types) {
      switch (mTy) {
        case Types.natInt:
        case Types.natFloat:
          body += '''
            final \$listSource = \$data.asTypedList(\$length);
            \$listSource.setAll(0, \$values);''';
          break;
        case Types.natBool:
          body += '''
            final \$listSource = \$data.asTypedList(\$length);
            \$listSource.setAll(0, \$values.map((\$value) => \$value ? 1 : 0));''';
          break;
        case Types.natString:
        case Types.natBytes:
        case Types.natUUID:
          body += '''
            for (var \$i = 0; \$i < \$length; \$i += 1) {
              ${_typeToffiForFieldWithPtr(map.mapTy, '\$values[\$i]', '\$data.elementAt(\$i)')};
            }''';
          break;
        default:
          throw Exception('Invalid type for array in code: `${mTy.toString}`');
      }
    } else if (mTy is StructTypeName) {
      body += '''
        for (var \$i = 0; \$i < \$length; \$i += 1) {
          ${mTy.structTypeName}._\$fromWithPtr(\$values[\$i], \$data.elementAt(\$i));
        }''';
    } else if (mTy is ListTypeName) {
      body += '''
        for (var \$i = 0; \$i < \$length; \$i += 1) {
          ${mTy.listTypeName}._\$fromWithPtr(\$values[\$i], \$data.elementAt(\$i));
        }''';
    } else if (mTy is EnumTypeName) {
      body += '''
        final \$listSource = \$data.asTypedList(\$length);
        \$listSource.setAll(0, \$values.map((\$ma) => $kTypesLib.${mTy.enumTypeName}.values.indexOf(\$ma)));''';
    } else {
      throw ArgumentError('Invalid type for map `${mTy}`');
    }

    final indexTy = map.indexTy.typeName;

    if (indexTy is Types &&
        (indexTy == Types.natInt || indexTy == Types.natString)) {
      if (indexTy == Types.natInt) {
        body += '''
          final \$listSourceKey = \$key.asTypedList(\$length);
          \$listSourceKey.setAll(0, \$keyValues);''';
      } else {
        body += '''
          for (var \$i = 0; \$i < \$length; \$i += 1) {
            ${_typeToffiForFieldWithPtr(map.indexTy, '\$keyValues[\$i]', '\$key.elementAt(\$i)')};
          }''';
      }
    } else {
      throw ArgumentError('Invalid type for map index `$indexTy`');
    }

    return body += '''
        return \$result;
      }($valueName)''';
  }

  String _setMapDataDisposeForffi(TypeMap map, String valueName) {
    var body = '''
      final \$data = $valueName.ref.data!.cast<${_getffiTypeArray(map.mapTy)}>();
      final \$key = $valueName.ref.key!.cast<${_getffiTypeArray(map.indexTy)}>();''';

    final mTy =
        (DartTypes.resolveTypeWithConst(map.mapTy, _packageLibrary)).typeName;

    if (mTy is Types) {
      switch (mTy) {
        case Types.natString:
        case Types.natBytes:
        case Types.natUUID:
          body = '''$body
            for (var \$i = 0; \$i < $valueName.ref.length!; \$i += 1) {
              ${_getffiTypeArray(map.mapTy)}.disposeWithPtr(\$data.elementAt(\$i));
            }''';
          break;
        default:
          break;
      }
    } else if (mTy is StructTypeName) {
      body = '''$body
        for (var \$i = 0; \$i < $valueName.ref.length!; \$i += 1) {
          ${mTy.structTypeName}._\$disposeWithPtr(\$data.elementAt(\$i));
        }''';
    } else if (mTy is ListTypeName) {
      body = '''$body
        for (var \$i = 0; \$i < $valueName.ref.length!; \$i += 1) {
          ${mTy.listTypeName}._\$disposeWithPtr(\$data.elementAt(\$i));
        }''';
    }

    final indexTy = map.indexTy.typeName as Types;

    if (indexTy == Types.natString) {
      body = '''$body
        for (var \$i = 0; \$i < $valueName.ref.length!; \$i += 1) {
          ${_getffiTypeArray(map.indexTy)}.disposeWithPtr(\$key.elementAt(\$i));
        }''';
    }

    return '''$body
      free(\$data);
      free(\$key);
      free($valueName);''';
  }

  String _getMapDataForDart(TypeMap map, String ffiName) {
    var body = '''
      (Pointer<AbiMap> \$value) {
        final \$length = \$value.ref.length!;
        final \$data = ${_getMapBodyDataForDart(map.mapTy, '\$value.ref.data!', '\$length')};
        final \$keys = ${_getMapBodyDataForDart(map.indexTy, '\$value.ref.key!', '\$length')};
        final \$result = <${DartTypes.getDartType(map.indexTy, _packageLibrary, kTypesLib)}, 
        ${DartTypes.getDartType(map.mapTy, _packageLibrary, kTypesLib)}>{};
        for (var \$i = 0; \$i < \$length; \$i += 1) {
          \$result[\$keys[\$i]] = \$data[\$i];
        }
        return \$result;
      }($ffiName)''';

    return body;
  }

  String _getMapBodyDataForDart(
      TypeName arrayType, String dataName, String lengthName) {
    var body = '(Pointer<Void> \$data, int \$length) {';

    final aTy =
        (DartTypes.resolveTypeWithConst(arrayType, _packageLibrary)).typeName;

    if (aTy is Types) {
      switch (aTy) {
        case Types.natInt:
          body += 'return \$data.cast<Int64>().asTypedList(\$length).toList();';
          break;
        case Types.natFloat:
          body +=
              'return \$data.cast<Double>().asTypedList(\$length).toList();';
          break;
        case Types.natBool:
          body += '''
            return \$data.cast<Int64>().asTypedList(\$length).listSource.map((\$mValue) => 
              \$mValue == 0 ? false : true).toList();''';
          break;
        case Types.natString:
          body += '''
            final \$listSource = \$data.cast<AbiString>();
            final \$result = <String>[];
            for (var \$i = 0; \$i < \$length; \$i += 1) {
              \$result.add(\$listSource.elementAt(\$i).asString());
            }
            return \$result;''';
          break;
        case Types.natBytes:
          body += '''
            final \$listSource = \$data.cast<AbiBytes>();
            final \$result = <Uint8List>[];
            for (var \$i = 0; \$i < \$length; \$i += 1) {
              \$result.add(\$listSource.elementAt(\$i).asUint8List());
            }
            return \$result;''';
            break;
        case Types.natUUID:
          body += '''
            final \$listSource = \$data.cast<AbiUuid>();
            final \$result = <String>[];
            for (var \$i = 0; \$i < \$length; \$i += 1) {
              \$result.add(\$listSource.elementAt(\$i).asUuid());
            }
            return \$result;''';
          break;
        default:
          throw Exception('Invalid type for array in code: `${aTy.toString}`');
      }
    } else if (aTy is StructTypeName) {
      body += '''
        final \$listSource = \$data.cast<${aTy.structTypeName}>();
        final \$result = <$kTypesLib.${aTy.structTypeName}>[];
        for (var \$i = 0; \$i < \$length; \$i += 1) {
          \$result.add(${aTy.structTypeName}._\$asValue(\$listSource.elementAt(\$i)));
        }
        return \$result;''';
    } else if (aTy is ListTypeName) {
      body += '''
        final \$listSource = \$data.cast<AbiVariant>();
        final \$result = <$kTypesLib.${aTy.listTypeName}>[];
        for (var \$i = 0; \$i < \$length; \$i += 1) {
          \$result.add(${aTy.listTypeName}._\$asValue(\$listSource.elementAt(\$i)));
        }
        return \$result;''';
    } else if (aTy is EnumTypeName) {
      body += '''
        return \$data.cast<Int64>().asTypedList(\$length).map((\$mValue) => 
          $kTypesLib.${aTy.enumTypeName}.values[\$mValue]).toList();''';
    } else {
      throw Exception();
    }

    body += '}($dataName, $lengthName)';

    return body;
  }

  String _setArrayDataForffi(TypeName arrayType, String valueName) {
    var body = '''
      (List<${DartTypes.getDartType(arrayType, _packageLibrary, kTypesLib)}> \$value) {
        final \$result = allocate<AbiArray>();
        ${_setArrayDataForffiWithPtr(arrayType, '\$value', '\$result')}
        return \$result; 
      }($valueName)''';

    return body;
  }

  String _setArrayDataForffiWithPtr(
      TypeName arrayType, String valueName, String ptrName) {
    final aTy =
        (DartTypes.resolveTypeWithConst(arrayType, _packageLibrary)).typeName;
    final body;

    if (aTy is Types) {
      switch (aTy) {
        case Types.natInt:
        case Types.natFloat:
          body = '''
            final \$listSource = \$data.asTypedList($valueName.length);
            \$listSource.setAll(0, $valueName);''';
          break;
        case Types.natBool:
          body = '''
            final \$listSource = \$data.asTypedList($valueName.length);
            \$listSource.setAll(0, $valueName.map((\$value) => \$value ? 1 : 0));''';
          break;
        case Types.natString:
        case Types.natBytes:
        case Types.natUUID:
          body = '''
            for (var \$i = 0; \$i < $valueName.length; \$i += 1) {
              ${_typeToffiForFieldWithPtr(arrayType, '$valueName[\$i]', '\$data.elementAt(\$i)')};
            }''';
          break;
        default:
          throw Exception('Invalid type for array in code: `${aTy.toString}`');
      }
    } else if (aTy is StructTypeName) {
      body = '''
        for (var \$i = 0; \$i < $valueName.length; \$i += 1) {
          ${aTy.structTypeName}._\$fromWithPtr($valueName[\$i], \$data.elementAt(\$i));
        }''';
    } else if (aTy is ListTypeName) {
      body = '''
        for (var \$i = 0; \$i < $valueName.length; \$i += 1) {
          ${aTy.listTypeName}._\$fromWithPtr($valueName[\$i], \$data.elementAt(\$i));
        }''';
    } else if (aTy is EnumTypeName) {
      body = '''
        final \$listSource = \$data.asTypedList($valueName.length);
        \$listSource.setAll(0, $valueName.map((\$ma) => $kTypesLib.${aTy.enumTypeName}.values.indexOf(\$ma)));''';
    } else if (aTy is TypeArray) {
      body = '''
        for (var \$i = 0; \$i < $valueName.length; \$i += 1) {
          ${_setArrayDataForffiWithPtr(aTy.ty, '$valueName[\$i]', '\$data.elementAt(\$i)')};
        }''';
    } else {
      throw ArgumentError('Invalid type for array ${arrayType.typeName}');
    }

    return '''
      final \$data = allocate<${_getffiTypeArray(arrayType)}>(count: $valueName.length);
      $body
      $ptrName.ref.data = \$data.cast<Void>();
      $ptrName.ref.length = $valueName.length;''';
  }

  String _setArrayDataDisposeForffi(TypeName arrayType, String valueName) {
    return '''
      ${_setArrayDataDisposeForffiWithPtr(arrayType, valueName)}
      free($valueName);''';
  }

  String _setArrayDataDisposeForffiWithPtr(
      TypeName arrayType, String valueName) {
    final aTy =
        (DartTypes.resolveTypeWithConst(arrayType, _packageLibrary)).typeName;
    var body = '''
      final \$data = $valueName.ref.data!.cast<${_getffiTypeArray(arrayType)}>();''';

    if (aTy is Types) {
      switch (aTy) {
        case Types.natString:
        case Types.natBytes:
        case Types.natUUID:
          body += '''
            for (var \$i = 0; \$i < $valueName.ref.length!; \$i += 1) {
              ${_getffiTypeArray(arrayType)}.disposeWithPtr(\$data.elementAt(\$i));
            }''';
          break;
        default:
          break;
      }
    } else if (aTy is StructTypeName) {
      body += '''
        for (var \$i = 0; \$i < $valueName.ref.length!; \$i += 1) {
          ${aTy.structTypeName}._\$disposeWithPtr(\$data.elementAt(\$i));
        }''';
    } else if (aTy is ListTypeName) {
      body += '''
        for (var \$i = 0; \$i < $valueName.ref.length!; \$i += 1) {
          ${aTy.listTypeName}._\$disposeWithPtr(\$data.elementAt(\$i));
        }''';
    } else if (aTy is TypeArray) {
      body += '''
        for (var \$i = 0; \$i < $valueName.ref.length!; \$i += 1) {
          ${_setArrayDataDisposeForffiWithPtr(aTy.ty, '\$data.elementAt(\$i)')};
        }''';
    }

    return '$body free(\$data);';
  }

  String _getArrayDataForDart(TypeName arrayType, String ffiName) {
    var body = '''
      (Pointer<AbiArray> \$value) {
        final \$data = \$value.ref.data!;
        final \$length = \$value.ref.length!;''';

    final aTy =
        (DartTypes.resolveTypeWithConst(arrayType, _packageLibrary)).typeName;

    if (aTy is Types) {
      switch (aTy) {
        case Types.natInt:
          body += 'return \$data.cast<Int64>().asTypedList(\$length).toList();';
          break;
        case Types.natFloat:
          body +=
              'return \$data.cast<Double>().asTypedList(\$length).toList();';
          break;
        case Types.natBool:
          body += '''
            return \$data.cast<Int64>().asTypedList(\$length).listSource.map((\$mValue) => 
              \$mValue == 0 ? false : true).toList();''';
          break;
        case Types.natString:
          body += '''
            final \$listSource = \$data.cast<AbiString>();
            final \$result = <String>[];
            for (var \$i = 0; \$i < \$length; \$i += 1) {
              \$result.add(\$listSource.elementAt(\$i).asString());
            }
            return \$result;''';
          break;
        case Types.natBytes:
          body += '''
            final \$listSource = \$data.cast<AbiBytes>();
            final \$result = <Uint8List>[];
            for (var \$i = 0; \$i < \$length; \$i += 1) {
              \$result.add(\$listSource.elementAt(\$i).asUint8List());
            }
            return \$result;''';
          break;
        case Types.natUUID:
          body += '''
            final \$listSource = \$data.cast<AbiUuid>();
            final \$result = <String>[];
            for (var \$i = 0; \$i < \$length; \$i += 1) {
              \$result.add(\$listSource.elementAt(\$i).asUuid());
            }
            return \$result;''';
          break;
        default:
          throw Exception('Invalid type for array in code: `${aTy.toString}`');
      }
    } else if (aTy is StructTypeName) {
      body += '''
        final \$listSource = \$data.cast<${aTy.structTypeName}>();
        final \$result = <$kTypesLib.${aTy.structTypeName}>[];
        for (var \$i = 0; \$i < \$length; \$i += 1) {
          \$result.add(${aTy.structTypeName}._\$asValue(\$listSource.elementAt(\$i)));
        }
        return \$result;''';
    } else if (aTy is EnumTypeName) {
      body += '''
        return \$data.cast<Int64>().asTypedList(\$length).map((\$mValue) => 
          $kTypesLib.${aTy.enumTypeName}.values[\$mValue]).toList();''';
    } else if (aTy is TypeArray) {
      body += '''
        final \$listSource = \$data.cast<AbiArray>();
        final \$result = <List<${DartTypes.getDartType(aTy.ty, _packageLibrary, kTypesLib)}>>[];
        for (var \$i = 0; \$i < \$length; \$i += 1) {
          \$result.add(${_getArrayDataForDart(aTy.ty, '\$listSource.elementAt(\$i)')});
        }
        return \$result;''';
    } else if (aTy is ListTypeName) {
      body += '''
        final \$listSource = \$data.cast<AbiVariant>();
        final \$result = <$kTypesLib.${aTy.listTypeName}>[];
        for (var \$i = 0; \$i < \$length; \$i += 1) {
          \$result.add(${aTy.listTypeName}._\$asValue(\$listSource.elementAt(\$i)));
        }
        return \$result;''';
    } else {
      throw ArgumentError('Invalid type for array');
    }

    body += '}($ffiName)';

    return body;
  }

  String _getffiTypeArray(TypeName type) {
    final ty = (DartTypes.resolveTypeWithConst(type, _packageLibrary)).typeName;

    if (ty is Types) {
      switch (ty) {
        case Types.natBool:
        case Types.natInt:
        case Types.natNone:
          return 'Int64';
        case Types.natFloat:
          return 'Double';
        case Types.natString:
          return 'AbiString';
        case Types.natBytes:
          return 'AbiBytes';
        case Types.natUUID:
          return 'AbiUuid';
        default:
          throw ArgumentError('Invalid `Types`');
      }
    } else if (ty is ListTypeName || ty is TypeOption || ty is TypeResult) {
      return 'AbiVariant';
    } else if (ty is TypeStream) {
      return 'AbiStream';
    } else if (ty is TypeArray) {
      return 'AbiArray';
    } else if (ty is TypeMap) {
      return 'AbiMap';
    } else if (ty is TypePair) {
      return 'AbiPair';
    } else if (ty is EnumTypeName) {
      return 'Int64';
    } else if (ty is StructTypeName) {
      return ty.structTypeName;
    } else if (ty is InterfaceTypeName) {
      return ty.interfaceTypeName;
    }

    throw ArgumentError('Could not convert array type');
  }

  String _getffiTypeForInterface(TypeName type) {
    final ty = (DartTypes.resolveTypeWithConst(type, _packageLibrary)).typeName;

    if (ty is Types) {
      switch (ty) {
        case Types.natBool:
        case Types.natInt:
        case Types.natNone:
          return 'Int64';
        case Types.natFloat:
          return 'Double';
        case Types.natString:
          return 'Pointer<AbiString>';
        case Types.natBytes:
          return 'Pointer<AbiBytes>';
        case Types.natUUID:
          return 'Pointer<AbiUuid>';
      }
    } else if (ty is ListTypeName || ty is TypeOption || ty is TypeResult) {
      return 'Pointer<AbiVariant>';
    } else if (ty is TypeStream) {
      return 'Pointer<AbiStream>';
    } else if (ty is TypeArray) {
      return 'Pointer<AbiArray>';
    } else if (ty is TypeMap) {
      return 'Pointer<AbiMap>';
    } else if (ty is TypePair) {
      return 'Pointer<AbiPair>';
    } else if (ty is EnumTypeName) {
      return 'Int64';
    } else if (ty is StructTypeName) {
      return 'Pointer<${ty.structTypeName}>';
    } else if (ty is InterfaceTypeName) {
      return 'Pointer<${ty.interfaceTypeName}>';
    }

    throw ArgumentError('Could not convert ffi type for interface');
  }

  String _getDartTypeForffi(TypeName type) {
    final ty = (DartTypes.resolveTypeWithConst(type, _packageLibrary)).typeName;

    if (ty is Types) {
      switch (ty) {
        case Types.natFloat:
          return 'double';
        case Types.natString:
          return 'Pointer<AbiString>';
        case Types.natBytes:
          return 'Pointer<AbiBytes>';
        case Types.natUUID:
          return 'Pointer<AbiUuid>';
        case Types.natInt:
        case Types.natBool:
        case Types.natNone:
          return 'int';
      }
    } else if (ty is InterfaceTypeName) {
      return 'Pointer<${ty.interfaceTypeName}>';
    } else if (ty is StructTypeName) {
      return 'Pointer<${ty.structTypeName}>';
    } else if (ty is ListTypeName || ty is TypeOption || ty is TypeResult) {
      return 'Pointer<AbiVariant>';
    } else if (ty is EnumTypeName) {
      return 'int';
    } else if (ty is TypeArray) {
      return 'Pointer<AbiArray>';
    } else if (ty is TypeMap) {
      return 'Pointer<AbiMap>';
    } else if (ty is TypePair) {
      return 'Pointer<AbiPair>';
    } else if (ty is TypeStream) {
      return 'Pointer<AbiStream>';
    }

    throw Exception(
        'Invalid type in interface for dart code: [${type.toString()}]');
  }

  // ffi structure constructor with ptr as argument.
  String _getffiTypeConversionForStruct(TypeStruct tyStruct) {
    return tyStruct.fields
        .where((field_node) => field_node.node is StructField)
        .map((f) => f.node as StructField)
        .fold('', (p, field) {
      final name = toCamelCase(field.ident);
      final ffiName = '\$value.ref.$name!';
      return '${p.isNotEmpty ? '$p,' : ''}${_ffiToTypeForField(field.ty, ffiName)}';
    });
  }

  Class _addStruct(TypeStruct tyStruct) {
    final fields = <Field>[];

    for (var field_node in tyStruct.fields) {
      final node = field_node.node;

      if (node is StructField) {
        final annts = <Expression>[];
        final ty = (DartTypes.resolveTypeWithConst(node.ty, _packageLibrary));

        if (ty.typeName is Types) {
          switch (ty.typeName) {
            case Types.natBool:
            case Types.natInt:
            case Types.natNone:
              annts.add(refer('Int64()'));
              break;
            case Types.natFloat:
              annts.add(refer('Double()'));
              break;
            default:
          }
        } else if (ty.typeName is EnumTypeName) {
          annts.add(refer('Int64()'));
        }

        fields.add(Field((b) => b
          ..name = toCamelCase(node.ident)
          ..type = refer('${_getDartTypeForffi(ty)}?')
          ..annotations.addAll(annts)));
      } else if (node is Comment) {}
    }

    return Class((b) => b
      ..name = tyStruct.ident
      ..extend = refer('Struct')
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$asValue'
        ..lambda = true
        ..body = Code(
            '$kTypesLib.${tyStruct.ident}(${_getffiTypeConversionForStruct(tyStruct)})')
        ..returns = refer('$kTypesLib.${tyStruct.ident}')
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('Pointer<${tyStruct.ident}>')))))
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$fromWithPtr'
        ..returns = refer('void')
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('$kTypesLib.${tyStruct.ident}')))
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$result'
          ..type = refer('Pointer<${tyStruct.ident}>')))
        ..body = Code(_getTypeffiConversionWithPtrBodyForStruct(tyStruct))))
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$from'
        ..returns = refer('Pointer<${tyStruct.ident}>')
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('$kTypesLib.${tyStruct.ident}')))
        ..body = Code(_getTypeffiConversionBodyForPtr(tyStruct.ident))))
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$dispose'
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('Pointer<${tyStruct.ident}>')))
        ..body = Code(_getTypeffiDisposeBodyForType())))
      ..methods.add(Method((b) => b
        ..static = true
        ..name = '_\$disposeWithPtr'
        ..requiredParameters.add(Parameter((b) => b
          ..name = '\$value'
          ..type = refer('Pointer<${tyStruct.ident}>')))
        ..body = Code(_getTypeffiDisposeBodyWithPtrForStruct(tyStruct))))
      ..fields.addAll(fields));
  }

  String _getTypeffiDisposeBodyWithPtrForStruct(TypeStruct tyStruct) {
    return tyStruct.fields
        .where((field_node) => field_node.node is StructField)
        .map((f) => f.node as StructField)
        .fold('', (p, field) {
      if (!_packageLibrary.isTypePrimitive(field.ty)) {
        final name = toCamelCase(field.ident);
        final valueName = '\$value.ref.$name!';
        return p + '${_typeffiForFieldDispose(field.ty, valueName)}';
      }
      return p;
    });
  }

  String _typeToffiForInterfaceField(
    TypeInterface tyInterface,
    InterfaceField field,
    String fieldFunctionName,
    String fieldDisposeFunctionName,
  ) {
    var body = '';
    var setValue = '';
    var finallyBlock = '';
    final instanceArg = field.isStatic ? '' : '\$instance';
    var argList = instanceArg;

    final errorMessage = '${tyInterface.ident}.${toCamelCase(field.ident)}';

    final tyName = field.ty.typeName;

    final getArgs = (TypeTuple tupleTy) {
      for (var value in tupleTy.fields) {
        if (argList.isNotEmpty) {
          argList += ',';
        }

        if (value.ty.typeName is TypeStream) {
          final resultName = '\$fValue${toPascalCase(value.ident)}';
          argList += '$resultName';
          body += '''
            final $resultName = allocate<AbiStream>();
            final \$streamDataValue = allocate<Int64>();''';

          // TODO see length
          setValue += '''
            \$streamDataValue.value = -1;
            \$fValue${toPascalCase(value.ident)}.ref.wakeHandle = \$wakePortSend;
            \$fValue${toPascalCase(value.ident)}.ref.wakeObject = DartCObjectInt.newObject(\$wakeObjectSend).cast();
            \$fValue${toPascalCase(value.ident)}.ref.wakeCallback = NativeApi.postCObject.cast();
            \$fValue${toPascalCase(value.ident)}.ref.state = AbiStreamSenderState.waiting;
            \$streamValue.ref.data = \$streamDataValue.cast();''';

          finallyBlock += 'free(\$streamDataValue);free($resultName);';
        } else {
          final valueName = toCamelCase(value.ident);
          if (_packageLibrary.isTypePrimitive(value.ty)) {
            argList += _typeToffiForField(value.ty, valueName);
          } else {
            final resultName = '\$fValue${toPascalCase(value.ident)}';
            argList += '$resultName';
            body +=
                'final $resultName = ${_typeToffiForField(value.ty, valueName)};';
            finallyBlock += _typeffiForFieldDispose(value.ty, resultName);
          }
        }
      }
    };

    final getRet = (TypeName ty) {
      if (argList.isNotEmpty) argList += ',';

      if (ty.typeName is TypeStream) {
        body += 'final \$streamValue = allocate<AbiStream>();';
        argList += '\$streamValue';

        setValue += '''
          \$streamValue.ref.wakeHandle = \$wakePort;
          \$streamValue.ref.wakeObject = DartCObjectInt.newObject(\$wakeObject).cast();
          \$streamValue.ref.wakeCallback = NativeApi.postCObject.cast();
          \$streamValue.ref.state = AbiStreamReceiverState.start;
          AbiInternalError.handleError($fieldFunctionName($argList), \'${errorMessage}\');''';

        finallyBlock += 'free(\$streamValue);';
      } else {
        argList += '\$fValue';
        body += 'final \$fValue = allocate<${_getffiTypeForInterface(ty)}>();';

        setValue +=
            'AbiInternalError.handleError($fieldFunctionName($argList), \'${errorMessage}\');';

        if (!_packageLibrary.isTypePrimitive(ty)) {
          final disposeArgList = instanceArg.isEmpty
              ? '\$fValue.value'
              : '$instanceArg,\$fValue.value';
          setValue += '''
            try {
              final \$result = ${_ffiToTypeForField(ty, '\$fValue.value')};
              return \$result;
            } finally {
              AbiInternalError.handleError($fieldDisposeFunctionName($disposeArgList), \'${errorMessage}\'); 
            }''';
        } else {
          setValue += '''
            final \$result = ${_ffiToTypeForField(ty, '\$fValue.value')};;
            return \$result;''';
        }

        finallyBlock += 'free(\$fValue);';
      }
    };

    if (tyName is TypeFunction) {
      getArgs(tyName.args.typeName);
      getRet(tyName.returnTy);
    } else if (tyName is TypeTuple) {
      getArgs(tyName);
      setValue =
          'AbiInternalError.handleError($fieldFunctionName($argList), \'${errorMessage}\');';
    } else {
      getRet(field.ty);
    }

    if (finallyBlock.isNotEmpty) {
      return '$body try { $setValue } finally { $finallyBlock }';
    }

    return '$body $setValue';
  }

  String _typeToffiForInterfaceFieldStream(
    TypeInterface tyInterface,
    InterfaceField field,
    String fieldFunctionName,
    String fieldDisposeFunctionName,
  ) {
    var body = '';
    var setValue = '';
    var finallyBlock = '';
    final instanceArg = field.isStatic ? '' : '\$instance';
    var argList = instanceArg;

    final errorMessage = '${tyInterface.ident}.${toCamelCase(field.ident)}';

    if (argList.isNotEmpty) argList += ',';

    argList += '\$streamValue,\$fValue';

    body += ''' 
      final \$streamValue = allocate<AbiStream>();
      final \$fValue = allocate<Pointer<AbiStream>>();''';

    final streamTy = PackageLibrary.fieldStreamReturnTy(field);
    final streamffiTy = _getffiTypeArray(streamTy);

    final disposeArgList =
        instanceArg.isEmpty ? '\$fValue.value' : '$instanceArg,\$fValue.value';

    setValue += '''
      \$streamValue.ref.wakeHandle = \$wakePort;
      \$streamValue.ref.wakeObject = DartCObjectInt.newObject(\$wakeObject).cast();
      \$streamValue.ref.wakeCallback = NativeApi.postCObject.cast();
      \$streamValue.ref.state = \$streamState;
      AbiInternalError.handleError($fieldFunctionName($argList), \'${errorMessage}\');
      try {
        final \$stream = \$fValue.value;
        switch (\$stream.ref.state!) {
          case AbiStreamSenderState.ok:
            break;
          case AbiStreamSenderState.done:
            \$sendPort.send([\$wakeObject, \$stream.ref.state!, 0]);
            break;
          case AbiStreamSenderState.value:
            final \$result = ${_ffiToTypeForFieldBoxed(streamTy, '\$stream.ref.data!.cast<$streamffiTy>()')};
            \$sendPort.send([\$wakeObject, \$stream.ref.state!, \$result]);
            break;
          default:
            throw ArgumentError();
        }
      } finally {
        AbiInternalError.handleError($fieldDisposeFunctionName($disposeArgList), \'${errorMessage}\'); 
      }''';

    finallyBlock += 'free(\$streamValue);free(\$fValue);';

    return '$body try { $setValue } finally { $finallyBlock }';
  }

  String _typeToffiForInterfaceFieldStreamSend(
    TypeInterface tyInterface,
    InterfaceField field,
    String fieldFunctionName,
    String fieldDisposeFunctionName,
  ) {
    var body = '';
    var setValue = '';
    var finallyBlock = '';
    final instanceArg = field.isStatic ? '' : '\$instance';
    var argList = instanceArg;

    final errorMessage = '${tyInterface.ident}.${toCamelCase(field.ident)}';

    if (argList.isNotEmpty) argList += ',';

    argList += '\$streamValue,\$fValue';

    body += ''' 
      final \$streamValue = allocate<AbiStream>();
      final \$fValue = allocate<Pointer<AbiStream>>();''';

    final disposeArgList =
        instanceArg.isEmpty ? '\$fValue.value' : '$instanceArg,\$fValue.value';
    final streamTy = PackageLibrary.fieldStreamSendTy(field);

    setValue += '''
      switch (\$streamState) {
        case AbiStreamSenderState.value:
          \$streamValue.ref.data = (${_typeToffiForFieldBoxed(streamTy, '\$streamData')}).cast();
          break;
        default:
          break;
      }
      \$streamValue.ref.wakeHandle = \$wakePort;
      \$streamValue.ref.wakeObject = DartCObjectInt.newObject(\$wakeObject).cast();
      \$streamValue.ref.wakeCallback = NativeApi.postCObject.cast();
      \$streamValue.ref.state = \$streamState;
      AbiInternalError.handleError($fieldFunctionName($argList), \'${errorMessage}\');
      try {
        final \$stream = \$fValue.value;
        switch (\$stream.ref.state!) {
          case AbiStreamReceiverState.ok:
            break;
          case AbiStreamReceiverState.close:
          case AbiStreamReceiverState.start:
          case AbiStreamReceiverState.pause:
          case AbiStreamReceiverState.resume:
            \$sendPort.send([\$wakeObject, \$stream.ref.state!]);
            break;
          default:
            throw ArgumentError();
        }
      } finally {
        AbiInternalError.handleError($fieldDisposeFunctionName($disposeArgList), \'${errorMessage}\'); 
      }''';

    finallyBlock += 'free(\$streamValue);free(\$fValue);';

    return '$body try { $setValue } finally { $finallyBlock }';
  }

  String _bodyForInterfaceFuture(
    TypeInterface typeInterface,
    InterfaceField field,
    String fieldFunctionName,
    String fieldDisposeFunctionName,
  ) {
    var argList = '';
    var argNames = '';
    final funcName = toCamelCase(field.ident);

    final tyName = field.ty.typeName;

    final getArgs = (TypeTuple tupleTy) {
      var index = 2;
      for (var value in tupleTy.fields) {
        if (value.ty.typeName is TypeStream) {
          if (argNames.isNotEmpty) argNames += ',';
          if (argList.isNotEmpty) argList += ',';
          argNames += '\$wakePortSend,\$wakeObjectSend';
          argList +=
              '\$data[${index.toString()}][0] as int,\$data[${index.toString()}][1] as int';
          break;
        } else {
          final dTy =
              DartTypes.getDartType(value.ty, _packageLibrary, kTypesLib);
          if (argNames.isNotEmpty) argNames += ',';
          argNames += '$dTy ${toCamelCase(value.ident)}';
          if (argList.isNotEmpty) argList += ',';
          argList += '\$data[${index.toString()}]';
        }

        index += 1;
      }
    };

    if (tyName is TypeFunction) {
      getArgs(tyName.args.typeName);
    } else if (tyName is TypeTuple) {
      getArgs(tyName);
    }

    if (PackageLibrary.fieldReturnsStream(field)) {
      if (argNames.isNotEmpty) argNames += ',';
      if (argList.isNotEmpty) argList += ',';
      argNames += '\$wakePort,\$wakeObject';
      argList += '\$data[1][0] as int,\$data[1][1] as int';

      // When sending a stream request, there's no data to dispose of.
      return '''
        case '$funcName':
          ($argNames) {
            ${_typeToffiForInterfaceField(typeInterface, field, fieldFunctionName, '')}
          }($argList);
          break;''';
    }

    return '''
      case '$funcName':
        \$sendPort.send([\$data[1] as int, ($argNames) {
          ${_typeToffiForInterfaceField(typeInterface, field, fieldFunctionName, fieldDisposeFunctionName)}
        }($argList)]);
        break;''';
  }

  String _bodyForInterfaceStream(
    TypeInterface typeInterface,
    InterfaceField field,
    String fieldFunctionName,
    String fieldDisposeFunctionName,
  ) {
    var argList =
        '(\$data[0] as StreamReceiverState).index,\$data[1] as int,\$data[3] as int,\$data[4] as SendPort';
    var argNames =
        'int \$streamState,int \$wakeObject,int \$wakePort,SendPort \$sendPort';
    final funcName = toCamelCase(field.ident);

    return '''
      case '$funcName':
        ($argNames) {
          ${_typeToffiForInterfaceFieldStream(typeInterface, field, fieldFunctionName, fieldDisposeFunctionName)}
        }($argList);
        break;''';
  }

  String _bodyForInterfaceStreamSend(
    TypeInterface typeInterface,
    InterfaceField field,
    String fieldFunctionName,
    String fieldDisposeFunctionName,
  ) {
    final sTy = DartTypes.getDartType(
        PackageLibrary.fieldStreamSendTy(field), _packageLibrary, kTypesLib);
    var argList =
        '(\$data[0] as StreamSenderState).index,\$data[1] as int,\$data[3] as int,\$data[4] as SendPort,\$data[5] as $sTy';
    var argNames =
        'int \$streamState,int \$wakeObject,int \$wakePort,SendPort \$sendPort,$sTy \$streamData';
    final funcName = toCamelCase(field.ident);

    return '''
      case '$funcName':
        ($argNames) {
          ${_typeToffiForInterfaceFieldStreamSend(typeInterface, field, fieldFunctionName, fieldDisposeFunctionName)}
        }($argList);
        break;''';
  }

  String _bodyForInterfaceField(
    TypeInterface typeInterface,
    InterfaceField field,
  ) {
    //!!!! Variables inside the body must start with an underscore.
    var body = '';
    var argList = '';

    final funcName = toCamelCase(field.ident);
    final tyName = field.ty.typeName;

    final getArgs = (TypeTuple tupleTy) {
      for (var value in tupleTy.fields) {
        if (value.ty.typeName is TypeStream) {
          argList +=
              '[_\$toMainSend${toPascalCase(field.ident)}Port!.sendPort.nativePort, _\$handleSend${toPascalCase(field.ident)}]';
          body += ''' 
            while (_\$streams${toPascalCase(field.ident)}.containsKey(_\$handleSend${toPascalCase(field.ident)})) {
              _\$handleSend${toPascalCase(field.ident)} += 1;
            }
            _\$streams${toPascalCase(field.ident)}[_\$handleSend${toPascalCase(field.ident)}] = [${toCamelCase(value.ident)}, null];''';
          break;
        } else {
          argList += '${toCamelCase(value.ident)},';
        }
      }
    };

    String rType;

    if (tyName is TypeFunction) {
      getArgs(tyName.args.typeName);
      rType =
          DartTypes.getDartType(tyName.returnTy, _packageLibrary, kTypesLib);
    } else if (tyName is TypeTuple) {
      getArgs(tyName);
      rType = 'void';
    } else {
      rType = DartTypes.getDartType(field.ty, _packageLibrary, kTypesLib);
    }

    if (PackageLibrary.fieldReturnsStream(field)) {
      body += ''' 
        while (_\$streamControllers${toPascalCase(field.ident)}.containsKey(_\$handle${toPascalCase(field.ident)})) {
          _\$handle${toPascalCase(field.ident)} += 1;
        }
        final \$handle = _\$handle${toPascalCase(field.ident)};
        final \$controller = StreamController<${DartTypes.getDartType(PackageLibrary.fieldStreamReturnTy(field), _packageLibrary, kTypesLib)}>(
        onListen: () {
          _\$toIsolate().then((\$value) => \$value.send([
            '$funcName', 
            [_\$toMain${toPascalCase(field.ident)}Port!.sendPort.nativePort, \$handle], 
            $argList
          ]));
        }, 
        onCancel: () {
          _\$toIsolatePort?.send([
            StreamReceiverState.close,
            \$handle,
            '${toCamelCase(field.ident)}',
            _\$toMain${toPascalCase(field.ident)}Port!.sendPort.nativePort,
            _\$toMain${toPascalCase(field.ident)}Port!.sendPort,
          ]);
        },
        onPause: () {
          _\$toIsolatePort?.send([
            StreamReceiverState.pause,
            \$handle,
            '${toCamelCase(field.ident)}',
            _\$toMain${toPascalCase(field.ident)}Port!.sendPort.nativePort,
            _\$toMain${toPascalCase(field.ident)}Port!.sendPort,
          ]);
        },
        onResume: () {
          _\$toIsolatePort?.send([
            StreamReceiverState.resume,
            \$handle,
            '${toCamelCase(field.ident)}',
            _\$toMain${toPascalCase(field.ident)}Port!.sendPort.nativePort,
            _\$toMain${toPascalCase(field.ident)}Port!.sendPort,
          ]);          
        });
        _\$streamControllers${toPascalCase(field.ident)}[\$handle] = \$controller;
        return \$controller.stream;''';
    } else {
      body += '''
        final \$completer = Completer<$rType>();
        (await _\$toIsolate()).send(['$funcName', \$completer.hashCode, $argList]);
        _\$completers[\$completer.hashCode] = \$completer;
        return \$completer.future;''';
    }

    return '$body';
  }

  String _addStreamSendPortListen(InterfaceField field,
      {required bool isStatic}) {
    final dartStreamType = DartTypes.getDartType(
        PackageLibrary.fieldStreamSendTy(field), _packageLibrary, kTypesLib);

    final tryDisposeBody = isStatic ? '_\$tryDispose();' : '';

    // [0] -> state
    // [1] -> handle
    // [2] -> method name
    // [3] -> native port
    // [4] -> send port
    // [5] -> state data
    return '''
      _\$toMainSend${toPascalCase(field.ident)}Port = ReceivePort();
      _\$toMainSend${toPascalCase(field.ident)}Port!.listen((\$data) {
        if (\$data is int) {
          _\$toIsolatePort?.send([
            StreamSenderState.request,
            \$data,
            '${toCamelCase(field.ident)}',
            _\$toMainSend${toPascalCase(field.ident)}Port!.sendPort.nativePort, 
            _\$toMainSend${toPascalCase(field.ident)}Port!.sendPort,
            0, 
          ]);
        } else if (\$data is List<dynamic>) {
          final \$handleValue = \$data[0] as int;
          if (!_\$streams${toPascalCase(field.ident)}.containsKey(\$handleValue)) return;
          final \$state = StreamReceiverState.values[\$data[1] as int];
          switch (\$state) {
            case StreamReceiverState.start:
              final \$stream = _\$streams${toPascalCase(field.ident)}[\$handleValue]![0] as Stream<$dartStreamType>;
              final \$streamSubscription = \$stream.listen((\$streamData) {
                _\$toIsolatePort?.send([
                  StreamSenderState.value,
                  \$handleValue,
                  '${toCamelCase(field.ident)}',
                  _\$toMainSend${toPascalCase(field.ident)}Port!.sendPort.nativePort, 
                  _\$toMainSend${toPascalCase(field.ident)}Port!.sendPort,
                  \$streamData,
                ]);
              });
              \$streamSubscription.onDone(() {
                _\$streams${toPascalCase(field.ident)}.remove(\$handleValue);
                _\$toIsolatePort?.send([
                  StreamSenderState.done,
                  \$handleValue,
                  '${toCamelCase(field.ident)}',
                  _\$toMainSend${toPascalCase(field.ident)}Port!.sendPort.nativePort, 
                  _\$toMainSend${toPascalCase(field.ident)}Port!.sendPort,
                  0,
                ]);$tryDisposeBody
              });
              _\$streams${toPascalCase(field.ident)}[\$handleValue]![1] = \$streamSubscription;
              break;
            case StreamReceiverState.pause:
              final \$streamSubscription = _\$streams${toPascalCase(field.ident)}[\$handleValue]![1]! as StreamSubscription<$dartStreamType>;
              \$streamSubscription.pause();
              break;
            case StreamReceiverState.resume:
              final \$streamSubscription = _\$streams${toPascalCase(field.ident)}[\$handleValue]![1]! as StreamSubscription<$dartStreamType>;
              \$streamSubscription.resume();
              break;
            case StreamReceiverState.close:
              final \$streams = _\$streams${toPascalCase(field.ident)}.remove(\$handleValue)!;
              final \$streamSubscription = \$streams[1]! as StreamSubscription<$dartStreamType>;
              \$streamSubscription.cancel();$tryDisposeBody
              break;
            case StreamReceiverState.ok:
              break;
            default:
              throw ArgumentError('Invalid stream state `\${\$state}`');
          }
        }
      });''';
  }

  String _addStreamPortListen(InterfaceField field, {required bool isStatic}) {
    final dartStreamType = DartTypes.getDartType(
        PackageLibrary.fieldStreamReturnTy(field), _packageLibrary, kTypesLib);

    final tryDisposeBody = isStatic ? '_\$tryDispose();' : '';

    // [0] -> state
    // [1] -> handle
    // [2] -> method name
    // [3] -> native port
    // [4] -> send port
    // If this port receives a value from the stream, then the isolate must be running somehow.
    // Therefore it's not going to wait for the isolate send port, instead assuming it's not null.
    return '''
      _\$toMain${toPascalCase(field.ident)}Port = ReceivePort();
      _\$toMain${toPascalCase(field.ident)}Port!.listen((\$data) {
        if (\$data is int) {
          _\$toIsolatePort?.send([
            StreamReceiverState.request,
            \$data,
            '${toCamelCase(field.ident)}',
            _\$toMain${toPascalCase(field.ident)}Port!.sendPort.nativePort,
            _\$toMain${toPascalCase(field.ident)}Port!.sendPort,
          ]);
        } else if (\$data is List<dynamic>) {
          final \$handleValue = \$data[0] as int;
          if (!_\$streamControllers${toPascalCase(field.ident)}.containsKey(\$handleValue)) return;
          final \$state = StreamSenderState.values[\$data[1] as int];
          switch (\$state) {
            case StreamSenderState.value:
              _\$streamControllers${toPascalCase(field.ident)}[\$handleValue]!.add(\$data[2] as $dartStreamType);
              break;
            case StreamSenderState.done:
              final \$controller = _\$streamControllers${toPascalCase(field.ident)}.remove(\$handleValue)!;
              \$controller.close();$tryDisposeBody
              break;
            case StreamSenderState.waiting:
              break;
            case StreamSenderState.ok:
              break;
            default:
              throw ArgumentError('Invalid stream state `\${\$state}`');
          }
        }
      });''';
  }

  List<Class> _addInterface(TypeInterface tyInterface) {
    final methods = <Method>[];
    final methodFields = <Field>[];
    final fields = <Field>[];

    final methodName = '_\$Method${tyInterface.ident}';
    final disposeName = '_\$Dispose${tyInterface.ident}';
    final streamName = '_\$Stream${tyInterface.ident}';
    final instanceDisposeName = '_\$InstanceDispose${tyInterface.ident}';
    final instanceCreateName = '_\$InstanceCreate${tyInterface.ident}';

    var methodBody = '';
    var streamBody = '';
    var streamBodyMainPortListen = '';
    var streamBodyDispose = '';
    var streamSendBody = '';

    for (var field_node in tyInterface.fields) {
      var node = field_node.node;

      if (node is InterfaceField && !node.isStatic) {
        // Add the variable for function call.
        final methodFieldName = toPascalCase(node.ident);
        final funcName = methodName;
        final funcType = 'method';

        final nativeNameFunction = '${funcName}${methodFieldName}Native';
        final funcNameFunction = '${funcName}${methodFieldName}Func';

        var fieldDisposeFunctionName = '';

        final fieldFunctionName = '_\$${funcType}${methodFieldName}';
        final ffiFunctionName =
            '${toSnakeCase(_packageLibrary.libraryName)}_${funcType}_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

        methodFields.add(Field((b) => b
          ..name = fieldFunctionName
          ..modifier = FieldModifier.final$
          ..static = true
          ..type = refer(funcNameFunction)
          ..assignment = Code(
              '_\$kLib.lookupFunction<$nativeNameFunction, $funcNameFunction>(\'$ffiFunctionName\')')));

        final setReturnDispose = (TypeName ty) {
          if (PackageLibrary.fieldReturnsStream(node)) {
            fieldDisposeFunctionName = '_\$disposeStream${methodFieldName}';
            final ffiDisposeFunctionName =
               '${toSnakeCase(_packageLibrary.libraryName)}_dispose_stream_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

            final nativeDisposeNameFunction =
                '${disposeName}${methodFieldName}Native';
            final funcDisposeNameFunction =
                '${disposeName}${methodFieldName}Func';

            methodFields.add(Field((b) => b
              ..name = fieldDisposeFunctionName
              ..modifier = FieldModifier.final$
              ..static = true
              ..assignment = Code(
                  '_\$kLib.lookupFunction<$nativeDisposeNameFunction, $funcDisposeNameFunction>(\'$ffiDisposeFunctionName\')')
              ..type = refer(funcDisposeNameFunction)));
          } else if (!_packageLibrary.isTypePrimitive(ty)) {
            fieldDisposeFunctionName = '_\$dispose${methodFieldName}';
            final ffiDisposeFunctionName =
                '${toSnakeCase(_packageLibrary.libraryName)}_dispose_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

            final nativeDisposeNameFunction =
                '${disposeName}${methodFieldName}Native';
            final funcDisposeNameFunction =
                '${disposeName}${methodFieldName}Func';

            methodFields.add(Field((b) => b
              ..name = fieldDisposeFunctionName
              ..modifier = FieldModifier.final$
              ..static = true
              ..assignment = Code(
                  '_\$kLib.lookupFunction<$nativeDisposeNameFunction, $funcDisposeNameFunction>(\'$ffiDisposeFunctionName\')')
              ..type = refer(funcDisposeNameFunction)));
          }
        };

        // Add the interface methods.
        var returnTy;
        var argsTy = <Parameter>[];
        final tyName = node.ty.typeName;

        final getArgs = (TypeTuple tyTuple) => tyTuple.fields
            .map((v) => Parameter((b) => b
              ..name = toCamelCase(v.ident)
              ..type = refer(
                  DartTypes.getDartType(v.ty, _packageLibrary, kTypesLib))))
            .toList();

        if (tyName is TypeFunction) {
          argsTy = getArgs(tyName.args.typeName as TypeTuple);
          setReturnDispose(tyName.returnTy);
        } else if (tyName is TypeTuple) {
          argsTy = getArgs(tyName);
        } else {
          setReturnDispose(node.ty);
        }

        returnTy =
            DartTypes.getDartReturnType(node.ty, _packageLibrary, kTypesLib);

        if (PackageLibrary.fieldSendsStream(node)) {
          methodFields.add(Field((b) => b
            ..name = '_\$streams${toPascalCase(node.ident)}'
            ..modifier = FieldModifier.final$
            ..assignment = Code('HashMap<int, List<dynamic>>()')));

          methodFields.add(Field((b) => b
            ..name = '_\$handleSend${toPascalCase(node.ident)}'
            ..type = refer('int')
            ..assignment = Code('1')));

          fields.add(Field((b) => b
            ..name = '_\$toMainSend${toPascalCase(node.ident)}Port'
            ..type = refer('ReceivePort?')));

          final streamPortListen =
              _addStreamSendPortListen(node, isStatic: false);

          final fieldSenderDisposeFunctionName =
              '_\$disposeStreamSender${methodFieldName}';
          final ffiDisposeFunctionName =
              '${toSnakeCase(_packageLibrary.libraryName)}_dispose_stream_sender_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

          final nativeDisposeNameFunction =
              '${disposeName}${methodFieldName}Native';
          final funcDisposeNameFunction =
              '${disposeName}${methodFieldName}Func';

          methodFields.add(Field((b) => b
            ..name = fieldSenderDisposeFunctionName
            ..modifier = FieldModifier.final$
            ..static = true
            ..assignment = Code(
                '_\$kLib.lookupFunction<$nativeDisposeNameFunction, $funcDisposeNameFunction>(\'$ffiDisposeFunctionName\')')
            ..type = refer(funcDisposeNameFunction)));

          final funcStreamType = 'streamSender';
          final fieldStreamFunctionName =
              '_\$${funcStreamType}${methodFieldName}';

          streamSendBody += _bodyForInterfaceStreamSend(tyInterface, node,
              fieldStreamFunctionName, fieldSenderDisposeFunctionName);
          streamBodyDispose += '''
              _\$toMainSend${toPascalCase(node.ident)}Port?.close();
              _\$toMainSend${toPascalCase(node.ident)}Port = null;''';

          streamBodyMainPortListen += streamPortListen;

          final nativeNameFunction = '${streamName}${methodFieldName}Native';
          final funcNameFunction = '${streamName}${methodFieldName}Func';

          final fieldFunctionName = '_\$streamSender${methodFieldName}';
          final ffiFunctionName =
              '${toSnakeCase(_packageLibrary.libraryName)}_stream_sender_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

          methodFields.add(Field((b) => b
            ..name = fieldFunctionName
            ..modifier = FieldModifier.final$
            ..static = true
            ..type = refer(funcNameFunction)
            ..assignment = Code(
                '_\$kLib.lookupFunction<$nativeNameFunction, $funcNameFunction>(\'$ffiFunctionName\')')));
        }

        // Add stream fields
        if (PackageLibrary.fieldReturnsStream(node)) {
          final dartStreamType = DartTypes.getDartType(
              PackageLibrary.fieldStreamReturnTy(node),
              _packageLibrary,
              kTypesLib);

          methodFields.add(Field((b) => b
            ..name = '_\$streamControllers${toPascalCase(node.ident)}'
            ..modifier = FieldModifier.final$
            ..assignment =
                Code('HashMap<int, StreamController<$dartStreamType>>()')));

          methodFields.add(Field((b) => b
            ..name = '_\$handle${toPascalCase(node.ident)}'
            ..type = refer('int')
            ..assignment = Code('1')));

          fields.add(Field((b) => b
            ..name = '_\$toMain${toPascalCase(node.ident)}Port'
            ..type = refer('ReceivePort?')));

          final streamPortListen = _addStreamPortListen(node, isStatic: false);

          methods.add(Method((b) => b
            ..name = toCamelCase(node.ident)
            ..requiredParameters.addAll(argsTy)
            ..returns = refer(returnTy)
            ..annotations.add(refer('override'))
            ..body = Code(_bodyForInterfaceField(tyInterface, node))));

          final funcStreamType = 'stream';
          final fieldStreamFunctionName =
              '_\$${funcStreamType}${methodFieldName}';

          streamBody += _bodyForInterfaceStream(tyInterface, node,
              fieldStreamFunctionName, fieldDisposeFunctionName);
          streamBodyDispose += '''
            _\$toMain${toPascalCase(node.ident)}Port?.close();
            _\$toMain${toPascalCase(node.ident)}Port = null;''';

          streamBodyMainPortListen += streamPortListen;

          final nativeNameFunction = '${streamName}${methodFieldName}Native';
          final funcNameFunction = '${streamName}${methodFieldName}Func';

          final fieldFunctionName = '_\$stream${methodFieldName}';
          final ffiFunctionName =
              '${toSnakeCase(_packageLibrary.libraryName)}_stream_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

          methodFields.add(Field((b) => b
            ..name = fieldFunctionName
            ..modifier = FieldModifier.final$
            ..static = true
            ..type = refer(funcNameFunction)
            ..assignment = Code(
                '_\$kLib.lookupFunction<$nativeNameFunction, $funcNameFunction>(\'$ffiFunctionName\')')));
        } else {
          methods.add(Method((b) => b
            ..name = toCamelCase(node.ident)
            ..requiredParameters.addAll(argsTy)
            ..returns = refer(returnTy)
            ..modifier = MethodModifier.async
            ..annotations.add(refer('override'))
            ..body = Code(_bodyForInterfaceField(tyInterface, node))));
        }

        methodBody += _bodyForInterfaceFuture(
            tyInterface, node, fieldFunctionName, fieldDisposeFunctionName);
      } else if (node is InterfaceComment) {}
    }

    fields.add(Field((b) => b
      ..name = '_\$toIsolatePort'
      ..type = refer('SendPort?')));

    fields.add(Field((b) => b
      ..name = '_\$toMainPort'
      ..type = refer('ReceivePort?')));

    fields.add(Field((b) => b
      ..name = '_\$isolate'
      ..type = refer('Isolate?')));

    methods.add(Method((b) => b
      ..name = '_\$toIsolate'
      ..modifier = MethodModifier.async
      ..returns = refer('Future<SendPort>')
      ..body = Code(''' 
        _\$toIsolatePort ??= await _\$setIsolate();      
        return _\$toIsolatePort!;''')));

    String runIsolateField;

    runIsolateField =
        '_\$isolate = await Isolate.spawn(_\$runIsolate, _\$toMainPort!.sendPort);';

    final interfaceImplements = <Reference>[];
    final constructors = <Constructor>[];

    var bodyFutureStream = '''
      if (\$data[0] is String) {
        switch(\$data[0]) {
          $methodBody
          default:
            throw ArgumentError('Invalid port args');
        }
      }''';

    var bodyFutureForSet = '';
    if (PackageLibrary.interfaceReturnsValue(tyInterface, isStatic: false)) {
      fields.add(Field((b) => b
        ..name = '_\$completers'
        ..type = refer('HashMap<int, Completer>')
        ..modifier = FieldModifier.final$
        ..assignment = Code('HashMap()')));

      bodyFutureForSet = '''
        else if (\$data is List<dynamic>) {
          if (\$data[0] is int) {
            _\$completers.remove(\$data[0])!.complete(\$data[1]);
          }
        }''';
    }

    methods.add(Method((b) => b
      ..name = '_\$setIsolate'
      ..modifier = MethodModifier.async
      ..returns = refer('Future<SendPort>')
      ..body = Code('''
        final \$completer = Completer<SendPort>();
        _\$toMainPort = ReceivePort();
        _\$toMainPort!.listen((\$data) {
          if (\$data is SendPort) {
            \$completer.complete(\$data);
          }$bodyFutureForSet
        });
        $streamBodyMainPortListen
        $runIsolateField
        return \$completer.future;''')));

    if (PackageLibrary.interfaceReturnsStream(tyInterface, isStatic: false) ||
        PackageLibrary.interfaceSendsStream(tyInterface, isStatic: false)) {
      if (PackageLibrary.interfaceReturnsStream(tyInterface, isStatic: false)) {
        if (bodyFutureStream.isNotEmpty) bodyFutureStream += 'else';
        bodyFutureStream += '''
          if (\$data[0] is StreamReceiverState) {
            switch (\$data[2] as String) {
              $streamBody
              default:
                throw ArgumentError('Invalid port args');  
            }
          }''';
      }

      if (PackageLibrary.interfaceSendsStream(tyInterface, isStatic: false)) {
        if (bodyFutureStream.isNotEmpty) bodyFutureStream += 'else';
        bodyFutureStream += '''
          if (\$data[0] is StreamSenderState) {
            switch (\$data[2] as String) {
              $streamSendBody
              default:
                throw ArgumentError('Invalid port args');  
            }
          }''';
      }
    }

    methods.add(Method((b) => b
      ..name = '_\$runIsolate'
      ..static = true
      ..returns = refer('void')
      ..modifier = MethodModifier.async
      ..requiredParameters.add(Parameter((b) => b
        ..name = '\$sendPort'
        ..type = refer('SendPort')))
      ..body = Code('''
        final \$receivePort = ReceivePort();
        final \$instance = _\$create();
        \$sendPort.send(\$receivePort.sendPort);
        await for (var \$data in \$receivePort) {
          if (\$data is List<dynamic>) {
            $bodyFutureStream
          } else {
            break;
          }
        }
        AbiInternalError.handleError(_\$instanceDispose(\$instance), \'${tyInterface.ident}\');''')));

    fields.add(Field((b) => b
      ..name = '_\$instanceCreate'
      ..type = refer('${instanceCreateName}Func')
      ..assignment = Code(
          '_\$kLib.lookupFunction<${instanceCreateName}Native, ${instanceCreateName}Func>(\'${toSnakeCase(_packageLibrary.libraryName)}_create_${toSnakeCase(tyInterface.ident)}\')')
      ..static = true
      ..modifier = FieldModifier.final$));

    fields.add(Field((b) => b
      ..name = '_\$instanceDispose'
      ..type = refer('${instanceDisposeName}Func')
      ..assignment = Code(
          '_\$kLib.lookupFunction<${instanceDisposeName}Native, ${instanceDisposeName}Func>(\'${toSnakeCase(_packageLibrary.libraryName)}_dispose_${toSnakeCase(tyInterface.ident)}\')')
      ..static = true
      ..modifier = FieldModifier.final$));

    methods.add(Method((b) => b
      ..name = '_\$create'
      ..static = true
      ..returns = refer('Pointer<_${tyInterface.ident}>')
      ..body = Code('''
        final \$result = allocate<Pointer<_${tyInterface.ident}>>();
        try {
          AbiInternalError.handleError(_\$instanceCreate(\$result), \'${tyInterface.ident}\'); 
          return \$result.value;
        } finally {
          free(\$result);
        }''')));

    interfaceImplements
        .add(refer('$kTypesInterfaceLib.${tyInterface.ident}Instance'));

    var disposeBody = '''
      _\$toIsolatePort?.send(false);
      _\$toMainPort?.close();
      _\$isolate?.kill();
      $streamBodyDispose
      _\$toIsolatePort = null;
      _\$toMainPort = null;
      _\$isolate = null;''';

    methods.add(Method((b) => b
      ..name = 'dispose'
      ..annotations.add(refer('override'))
      ..returns = refer('void')
      ..body = Code('$disposeBody')));

    interfaceImplements.add(refer('Disposable'));

    return [
      Class((b) => b
        ..implements.addAll(interfaceImplements)
        ..constructors.addAll(constructors)
        ..name = tyInterface.ident
        ..fields.addAll(fields)
        ..fields.addAll(methodFields)
        ..methods.addAll(methods)),
      Class((b) => b
        ..name = '_${tyInterface.ident}'
        ..extend = refer('Struct'))
    ];
  }

  List<Class> _addInterfaceStatic(TypeInterface tyInterface) {
    final methods = <Method>[];
    final methodFields = <Field>[];
    final fields = <Field>[];

    final staticName = '_\$Static${tyInterface.ident}';
    final disposeName = '_\$Dispose${tyInterface.ident}';
    final streamName = '_\$Stream${tyInterface.ident}';

    var methodBody = '';
    var streamBody = '';
    var streamBodyMainPortListen = '';
    var streamBodyDispose = '';
    var streamSendBody = '';

    final streamCheckEmptyIsolate = <String>[];

    for (var field_node in tyInterface.fields) {
      var node = field_node.node;

      if (node is InterfaceField && node.isStatic) {
        final methodFieldName = toPascalCase(node.ident);
        final funcName = staticName;
        final funcType = 'static';

        final nativeNameFunction = '${funcName}${methodFieldName}Native';
        final funcNameFunction = '${funcName}${methodFieldName}Func';

        var fieldDisposeFunctionName = '';

        final fieldFunctionName = '_\$${funcType}${methodFieldName}';
        final ffiFunctionName =
            '${toSnakeCase(_packageLibrary.libraryName)}_${funcType}_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

        methodFields.add(Field((b) => b
          ..name = fieldFunctionName
          ..modifier = FieldModifier.final$
          ..static = true
          ..type = refer(funcNameFunction)
          ..assignment = Code(
              '_\$kLib.lookupFunction<$nativeNameFunction, $funcNameFunction>(\'$ffiFunctionName\')')));

        final setReturnDispose = (TypeName ty) {
          if (PackageLibrary.fieldReturnsStream(node)) {
            fieldDisposeFunctionName = '_\$disposeStream${methodFieldName}';
            final ffiDisposeFunctionName =
                '${toSnakeCase(_packageLibrary.libraryName)}_dispose_stream_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

            final nativeDisposeNameFunction =
                '${disposeName}${methodFieldName}Native';
            final funcDisposeNameFunction =
                '${disposeName}${methodFieldName}Func';

            methodFields.add(Field((b) => b
              ..name = fieldDisposeFunctionName
              ..modifier = FieldModifier.final$
              ..static = true
              ..assignment = Code(
                  '_\$kLib.lookupFunction<$nativeDisposeNameFunction, $funcDisposeNameFunction>(\'$ffiDisposeFunctionName\')')
              ..type = refer(funcDisposeNameFunction)));
          } else if (!_packageLibrary.isTypePrimitive(ty)) {
            fieldDisposeFunctionName = '_\$dispose${methodFieldName}';
            final ffiDisposeFunctionName =
                '${toSnakeCase(_packageLibrary.libraryName)}_dispose_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

            final nativeDisposeNameFunction =
                '${disposeName}${methodFieldName}Native';
            final funcDisposeNameFunction =
                '${disposeName}${methodFieldName}Func';

            methodFields.add(Field((b) => b
              ..name = fieldDisposeFunctionName
              ..modifier = FieldModifier.final$
              ..static = true
              ..assignment = Code(
                  '_\$kLib.lookupFunction<$nativeDisposeNameFunction, $funcDisposeNameFunction>(\'$ffiDisposeFunctionName\')')
              ..type = refer(funcDisposeNameFunction)));
          }
        };

        // Add the interface methods.
        var returnTy;
        var argsTy = <Parameter>[];
        final tyName = node.ty.typeName;

        final getArgs = (TypeTuple tyTuple) => tyTuple.fields
            .map((v) => Parameter((b) => b
              ..name = toCamelCase(v.ident)
              ..type = refer(
                  DartTypes.getDartType(v.ty, _packageLibrary, kTypesLib))))
            .toList();

        if (tyName is TypeFunction) {
          argsTy = getArgs(tyName.args.typeName as TypeTuple);
          setReturnDispose(tyName.returnTy);
        } else if (tyName is TypeTuple) {
          argsTy = getArgs(tyName);
        } else {
          setReturnDispose(node.ty);
        }

        returnTy =
            DartTypes.getDartReturnType(node.ty, _packageLibrary, kTypesLib);

        if (PackageLibrary.fieldSendsStream(node)) {
          methodFields.add(Field((b) => b
            ..name = '_\$streams${toPascalCase(node.ident)}'
            ..modifier = FieldModifier.final$
            ..assignment = Code('HashMap<int, List<dynamic>>()')));

          streamCheckEmptyIsolate
              .add('_\$streams${toPascalCase(node.ident)}.isEmpty');

          methodFields.add(Field((b) => b
            ..name = '_\$handleSend${toPascalCase(node.ident)}'
            ..type = refer('int')
            ..assignment = Code('1')));

          fields.add(Field((b) => b
            ..name = '_\$toMainSend${toPascalCase(node.ident)}Port'
            ..type = refer('ReceivePort?')));

          final streamPortListen =
              _addStreamSendPortListen(node, isStatic: true);

          final fieldSenderDisposeFunctionName =
              '_\$disposeStreamSender${methodFieldName}';
          final ffiDisposeFunctionName =
              '${toSnakeCase(_packageLibrary.libraryName)}_dispose_stream_sender_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

          final nativeDisposeNameFunction =
              '${disposeName}${methodFieldName}Native';
          final funcDisposeNameFunction =
              '${disposeName}${methodFieldName}Func';

          methodFields.add(Field((b) => b
            ..name = fieldSenderDisposeFunctionName
            ..modifier = FieldModifier.final$
            ..static = true
            ..assignment = Code(
                '_\$kLib.lookupFunction<$nativeDisposeNameFunction, $funcDisposeNameFunction>(\'$ffiDisposeFunctionName\')')
            ..type = refer(funcDisposeNameFunction)));

          final funcStreamType = 'streamSender';
          final fieldStreamFunctionName =
              '_\$${funcStreamType}${methodFieldName}';

          streamSendBody += _bodyForInterfaceStreamSend(tyInterface, node,
              fieldStreamFunctionName, fieldSenderDisposeFunctionName);
          streamBodyDispose += '''
            _\$toMainSend${toPascalCase(node.ident)}Port?.close();
            _\$toMainSend${toPascalCase(node.ident)}Port = null;''';
          streamBodyMainPortListen += streamPortListen;

          final nativeNameFunction = '${streamName}${methodFieldName}Native';
          final funcNameFunction = '${streamName}${methodFieldName}Func';

          final fieldFunctionName = '_\$streamSender${methodFieldName}';
          final ffiFunctionName =
              '${toSnakeCase(_packageLibrary.libraryName)}_stream_sender_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

          methodFields.add(Field((b) => b
            ..name = fieldFunctionName
            ..modifier = FieldModifier.final$
            ..static = true
            ..type = refer(funcNameFunction)
            ..assignment = Code(
                '_\$kLib.lookupFunction<$nativeNameFunction, $funcNameFunction>(\'$ffiFunctionName\')')));
        }

        // Add stream fields
        if (PackageLibrary.fieldReturnsStream(node)) {
          final dartStreamType = DartTypes.getDartType(
              PackageLibrary.fieldStreamReturnTy(node),
              _packageLibrary,
              kTypesLib);

          methodFields.add(Field((b) => b
            ..name = '_\$streamControllers${toPascalCase(node.ident)}'
            ..modifier = FieldModifier.final$
            ..assignment =
                Code('HashMap<int, StreamController<$dartStreamType>>()')));

          streamCheckEmptyIsolate
              .add('_\$streamControllers${toPascalCase(node.ident)}.isEmpty');

          methodFields.add(Field((b) => b
            ..name = '_\$handle${toPascalCase(node.ident)}'
            ..type = refer('int')
            ..assignment = Code('1')));

          fields.add(Field((b) => b
            ..name = '_\$toMain${toPascalCase(node.ident)}Port'
            ..type = refer('ReceivePort?')));

          final streamPortListen = _addStreamPortListen(node, isStatic: true);

          methods.add(Method((b) => b
            ..name = toCamelCase(node.ident)
            ..requiredParameters.addAll(argsTy)
            ..returns = refer(returnTy)
            ..annotations.add(refer('override'))
            ..body = Code(_bodyForInterfaceField(tyInterface, node))));

          final funcStreamType = 'stream';
          final fieldStreamFunctionName =
              '_\$${funcStreamType}${methodFieldName}';

          streamBody += _bodyForInterfaceStream(tyInterface, node,
              fieldStreamFunctionName, fieldDisposeFunctionName);
          streamBodyDispose += '''
            _\$toMain${toPascalCase(node.ident)}Port?.close();
            _\$toMain${toPascalCase(node.ident)}Port = null;''';
          streamBodyMainPortListen += streamPortListen;

          final nativeNameFunction = '${streamName}${methodFieldName}Native';
          final funcNameFunction = '${streamName}${methodFieldName}Func';

          final fieldFunctionName = '_\$stream${methodFieldName}';
          final ffiFunctionName =
              '${toSnakeCase(_packageLibrary.libraryName)}_stream_${toSnakeCase(tyInterface.ident)}_${toSnakeCase(node.ident)}';

          methodFields.add(Field((b) => b
            ..name = fieldFunctionName
            ..modifier = FieldModifier.final$
            ..static = true
            ..type = refer(funcNameFunction)
            ..assignment = Code(
                '_\$kLib.lookupFunction<$nativeNameFunction, $funcNameFunction>(\'$ffiFunctionName\')')));
        } else {
          methods.add(Method((b) => b
            ..name = toCamelCase(node.ident)
            ..requiredParameters.addAll(argsTy)
            ..returns = refer(returnTy)
            ..modifier = MethodModifier.async
            ..annotations.add(refer('override'))
            ..body = Code(_bodyForInterfaceField(tyInterface, node))));
        }

        methodBody += _bodyForInterfaceFuture(
            tyInterface, node, fieldFunctionName, fieldDisposeFunctionName);
      } else if (node is InterfaceComment) {}
    }

    fields.add(Field((b) => b
      ..name = '_\$toIsolatePort'
      ..type = refer('SendPort?')));

    fields.add(Field((b) => b
      ..name = '_\$toMainPort'
      ..type = refer('ReceivePort?')));

    fields.add(Field((b) => b
      ..name = '_\$isolate'
      ..type = refer('Isolate?')));

    methods.add(Method((b) => b
      ..name = '_\$toIsolate'
      ..modifier = MethodModifier.async
      ..returns = refer('Future<SendPort>')
      ..body = Code(''' 
        _\$toIsolatePort ??= await _\$setIsolate();      
        return _\$toIsolatePort!;''')));

    final runIsolateField =
        '_\$isolate = await Isolate.spawn(_\$runIsolate, _\$toMainPort!.sendPort);';

    var bodyFutureStream = '''
      if (\$data[0] is String) {
        switch(\$data[0]) {
          $methodBody
          default:
            throw ArgumentError('Invalid port args');
        }
      }''';

    var bodyFutureForSet = '';
    if (PackageLibrary.interfaceReturnsValue(tyInterface, isStatic: true)) {
      fields.add(Field((b) => b
        ..name = '_\$completers'
        ..type = refer('HashMap<int, Completer>')
        ..modifier = FieldModifier.final$
        ..assignment = Code('HashMap()')));

      streamCheckEmptyIsolate.add('_\$completers.isEmpty');

      bodyFutureForSet = '''
        else if (\$data is List<dynamic>) {
          if (\$data[0] is int) {
            _\$completers.remove(\$data[0])!.complete(\$data[1]);
          }
           _\$tryDispose();
        }''';
    }

    final interfaceImplements = <Reference>[];
    final constructors = <Constructor>[];

    methods.add(Method((b) => b
      ..name = '_\$setIsolate'
      ..modifier = MethodModifier.async
      ..returns = refer('Future<SendPort>')
      ..body = Code('''
        final \$completer = Completer<SendPort>();
        _\$toMainPort = ReceivePort();
        _\$toMainPort!.listen((\$data) {
          if (\$data is SendPort) {
            \$completer.complete(\$data);
          } $bodyFutureForSet 
        });
        $streamBodyMainPortListen
        $runIsolateField
        return \$completer.future;''')));

    interfaceImplements
        .add(refer('$kTypesInterfaceLib.${tyInterface.ident}Static'));

    if (PackageLibrary.interfaceReturnsStream(tyInterface, isStatic: true) ||
        PackageLibrary.interfaceSendsStream(tyInterface, isStatic: true)) {
      if (PackageLibrary.interfaceReturnsStream(tyInterface, isStatic: true)) {
        if (bodyFutureStream.isNotEmpty) bodyFutureStream += 'else';
        bodyFutureStream += '''
          if (\$data[0] is StreamReceiverState) {
            switch (\$data[2] as String) {
              $streamBody
              default:
                throw ArgumentError('Invalid port args');  
            }
          }''';
      }

      if (PackageLibrary.interfaceSendsStream(tyInterface, isStatic: true)) {
        if (bodyFutureStream.isNotEmpty) bodyFutureStream += 'else';
        bodyFutureStream += '''
          if (\$data[0] is StreamSenderState) {
            switch (\$data[2] as String) {
              $streamSendBody
              default:
                throw ArgumentError('Invalid port args');  
            }
          }''';
      }
    }

    methods.add(Method((b) => b
      ..name = '_\$runIsolate'
      ..static = true
      ..modifier = MethodModifier.async
      ..returns = refer('void')
      ..requiredParameters.add(Parameter((b) => b
        ..name = '\$sendPort'
        ..type = refer('SendPort')))
      ..body = Code('''
        final \$receivePort = ReceivePort();
        \$sendPort.send(\$receivePort.sendPort);
        await for (var \$data in \$receivePort) {
          if (\$data is List<dynamic>) { 
            $bodyFutureStream
          } else {
            break;
          }
        }''')));

    final streamCheckField = streamCheckEmptyIsolate.fold(
        '', (String p, String value) => p.isEmpty ? value : '$p && $value');

    assert(streamCheckEmptyIsolate.isEmpty);

    final closeIsolateField = '''
      if ($streamCheckField) {
        _\$instance = null;
        _\$isolate?.kill();
        _\$toMainPort?.close();
        $streamBodyDispose
        _\$toIsolatePort = null;
        _\$toMainPort = null;
        _\$isolate = null;
      }''';

    methods.add(Method((b) => b
      ..name = '_\$tryDispose'
      ..returns = refer('void')
      ..body = Code(closeIsolateField)));

    methods.add(Method((b) => b
      ..name = 'dispose'
      ..annotations.add(refer('override'))
      ..returns = refer('void')
      ..body = Code('')));

    interfaceImplements.add(refer('Disposable'));

    methods.add(Method((b) => b
      ..static = true
      ..name = 'asStatic'
      ..returns = refer('${tyInterface.ident}Static')
      ..body = Code('''
        _\$instance ??= ${tyInterface.ident}Static._\$${tyInterface.ident}Static();
        return _\$instance!;''')));

    constructors
        .add(Constructor((b) => b..name = '_\$${tyInterface.ident}Static'));
    fields.add(Field((b) => b
      ..name = '_\$instance'
      ..static = true
      ..type = refer('${tyInterface.ident}Static?')));

    return [
      Class((b) => b
        ..implements.addAll(interfaceImplements)
        ..constructors.addAll(constructors)
        ..name = '${tyInterface.ident}Static'
        ..fields.addAll(fields)
        ..fields.addAll(methodFields)
        ..methods.addAll(methods)),
    ];
  }

  // String get _tryServerRelativePath {
  //   try {
  //     final result = Module.getServerField<ids_types.NatString>(
  //             _module.targetServer, 'relative_path')
  //         .value;

  //     // TODO Fix internal ffi path
  //     if (!result.endsWith('/')) {
  //       return result + '/';
  //     }

  //     return result;
  //   } catch (e) {
  //     return 'target/';
  //   }
  // }

  List<Code> _addInterfaceTypedef(TypeInterface tyInterface) {
    final typedefs = <Code>[];

    final staticName = '_\$Static${tyInterface.ident}';
    final disposeName = '_\$Dispose${tyInterface.ident}';
    final methodName = '_\$Method${tyInterface.ident}';
    final streamName = '_\$Stream${tyInterface.ident}';
    final instanceArgName = 'Pointer<_${tyInterface.ident}>';

    if (PackageLibrary.interfaceHasNonStaticField(tyInterface)) {
      typedefs.add(Code('''
        typedef _\$InstanceCreate${tyInterface.ident}Native = Int64 Function(Pointer<Pointer<_${tyInterface.ident}>>);
        typedef _\$InstanceCreate${tyInterface.ident}Func = int Function(Pointer<Pointer<_${tyInterface.ident}>>);
        typedef _\$InstanceDispose${tyInterface.ident}Native = Int64 Function($instanceArgName);
        typedef _\$InstanceDispose${tyInterface.ident}Func = int Function($instanceArgName);'''));
    }

    for (var field_node in tyInterface.fields) {
      final node = field_node.node;

      if (node is InterfaceField) {
        final methodFieldName = toPascalCase(node.ident);
        final instanceFieldArgName = node.isStatic ? '' : instanceArgName;
        final funcName = node.isStatic ? staticName : methodName;
        var argsNative = instanceFieldArgName;
        var argsFunc = instanceFieldArgName;

        final tyName = node.ty.typeName;

        final getArgs = (TypeTuple tupleTy) {
          for (var value in tupleTy.fields) {
            if (argsNative.isNotEmpty) argsNative += ',';
            if (argsFunc.isNotEmpty) argsFunc += ',';
            argsNative += _getffiTypeForInterface(value.ty);
            argsFunc += _getDartTypeForffi(value.ty);
          }
        };

        final getRet = (TypeName ty) {
          final retTyy = _getffiTypeForInterface(ty);

          final retTyName =
              ty.typeName is TypeStream ? retTyy : 'Pointer<$retTyy>';

          if (argsNative.isNotEmpty) argsNative += ',';
          if (argsFunc.isNotEmpty) argsFunc += ',';
          argsNative += retTyName;
          argsFunc += retTyName;

          // Add the dispose typedef if the returned type is supposed to be freed.
          if (!_packageLibrary.isTypePrimitive(ty)) {
            var argDispose =
                instanceFieldArgName.isNotEmpty ? '$instanceFieldArgName,' : '';
            argDispose += _getffiTypeForInterface(ty);
            typedefs.add(Code('''
              typedef ${disposeName}${methodFieldName}Native = Int64 Function($argDispose);
              typedef ${disposeName}${methodFieldName}Func = int Function($argDispose);'''));
          }
        };

        if (tyName is TypeFunction) {
          getArgs(tyName.args.typeName as TypeTuple);
          getRet(tyName.returnTy);
        } else if (tyName is TypeTuple) {
          getArgs(tyName);
        } else {
          getRet(node.ty);
        }

        typedefs.add(Code('''
          typedef ${funcName}${methodFieldName}Native = Int64 Function($argsNative);
          typedef ${funcName}${methodFieldName}Func = int Function($argsFunc);'''));

        // stream typedef
        if (PackageLibrary.fieldReturnsStream(node) ||
            PackageLibrary.fieldSendsStream(node)) {
          var argsNative =
              instanceFieldArgName.isNotEmpty ? '$instanceFieldArgName,' : '';
          argsNative += 'Pointer<AbiStream>,Pointer<Pointer<AbiStream>>';
          typedefs.add(Code(''' 
            typedef ${streamName}${methodFieldName}Native = Int64 Function($argsNative);
            typedef ${streamName}${methodFieldName}Func = int Function($argsNative);'''));
        }
      }
    }

    return typedefs;
  }

  _FFIDartTypes(PackageLibrary packageLibrary)
      : _packageLibrary = packageLibrary {
    final enums = <Code>[];
    final structs = <Class>[];
    final structExts = <Code>[];
    final typeLists = <Class>[];
    final interfaces = <Class>[];
    final interfaceTypedefs = <Code>[];

    for (var type_node in packageLibrary.idlNodes) {
      final dynamic node = type_node.node;

      if (node is Comment) {
      } else if (node is StructComment) {
      } else if (node is EnumComment) {
      } else if (node is ConstComment) {
      } else if (node is TypeStruct) {
        structs.add(_addStruct(node));
      } else if (node is TypeEnum) {
      } else if (node is TypeConst) {
      } else if (node is TypeListComment) {
      } else if (node is TypeList) {
        typeLists.add(_addTypeList(node));
      } else if (node is TypeInterface) {
        if (PackageLibrary.interfaceHasNonStaticField(node)) {
          interfaces.addAll(_addInterface(node));
        }
        if (PackageLibrary.interfaceHasStaticField(node)) {
          interfaces.addAll(_addInterfaceStatic(node));
        }
        interfaceTypedefs.addAll(_addInterfaceTypedef(node));
      }
    }

    final fields = <Field>[];

    if (packageLibrary.idlNodes.any((a) => a.node is TypeInterface)) {
      fields.add(
        Field((b) => b
          ..name = '_\$kLib'
          ..modifier = FieldModifier.final$
          ..type = refer('DynamicLibrary')
          ..assignment = Code(
            '''openLibrary('${_packageLibrary.libraryName}', '')''', // TODO See when a relative path for the lib files is necessary.
          )),
      );
    }

    final library = Library((b) => b
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = '$kTypesInterfaceLib'
        ..url = '$kTypesInterfaceLib.dart'))
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = '$kTypesLib'
        ..url = '$kTypesLib.dart'))
      ..directives.add(Directive.import('dart:ffi'))
      ..directives.add(Directive.import('dart:async'))
      ..directives.add(Directive.import('dart:typed_data'))
      ..directives.add(Directive.import('dart:isolate'))
      ..directives.add(Directive.import('dart:collection'))
      ..directives
          .add(Directive.import('package:idl_internal/idl_internal.dart'))
      ..body.addAll(fields)
      ..body.addAll(typeLists)
      ..body.addAll(enums)
      ..body.addAll(interfaces)
      ..body.addAll(structs)
      ..body.addAll(structExts)
      ..body.addAll(interfaceTypedefs));

    _library = library;
  }

  late Library _library;
  final PackageLibrary _packageLibrary;

  @override
  String toString() {
    final emitter = DartEmitter();
    return DartFormatter().format('${_library.accept(emitter)}').trim();
  }
}

class _FFIInterfaceConstructors {
  Class _addInterface(TypeInterface tyInterface) {
    final methods = <Method>[];

    if (PackageLibrary.interfaceHasNonStaticField(tyInterface)) {
      methods.add(Method((b) => b
        ..name = 'createInstance'
        ..lambda = true
        ..static = true
        ..returns = refer('$kTypesInterfaceLib.${tyInterface.ident}Instance')
        ..body = Code('idl_ffi.${tyInterface.ident}()')));
    }

    if (PackageLibrary.interfaceHasStaticField(tyInterface)) {
      methods.add(Method((b) => b
        ..name = 'createInstanceStatic'
        ..lambda = true
        ..static = true
        ..returns = refer('$kTypesInterfaceLib.${tyInterface.ident}Static')
        ..body = Code('idl_ffi.${tyInterface.ident}Static.asStatic()')));
    }

    return Class((b) => b
      ..name = '${tyInterface.ident}Constructor'
      ..methods.addAll(methods));
  }

  _FFIInterfaceConstructors(PackageLibrary packageLibrary)
      : _packageLibrary = packageLibrary {
    final _interfaces = <Class>[];

    for (var type_node in packageLibrary.idlNodes) {
      final dynamic node = type_node.node;

      if (node is TypeInterface) {
        _interfaces.add(_addInterface(node));
      }
    }

    final library = Library((b) => b
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = 'idl_ffi'
        ..url = 'idl_ffi.dart'))
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = '$kTypesInterfaceLib'
        ..url = '$kTypesInterfaceLib.dart'))
      ..body.addAll(_interfaces));

    _library = library;
  }

  late Library _library;
  final PackageLibrary _packageLibrary;

  @override
  String toString() {
    final emitter = DartEmitter();

    return DartFormatter().format('${_library.accept(emitter)}').trim();
  }
}

class FFILayer implements DartIdlLayer {
  @override
  LayerItem generate(
    Module module,
    bool endPoint,
    List<DartIdlLayer> subLayers,
  ) {
    if (subLayers.isNotEmpty) {
      throw ArgumentError('FFI layer cannot have sublayers');
    }

    final result = createItemsForLayer(
      module,
      _layer.ident,
      endPoint,
      module.libraries.map((packageLibrary) {
        final dartffiTypes = _FFIDartTypes(packageLibrary);
        final dartffiInterfaceConstructors =
            _FFIInterfaceConstructors(packageLibrary);
        return LayerItemCreation(
            interfaceConstructors: dartffiInterfaceConstructors.toString(),
            layerTypes: dartffiTypes.toString(),
            packageLibrary: packageLibrary);
      }).toList(),
    );

    return LayerItem(
        storageItems: result, pubspec: Pubspec(module.packageName));
  }

  @override
  ids_types.Layer get layer => _layer;

  final ids_types.Layer _layer;

  FFILayer(ids_types.Layer layer) : _layer = layer;
}
