import 'package:desktop/desktop.dart';

class TreeViewNavigator extends StatelessWidget {
  const TreeViewNavigator({
    Key? key,
    required this.navigatorKey,
    required this.child,
  }) : super(key: key);

  final Widget child;

  final GlobalKey<NavigatorState> navigatorKey;

  static NavigatorState? of(BuildContext context) {
    final _CurrentNavigator? currentNavigator =
        context.dependOnInheritedWidgetOfExactType<_CurrentNavigator>();

    assert(currentNavigator != null);

    return currentNavigator!.navigatorState;
  }

  Widget build(BuildContext context) {
    return _CurrentNavigator(
      navigatorKey: navigatorKey,
      child: child,
    );
  }
}

class _CurrentNavigator extends InheritedWidget {
  const _CurrentNavigator({
    Key? key,
    required Widget child,
    this.navigatorKey,
  }) : super(key: key, child: child);

  final GlobalKey<NavigatorState>? navigatorKey;

  NavigatorState? get navigatorState => navigatorKey?.currentState;

  @override
  bool updateShouldNotify(_CurrentNavigator old) =>
      old.navigatorKey != navigatorKey;
}

class TreeMenuRoute<T> extends PopupRoute<T> {
  TreeMenuRoute({
    required WidgetBuilder pageBuilder,
    String? barrierLabel,
    RouteTransitionsBuilder? transitionBuilder,
    RouteSettings? settings,
  })  : _pageBuilder = pageBuilder,
        _barrierLabel = barrierLabel,
        super(settings: settings);

  final WidgetBuilder _pageBuilder;

  Tween<Offset>? _offsetTween;

  Animation<double>? _animation;

  static final Curve _animationCurve = Curves.easeOutCubic;

  @override
  bool get barrierDismissible => true;

  @override
  String? get barrierLabel => _barrierLabel;
  final String? _barrierLabel;

  @override
  Color? get barrierColor => null;

  @override
  Duration get transitionDuration => const Duration(milliseconds: 200);

  @override
  Animation<double> createAnimation() {
    assert(_animation == null);
    _animation = CurvedAnimation(
      parent: super.createAnimation(),
      curve: _animationCurve,
      reverseCurve: _animationCurve.flipped,
    );

    _offsetTween = Tween<Offset>(
      begin: const Offset(-1.0, 0.0),
      end: const Offset(0.0, 0.0),
    );
    return _animation!;
  }

  @override
  Widget buildPage(BuildContext context, Animation<double> animation,
      Animation<double> secondaryAnimation) {
    return Semantics(
      child: _pageBuilder(context),
      scopesRoute: true,
      explicitChildNodes: true,
    );
  }

  @override
  Widget buildTransitions(BuildContext context, Animation<double> animation,
      Animation<double> secondaryAnimation, Widget child) {
    return ClipRect(
      child: Align(
        alignment: Alignment.topLeft,
        heightFactor: 1.0,
        child: FractionalTranslation(
          translation: _offsetTween!.evaluate(_animation!),
          child: child,
        ),
      ),
    );
  }
}

class TreeView extends StatefulWidget {
  const TreeView({
    required this.builder,
    required this.navigatorObserver,
    this.navigatorKey,
    Key? key,
  }) : super(key: key);

  final WidgetBuilder builder;

  final GlobalKey<NavigatorState>? navigatorKey;

  final NavigatorObserver navigatorObserver;

  @override
  TreeViewState createState() => TreeViewState();
}

class TreeViewState extends State<TreeView> {
  @override
  Widget build(BuildContext context) {
    return Navigator(
      key: widget.navigatorKey,
      onGenerateRoute: _onGenerateRoute,
      onUnknownRoute: _onUnknownRoute,
      observers: <NavigatorObserver>[widget.navigatorObserver],
    );
  }

  Route<dynamic>? _onGenerateRoute(RouteSettings settings) {
    final String name = settings.name!;

    if (name == Navigator.defaultRouteName) {
      return DesktopPageRoute<dynamic>(
        builder: widget.builder,
        settings: settings,
      );
    }

    return null;
  }

  Route<dynamic> _onUnknownRoute(RouteSettings settings) {
    ThemeData themeData = Theme.of(context);

    return TreeMenuRoute(
      pageBuilder: (context) => Container(
        alignment: Alignment.center,
        color: themeData.colorScheme.background,
        child: Text(
          'Page "${settings.name}" not found',
          style: themeData.textTheme.title,
        ),
      ),
      settings: settings,
    );
  }
}

class TreeObserver extends NavigatorObserver {
  @override
  void didPush(Route<dynamic> route, Route<dynamic>? previousRoute) {
//    assert(route.navigator == _tabState._currentNavigator);
    if (route is PopupRoute<dynamic> || route is PageRoute<dynamic>) {}
  }

  @override
  void didPop(Route<dynamic> route, Route<dynamic>? previousRoute) {
    if (!route.isFirst && route is PopupRoute<dynamic> ||
        route is PageRoute<dynamic>) {}
  }

  @override
  void didRemove(Route<dynamic> route, Route<dynamic>? previousRoute) {}

  @override
  void didReplace({Route<dynamic>? newRoute, Route<dynamic>? oldRoute}) {}
}
