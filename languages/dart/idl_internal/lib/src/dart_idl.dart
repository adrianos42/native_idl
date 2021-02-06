import 'package:quiver/core.dart';

class Result<R, E> {
  factory Result.ok(R value) => Result._(value, true);
  factory Result.err(E value) => Result._(value, false);

  Result._(this._value, this._variant);

  final dynamic _value;
  final bool _variant;

  Object? get ok => _variant ? _value : null;
  Object? get err => _variant ? null : _value;

  @override
  bool operator ==(dynamic other) =>
      other._variant == _variant && other._value == _value;

  @override
  int get hashCode => hash2(_value, _variant);
}

abstract class Disposable {
  void dispose();
}

enum StreamSenderState {
  ok, 
  value,
  request,
  waiting,
  done,
}

enum StreamReceiverState {
  ok,
  close,
  start,
  pause,
  resume,
  request,
}

class Pair<F, S> {
  final F first;
  final S second;

  const Pair(this.first, this.second);

  Pair withFirst(F value) => Pair(value, second);
  Pair withSecond(S value) => Pair(first, value);

  @override
  bool operator ==(Object other) => other is Pair && other.first == first && other.second == second;

  @override
  int get hashCode => hash2(first, second);
}