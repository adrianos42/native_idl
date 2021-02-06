import 'package:pub_semver/pub_semver.dart';

abstract class YamlValue {
  dynamic get value;
}

String toStringMap(dynamic value, [int depth = 0]) {
  final spaceDepth = '  ' * depth;
  var result = '';

  if (value is Map) {
    for (var entry in value.entries) {
      var yValue;
      var valueEntry;

      if (entry.value is YamlValue) {
        valueEntry = (entry.value as YamlValue).value;
      } else {
        valueEntry = entry.value;
      }

      if (valueEntry is Map || valueEntry is List) {
        yValue = toStringMap(valueEntry, depth + 1);
      } else if (valueEntry is VersionConstraint) {
        yValue = ' \'${valueEntry.toString()}\'';
      } else {
        yValue = ' ${valueEntry.toString()}';
      }

      if (depth > 0) result += '\n';
      result += '''${spaceDepth}${entry.key}:${yValue}''';
    }
  } else if (value is List) {
    for (var entry in value) {
      var yValue;
      if (entry is Map || entry is List) {
        yValue = toStringMap(entry, depth + 1);
      } else {
        yValue = '${entry.toString()}';
      }
      if (depth > 0) result += '\n';
      result += '''${spaceDepth}- ${yValue}''';
    }
  }

  return result;
}

dynamic fromStringMap(dynamic value) {
  if (value is Map) {
    final result = <String, dynamic>{};
    for (var entry in value.entries) {
      if (entry.value is Map) {
        result.putIfAbsent(entry.key, () => fromStringMap(entry.value));
      } else if (entry.value is String) {
        result.putIfAbsent(entry.key, () => entry.value);
      } else if (entry.value is List) {
        result.putIfAbsent(entry.key, () => fromStringMap(entry.value));
      }
    }
    return result;
  } else if (value is List) {
    return value;
  }

  throw Exception('Unknown value');
}
