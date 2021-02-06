String toSnakeCase(String value) => value.replaceAllMapped(
    RegExp(r'(^[A-Z])|([A-Z]$)|([A-Z])'),
    (match) => match[1] != null
        ? match[1]!.toLowerCase()
        : match[2] != null
            ? match[2]!.toLowerCase()
            : '_' + match[3]!.toLowerCase());

String toPascalCase(String value) => value.replaceAllMapped(
    RegExp(r'(^[a-z])|(?:_([a-z]))|(_)'),
    (match) => match[1] != null
        ? match[1]!.toUpperCase()
        : match[2] != null
            ? match[2]!.toUpperCase()
            : '');

String toCamelCase(String value) => value.replaceAllMapped(
    RegExp(r'(^[A-Z])|(?:_([a-z]))|(_)'),
    (match) => match[1] != null
        ? match[1]!.toLowerCase()
        : match[2] != null
            ? match[2]!.toUpperCase()
            : '');
