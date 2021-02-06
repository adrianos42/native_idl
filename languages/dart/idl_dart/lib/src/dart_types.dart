import 'dart_layer.dart';
import 'module.dart';
import 'string_pos.dart';
import 'json/idl_types_json.dart';
import 'json/idl_json.dart';
import 'dart_spec.dart';
import 'package:code_builder/code_builder.dart';
import 'package:dart_style/dart_style.dart';

// Types used by the client
const kTypesLib = 'idl_types';
// Interface used by almost all layers
const kTypesInterfaceLib = 'idl_types_interface';
// Constructors generated by the endpoint layer
const kInterfaceConstructor = 'idl_interface_constructor';

class DartTypes {
  Class _addStruct(TypeStruct tyStruct) {
    final fields = <Field>[];
    final consParameters = <Parameter>[];
    var cmpBody = '';

    for (var field_node in tyStruct.fields) {
      final node = field_node.node;

      if (node is StructField) {
        final dName = toCamelCase(node.ident);
        consParameters.add(Parameter((b) => b
          ..name = dName
          ..toThis = true));
        fields.add(Field((b) => b
          ..name = dName
          ..modifier = FieldModifier.final$
          ..type = refer(getDartType(node.ty, _module))));

        // create comparison body
        if (cmpBody.isNotEmpty) cmpBody += '&&';

        cmpBody += 'other.${dName} == ${dName}';
      } else if (node is Comment) {}
    }

    return Class((b) => b
      ..name = tyStruct.ident
      ..fields.addAll(fields)
      ..methods.add(Method((b) => b
        ..name = 'operator =='
        ..annotations.add(refer('override'))
        ..lambda = true
        ..requiredParameters.add(Parameter((b) => b
          ..name = 'other'
          ..type = refer('dynamic')))
        ..returns = refer('bool')
        ..body = Code(cmpBody)))
      ..constructors.add(Constructor((b) => b
        ..constant = true
        ..requiredParameters.addAll(consParameters))));
  }

  Class _addInterface(TypeInterface tyInterface,
      {String interfaceLayerReferenceName = kInterfaceConstructor,
      String nName = '',
      bool = false}) {
    final methods = <Method>[];

    for (var field_node in tyInterface.fields) {
      final node = field_node.node;

      if (node is InterfaceField) {
        String returnTy;
        var argsTy = <Parameter>[];
        var argsName = '';
        final tyName = node.ty.typeName;

        final getArgs = (TypeTuple tyTuple) => tyTuple.fields
            .map((v) => Parameter((b) => b
              ..name = toCamelCase(v.ident)
              ..type = refer(getDartType(v.ty, _module, nName))))
            .toList();

        final getArgsName =
            (TypeTuple tyTuple) => tyTuple.fields.fold('', (String p, value) {
                  if (p.isNotEmpty) p += ',';
                  return p + toCamelCase(value.ident);
                });

        if (tyName is TypeFunction) {
          final tyTuple = tyName.args.typeName as TypeTuple;
          argsTy = getArgs(tyTuple);
          argsName = getArgsName(tyTuple);
        } else if (tyName is TypeTuple) {
          argsName = getArgsName(tyName);
          argsTy = getArgs(tyName);
        }

        returnTy = DartTypes.getDartReturnType(node.ty, _module);

        final method = Method((b) => b
          ..name = toCamelCase(node.ident)
          ..static = node.isStatic
          ..lambda = true
          ..requiredParameters.addAll(argsTy)
          ..body = Code('''
            ${node.isStatic ? '$interfaceLayerReferenceName.${tyInterface.ident}Constructor.createInstanceStatic()' : '_instance!'}.${toCamelCase(node.ident)}($argsName)''')
          ..returns = refer(returnTy));

        methods.add(method);
      } else if (node is Comment) {}
    }

    final typesInterfaceLib = kTypesInterfaceLib;

    final fields = <Field>[];
    if (Module.interfaceHasNonStaticField(tyInterface)) {
      fields.add(Field((b) => b
        ..name = '_instance'
        ..assignment = Code(
            '$interfaceLayerReferenceName.${tyInterface.ident}Constructor.createInstance()')
        ..type = refer('$typesInterfaceLib.${tyInterface.ident}Instance?')));
    }

    final interfaceImplements = <Reference>[];

    methods.add(Method((b) => b
      ..name = 'dispose'
      ..annotations.add(refer('override'))
      ..body = Code('''
        (_instance! as Disposable).dispose();
        _instance = null;''')));

    interfaceImplements.add(refer('Disposable'));

    return Class((b) => b
      ..implements.addAll(interfaceImplements)
      ..name = '${tyInterface.ident}'
      ..fields.addAll(fields)
      ..methods.addAll(methods));
  }

  Class _addConst(TypeConst tyConst) {
    final constFields = <Field>[];

    for (var cNode in tyConst.fields) {
      final node = cNode.node;
      var dType;
      var assignment;

      if (node is ConstField) {
        switch (tyConst.constType) {
          case ConstTypes.natInt:
            dType = 'int';
            assignment = node.value;
            break;
          case ConstTypes.natFloat:
            assignment = node.value;
            dType = 'double';
            break;
          case ConstTypes.natString:
            assignment = '\'${node.value}\'';
            dType = 'String';
            break;
        }

        constFields.add(Field((b) => b
          ..modifier = FieldModifier.final$
          ..name = toCamelCase(node.ident)
          ..assignment = Code(assignment)
          ..static = true
          ..type = refer(dType)));
      }
    }

    return Class((b) => b
      ..name = tyConst.ident
      ..fields.addAll(constFields));
  }

  Class _addTypeList(TypeList tyList) {
    final enumFields = <Field>[];
    final factFields = <Constructor>[];
    var enumFieldIndex = 0;

    for (var tyNode in tyList.fields) {
      final node = tyNode.node;

      if (node is TypeListField) {
        factFields.add(Constructor((b) => b
          ..name = 'as${node.ident}'
          ..lambda = true
          ..factory = true
          ..body = Code(
              '${tyList.ident}._(value, ${tyList.ident}.${toCamelCase(node.ident)})')
          ..requiredParameters.add(Parameter((b) => b
            ..name = 'value'
            ..type = refer(getDartType(node.ty, _module))))));
        enumFields.add(Field((b) => b
          ..static = true
          ..name = toCamelCase(node.ident)
          ..type = refer('int')
          ..modifier = FieldModifier.constant
          ..assignment = Code(enumFieldIndex.toString())));
        enumFieldIndex += 1;
      } else if (node is TypeListComment) {}
    }

    return Class((b) => b
      ..name = tyList.ident
      ..constructors.addAll(factFields)
      ..constructors.add(Constructor((b) => b
        ..name = '_'
        ..constant = true
        ..requiredParameters.add(Parameter((b) => b
          ..toThis = true
          ..name = '_value'))
        ..requiredParameters.add(Parameter((b) => b
          ..toThis = true
          ..name = '_variant'))))
      ..fields.addAll(enumFields)
      ..methods.add(Method((b) => b
        ..name = 'value'
        ..type = MethodType.getter
        ..lambda = true
        ..returns = refer('dynamic')
        ..body = Code('_value')))
      ..fields.add(Field((b) => b
        ..type = refer('dynamic')
        ..name = '_value'
        ..modifier = FieldModifier.final$))
      ..methods.add(Method((b) => b
        ..name = 'variant'
        ..type = MethodType.getter
        ..lambda = true
        ..returns = refer('int')
        ..body = Code('_variant')))
      ..fields.add(Field((b) => b
        ..type = refer('int')
        ..name = '_variant'
        ..modifier = FieldModifier.final$))
      ..methods.add(Method((b) => b
        ..name = 'operator =='
        ..requiredParameters.add(Parameter((b) => b
          ..name = 'other'
          ..type = refer('dynamic')))
        ..returns = refer('bool')
        ..lambda = true
        ..body = Code(''' 
          variant == other.variant && 
          value == other.value'''))));
  }

  Code _addEnum(TypeEnum tyEnum) {
    final fields = [];

    for (var field_node in tyEnum.fields) {
      final node = field_node.node;

      if (node is EnumField) {
        fields.add(toCamelCase(node.ident));
      }
    }

    return Code(fields.fold(
            'enum ${tyEnum.ident} {', (String p, value) => p + value + ',') +
        '}\n\n');
  }

  DartTypes._();

  factory DartTypes.generate(Module module) {
    final dartTypes = DartTypes._();
    dartTypes._module = module;

    final _enums = <Code>[];
    final _structs = <Class>[];
    final _consts = <Class>[];
    final _typeLists = <Class>[];
    final _interfaces = <Class>[];

    for (var type_node in module.request.idlNodes) {
      final dynamic node = type_node.node;

      if (node is Comment) {
      } else if (node is StructComment) {
      } else if (node is EnumComment) {
      } else if (node is ConstComment) {
      } else if (node is TypeStruct) {
        _structs.add(dartTypes._addStruct(node));
      } else if (node is TypeEnum) {
        _enums.add(dartTypes._addEnum(node));
      } else if (node is TypeConst) {
        _consts.add(dartTypes._addConst(node));
      } else if (node is TypeListComment) {
      } else if (node is TypeList) {
        _typeLists.add(dartTypes._addTypeList(node));
      } else if (node is TypeInterface) {
        _interfaces.add(dartTypes._addInterface(node));
      }
    }

    final typesInterfaceLib = kTypesInterfaceLib;

    final library = Library((b) => b
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = '$typesInterfaceLib'
        ..url = '$typesInterfaceLib.dart'))
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = '$kInterfaceConstructor'
        ..url = '$kInterfaceConstructor.dart'))
      ..directives.add(Directive.import('dart:typed_data'))
      ..directives.add(Directive.import('dart:collection'))
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..url = 'package:idl_internal/idl_internal.dart'))
      ..body.addAll(_consts)
      ..body.addAll(_enums)
      ..body.addAll(_typeLists)
      ..body.addAll(_interfaces)
      ..body.addAll(_structs));

    dartTypes._library = library;

    return dartTypes;
  }

  factory DartTypes.generateInterfaceOnlyLayer(
      Module module, String layerReferenceName,
      [bool = false]) {
    final dartTypes = DartTypes._();
    dartTypes._module = module;

    final _interfaces = <Class>[];
    final interfaceLayerReferenceName =
        'idl_${layerReferenceName}_interface_constructor';

    for (var type_node in module.request.idlNodes) {
      final dynamic node = type_node.node;

      if (node is TypeInterface) {
        _interfaces.add(dartTypes._addInterface(
          node,
          interfaceLayerReferenceName: interfaceLayerReferenceName,
          nName: kTypesLib,
        ));
      }
    }

    final typesInterfaceLib = kTypesInterfaceLib;

    final library = Library((b) => b
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = '$typesInterfaceLib'
        ..url = '$typesInterfaceLib.dart'))
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = '$kTypesLib'
        ..url = '$kTypesLib.dart'))
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = '$interfaceLayerReferenceName'
        ..url = '$interfaceLayerReferenceName.dart'))
      ..directives.add(Directive.import('dart:typed_data'))
      ..directives.add(Directive.import('dart:collection'))
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..url = 'package:idl_internal/idl_internal.dart'))
      ..body.addAll(_interfaces));

    dartTypes._library = library;

    return dartTypes;
  }

  late Library _library;
  late Module _module;

  static TypeName resolveTypeWithConst(TypeName tyName, Module module) {
    final ty = tyName.typeName;
    if (ty is ConstTypeName) {
      final tyConst = module.findTypeConst(ty);

      switch (tyConst.constType) {
        case ConstTypes.natInt:
          return TypeName(typeName: Types.natInt);
        case ConstTypes.natFloat:
          return TypeName(typeName: Types.natFloat);
        case ConstTypes.natString:
          return TypeName(typeName: Types.natString);
      }
    }
    return tyName;
  }

  static String getDartReturnType(TypeName ty, Module module,
      [String nName = '']) {
    if (ty.typeName is TypeTuple) {
      return 'Future<void>';
    } else if (ty.typeName is TypeFunction) {
      final rTy = DartTypes.getDartType(ty.typeName.returnTy, module, nName);
      return ty.typeName.returnTy.typeName is TypeStream ? rTy : 'Future<$rTy>';
    } else {
      final rTy = DartTypes.getDartType(ty, module, nName);
      return ty.typeName is TypeStream ? rTy : 'Future<$rTy>';
    }
  }

  static String getDartType(TypeName ty, Module module, [String nName = '']) {
    final typeName = resolveTypeWithConst(ty, module).typeName;

    var wName = '';

    if (nName.isNotEmpty) {
      wName = '$nName.';
    }

    if (typeName is Types) {
      switch (typeName) {
        case Types.natBool:
          return 'bool';
        case Types.natBytes:
          return 'Uint8List';
        case Types.natFloat:
          return 'double';
        case Types.natInt:
          return 'int';
        case Types.natNone:
          return 'Null';
        case Types.natString:
          return 'String';
      }
    } else if (typeName is TypeArray) {
      return 'List<${getDartType(typeName.ty, module, nName)}>';
    } else if (typeName is TypeMap) {
      return 'Map<${getDartType(typeName.indexTy, module, nName)}, ${getDartType(typeName.mapTy, module, nName)}>';
    } else if (typeName is TypePair) {
      return 'Pair<${getDartType(typeName.firstTy, module, nName)}, ${getDartType(typeName.secondTy, module, nName)}>';
    } else if (typeName is TypeOption) {
      return '${getDartType(typeName.someTy, module, nName)}?';
    } else if (typeName is TypeResult) {
      return 'Result<${getDartType(typeName.okTy, module, nName)}, ${getDartType(typeName.errTy, module, nName)}>';
    } else if (typeName is TypeStream) {
      return 'Stream<${getDartType(typeName.sTy, module, nName)}>';
    } else if (typeName is EnumTypeName) {
      return '$wName${typeName.enumTypeName}';
    } else if (typeName is ListTypeName) {
      return '$wName${typeName.listTypeName}';
    } else if (typeName is StructTypeName) {
      return '$wName${typeName.structTypeName}';
    }

    throw ArgumentError();
  }

  @override
  String toString() {
    final emitter = DartEmitter();
    return DartFormatter().format('${_library.accept(emitter)}').trim();
  }
}

class DartTypesInterface {
  List<Class> _addInterface(TypeInterface tyInterface) {
    final interfaces = <Class>[];

    final staticMethods = <Method>[];
    final methods = <Method>[];

    for (var field_node in tyInterface.fields) {
      final node = field_node.node;

      if (node is InterfaceField) {
        String returnTy;
        var argsTy = <Parameter>[];
        final tyName = node.ty.typeName;

        final getArgs = (TypeTuple tyTuple) => tyTuple.fields
            .map((v) => Parameter((b) => b
              ..name = toCamelCase(v.ident)
              ..type = refer(DartTypes.getDartType(v.ty, _module, kTypesLib))))
            .toList();

        if (tyName is TypeFunction) {
          argsTy = getArgs(tyName.args.typeName as TypeTuple);
        } else if (tyName is TypeTuple) {
          argsTy = getArgs(tyName);
        }

        returnTy = DartTypes.getDartReturnType(node.ty, _module, kTypesLib);

        final method = Method((b) => b
          ..name = toCamelCase(node.ident)
          ..requiredParameters.addAll(argsTy)
          ..returns = refer(returnTy));

        if (node.isStatic) {
          staticMethods.add(method);
        } else {
          methods.add(method);
        }
      } else if (node is Comment) {}
    }

    if (staticMethods.isNotEmpty) {
      interfaces.add(Class((b) => b
        ..name = '${tyInterface.ident}Static'
        ..abstract = true
        ..methods.addAll(staticMethods)));
    }

    if (methods.isNotEmpty) {
      interfaces.add(Class((b) => b
        ..name = '${tyInterface.ident}Instance'
        ..abstract = true
        ..methods.addAll(methods)));
    }

    return interfaces;
  }

  DartTypesInterface._();

  factory DartTypesInterface.generate(Module module) {
    final dartTypes = DartTypesInterface._();
    dartTypes._module = module;

    final _interfaces = <Class>[];

    for (var type_node in module.request.idlNodes) {
      final dynamic node = type_node.node;

      if (node is TypeInterface) {
        _interfaces.addAll(dartTypes._addInterface(node));
      }
    }

    final library = Library((b) => b
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..as = '$kTypesLib'
        ..url = '$kTypesLib.dart'))
      ..directives.add(Directive.import('dart:typed_data'))
      ..directives.add(Directive.import('dart:collection'))
      ..directives.add(Directive((b) => b
        ..type = DirectiveType.import
        ..url = 'package:idl_internal/idl_internal.dart'))
      ..body.addAll(_interfaces));

    dartTypes._library = library;

    return dartTypes;
  }

  late Library _library;
  late Module _module;

  @override
  String toString() {
    final emitter = DartEmitter();
    return DartFormatter().format('${_library.accept(emitter)}').trim();
  }
}

class ClientTypes {
  static LayerItem generate(Module module) {
    final result = List<StorageItem>.empty(growable: true);

    final dartLibrary = DartLib.generate(module);
    final pubspec = DartSpec.generate(module);

    // TODO do not generate if it's not necessary
    final dartTypesInterface = DartTypesInterface.generate(module);
    final dartTypes = DartTypes.generate(module);

    result.addAll([
      createFolderItem('lib', [
        createSourceItem('${module.libraryName}.dart', dartLibrary.toString()),
        createFolderItem('src', [
          createSourceItem('$kTypesLib.dart', dartTypes.toString()),
          createSourceItem(
              '$kTypesInterfaceLib.dart', dartTypesInterface.toString()),
        ]),
      ]),
    ]);

    return LayerItem(pubspec: pubspec, storageItems: result);
  }
}