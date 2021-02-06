import 'package:pub_semver/pub_semver.dart';
import 'package:yaml/yaml.dart';
import 'dependency.dart';
import 'yaml_string.dart';

class Pubspec {
  /// Required for every package.
  final String name;

  /// Required for packages that are hosted on the pub.dev site.
  final Version? version;

  /// Required for packages that are hosted on the pub.dev site.
  final String? description;

  /// Optional. URL pointing to the package’s homepage (or source code repository).
  final String? homepage;

  /// Optional. URL pointing to the package’s source code repository.
  final Uri? repository;

  /// Optional. URL pointing to an issue tracker for the package.
  final Uri? issueTracker;

  /// Optional. URL pointing to documentation for the package.
  final String? documentation;

  /// Optional. Specify where to publish a package.
  final String? publishTo;

  /// Can be omitted if your package has no dependencies.
  final Map<String, Dependency>? dependencies;

  /// Can be omitted if your package has no dev dependencies.
  final Map<String, Dependency>? devDependencies;

  /// Can be omitted if you do not need to override any dependencies.
  final Map<String, Dependency>? dependencyOverrides;

  /// Required as of Dart 2.
  final Map<String, VersionConstraint>? environment;

  final Map<String, dynamic>? flutter;

  Pubspec(
    this.name, {
    this.environment,
    this.version,
    this.repository,
    this.issueTracker,
    this.documentation,
    this.homepage,
    this.description,
    this.dependencies,
    this.devDependencies,
    this.dependencyOverrides,
    this.flutter,
    this.publishTo,
  });

  factory Pubspec.parse(String text) {
    String name;
    Version? version;
    String? description;
    String? homepage;
    Uri? repository;
    Uri? issueTracker;
    String? documentation;
    String? publishTo;
    Map<String, Dependency>? dependencies;
    Map<String, Dependency>? devDependencies;
    Map<String, Dependency>? dependencyOverrides;
    Map<String, VersionConstraint>? environment;
    Map<String, dynamic>? flutter;

    final yamlData = loadYaml(text);

    name = yamlData['name'];

    final versionNode = yamlData['version'];
    if (versionNode != null) {
      version = Version.parse(versionNode);
    }

    description = yamlData['description'];
    homepage = yamlData['homepage'];
    repository = yamlData['repository'];
    issueTracker = yamlData['issueTracker'];
    documentation = yamlData['documentation'];
    publishTo = yamlData['publishTo'];

    final dependenciesNode = yamlData['dependencies'];
    if (dependenciesNode != null) {
      dependencies = {};
      for (var entry in dependenciesNode.entries) {
        dependencies.putIfAbsent(
            entry.key, () => Dependency.parse(entry.value));
      }
    }

    final devDependenciesNode = yamlData['devDependencies'];
    if (devDependenciesNode != null) {
      devDependencies = {};
      for (var entry in dependenciesNode.entries) {
        devDependencies.putIfAbsent(
            entry.key, () => Dependency.parse(entry.value));
      }
    }

    final dependencyOverridesNode = yamlData['dependencyOverrides'];
    if (dependencyOverridesNode != null) {
      dependencyOverrides = {};
      for (var entry in dependenciesNode.entries) {
        dependencyOverrides.putIfAbsent(
            entry.key, () => Dependency.parse(entry.value));
      }
    }

    final flutterNode = yamlData['flutter'];
    if (flutterNode != null) {
      flutter = fromStringMap(flutterNode);
    }

    final environmentNode = yamlData['environment'];
    if (environmentNode != null) {
      environment = {};
      for (var entry in environmentNode.entries) {
        environment.putIfAbsent(
            entry.key, () => VersionConstraint.parse(entry.value));
      }
    }

    return Pubspec(
      name,
      version: version,
      description: description,
      homepage: homepage,
      repository: repository,
      issueTracker: issueTracker,
      documentation: documentation,
      publishTo: publishTo,
      dependencies: dependencies,
      devDependencies: devDependencies,
      dependencyOverrides: dependencyOverrides,
      environment: environment,
      flutter: flutter,
    );
  }

  Pubspec merge(Pubspec value) {
    if (name != value.name) {
      throw Exception('Different library name when merging pubspecs');
    }
    final yDependencies = dependencies ?? {};
    value.dependencies
        ?.forEach((key, value) => yDependencies.putIfAbsent(key, () => value));

    final yDevDependencies = devDependencies ?? {};
    value.devDependencies?.forEach(
        (key, value) => yDevDependencies.putIfAbsent(key, () => value));

    final yDependencyOverrides = dependencyOverrides ?? {};
    value.dependencyOverrides?.forEach(
        (key, value) => yDependencyOverrides.putIfAbsent(key, () => value));

    final yFlutter = flutter ?? {};
    value.flutter
        ?.forEach((key, value) => yFlutter.putIfAbsent(key, () => value));

    final yEnvironment = environment ?? {};
    value.environment
        ?.forEach((key, value) => yEnvironment.putIfAbsent(key, () => value));

    return Pubspec(
      name,
      flutter: yFlutter,
      environment: yEnvironment,
      dependencies: yDependencies,
      devDependencies: yDevDependencies,
      dependencyOverrides: yDependencyOverrides,
      description: description ?? value.description,
      documentation: documentation ?? value.documentation,
      homepage: homepage ?? value.homepage,
      issueTracker: issueTracker ?? value.issueTracker,
      publishTo: publishTo ?? value.publishTo,
    );
  }

  @override
  String toString() {
    final yEnvironment = toStringMap({'environment': environment});
    var result = '''
name: $name
$yEnvironment
''';
    if (version != null) {
      result += '''
version: $version
''';
    }
    if (repository != null) {
      result += '''
repository: $repository
''';
    }
    if (issueTracker != null) {
      result += '''
issue_tracker: $issueTracker
''';
    }
    if (documentation != null) {
      result += '''
documentation: $documentation
''';
    }
    if (homepage != null) {
      result += '''
homepage: $homepage
''';
    }
    if (description != null) {
      result += '''
description: $description
''';
    }
    if (dependencies != null) {
      final mapValue = toStringMap({'dependencies': dependencies!});
      result += '''
$mapValue
''';
    }
    if (devDependencies != null) {
      final mapValue = toStringMap({'dev_dependencies': devDependencies!});
      result += '''
$mapValue
''';
    }
    if (dependencyOverrides != null) {
      final mapValue =
          toStringMap({'dependency_overrides': dependencyOverrides!});
      result += '''
$mapValue
''';
    }
    if (flutter != null) {
      final mapValue = toStringMap({'flutter': flutter!});
      result += '''
$mapValue
''';
    }
    if (publishTo != null) {
      result += '''
publish_to: $publishTo
''';
    }

    return result;
  }
}
