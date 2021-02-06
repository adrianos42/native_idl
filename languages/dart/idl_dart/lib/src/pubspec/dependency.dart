import 'dart:collection';

import 'package:pub_semver/pub_semver.dart';

import 'yaml_string.dart';

class Dependency implements YamlValue {
  final VersionConstraint? version;
  final Uri? path;

  Dependency({this.version, this.path});

  @override
  dynamic get value {
    if (version != null) {
      return version;
    } else if (path != null) {
      return {'path': path};
    } else {
      throw Exception('Need a version number or a path for dependency');
    }
  }

  factory Dependency.parse(dynamic value) {
    if (value is Map) {
      final entry = value.entries.first;

      if (entry.key == 'sdk') {
        return SdkDependency.parse(entry.value);
      } else if (entry.key == 'git') {
        return GitDependency.parse(entry.value);
      } else if (entry.key == 'path') {
        return Dependency(path: Uri.parse(entry.value));
      } else {
        throw Exception();
      }
    } else if (value is String) {
      return Dependency(version: VersionConstraint.parse(value));
    } else {
      throw Exception();
    }
  }
}

// TODO
class SdkDependency extends Dependency {
  final String sdk;

  @override
  final VersionConstraint? version;

  SdkDependency(this.sdk, {this.version});

  factory SdkDependency.parse(dynamic value) {
    if (value is Map) {
      final entry = value.entries.first;
      return SdkDependency(entry.key,
          version: VersionConstraint.parse(entry.value));
    } else if (value is String) {
      return SdkDependency(value);
    } else {
      throw Exception();
    }
  }

  @override
  dynamic get value {
    if (version == null) {
      return {'sdk': sdk};
    } else {
      return {
        'sdk': {sdk: version}
      };
    }
  }
}

class GitDependency extends Dependency {
  final Uri url;
  final String? ref;
  @override
  final Uri? path;

  GitDependency({required this.url, this.ref, this.path});

  factory GitDependency.parse(dynamic value) {
    if (value is Map) {
      Uri? path;
      String? ref;
      final url = Uri.parse(value['url']);

      if (value['ref'] != null) {
        ref = value['ref'];
      } else if (value['path'] != null) {
        path = Uri.parse(value['path']);
      }
      return GitDependency(url: url, path: path, ref: ref);
    } else if (value is String) {
      return GitDependency(url: Uri.parse(value));
    } else {
      throw Exception();
    }
  }

  @override
  dynamic get value {
    if (ref != null) {
      return {
        'git': {'url': url, 'ref': ref}
      };
    } else if (path != null) {
      return {
        'git': {'url': url, 'path': path}
      };
    } else {
      return {'git': url};
    }
  }
}
