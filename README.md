# Native Idl

## Installation

```sh
cargo install --path idl_cmd/
```

### Dart support

With `target_dir` for the dart binary directory.
Needs to add it to the path variable.

```sh
cd languages/dart/idl_dart
pub get
dart2native bin/main.dart -o target_dir/idl_dart
```
