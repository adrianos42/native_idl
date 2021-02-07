import 'dart:collection';

import 'package:desktop/desktop.dart';
import 'tree_i.dart';

class TreeNode {
  final List<TreeNode>? children;
  final WidgetBuilder? builder;
  final String title;

  const TreeNode(this.title, {this.builder, this.children})
      : assert(builder == null || children == null);
}

class Tree extends StatefulWidget {
  Tree({
    this.title,
    required this.nodes,
    Key? key,
  }) : super(key: key);

  final Widget? title;
  final List<TreeNode> nodes;

  @override
  _TreeState createState() => _TreeState();

  static _TreeState? _of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<_TreeScope>()?.treeState;
}

class _TreeState extends State<Tree> {
  final _pages = HashMap<String, WidgetBuilder>();
  String? _current;

  void setPage(String name) => setState(() => _current = name);

  void _createEntries(String name, TreeNode node) {
    final nameResult = '''$name${node.title}''';

    if (node.children != null) {
      for (var child in node.children!) {
        _createEntries(nameResult, child);
      }
    } else if (node.builder != null) {
      _pages[nameResult] = node.builder!;
    } else {
      throw Exception('Either builder or children must be non null');
    }
  }

  @override
  Widget build(BuildContext context) {
    final pagesResult = List<Widget>.empty(growable: true);

    if (widget.nodes.isEmpty) {
      throw Exception('Nodes cannot be empty');
    }

    _current ??= widget.nodes.first.title;

    for (var node in widget.nodes) {
      _createEntries('', node);
    }

    for (var entry in _pages.entries) {
      final active = entry.key == _current!;
      pagesResult.add(
        Offstage(
          offstage: !active,
          child: TickerMode(
            enabled: active,
            child: FocusScope(
              canRequestFocus: active,
              child: Builder(
                builder: (context) {
                  return TabView(
                    builder: entry.value,
                    navigatorObserver: TreeObserver(),
                  );
                },
              ),
            ),
          ),
        ),
      );
    }

    Widget result = Row(
      children: [
        Container(
          alignment: Alignment.topLeft,
          width: 200.0,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (widget.title != null)
                Padding(
                  padding: EdgeInsets.symmetric(vertical: 8.0),
                  child: widget.title!,
                ),
              Column(
                mainAxisSize: MainAxisSize.min,
                mainAxisAlignment: MainAxisAlignment.start,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: widget.nodes
                    .map((e) => TreeColumn(
                          node: e,
                          parentName: '',
                        ))
                    .toList(),
              ),
            ],
          ),
        ),
        Expanded(
          child: Stack(
            children: pagesResult,
          ),
        )
      ],
    );

    result = _TreeScope(
      child: result,
      treeState: this,
    );

    return result;
  }
}

class _TreeScope extends InheritedWidget {
  const _TreeScope({
    Key? key,
    required this.treeState,
    required Widget child,
  }) : super(key: key, child: child);

  final _TreeState treeState;

  @override
  bool updateShouldNotify(_TreeScope old) => old.treeState != treeState;
}

class TreeColumn extends StatefulWidget {
  TreeColumn({
    required this.node,
    required this.parentName,
    Key? key,
  }) : super(key: key);

  final TreeNode node;
  final String parentName;

  @override
  _TreeColumnState createState() => _TreeColumnState();
}

class _TreeColumnState extends State<TreeColumn> {
  var _collapsed = true;

  String get name => '${widget.parentName}${widget.node.title}';

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;
    final textTheme = Theme.of(context).textTheme;

    if (widget.node.title.isEmpty) {
      throw Exception('Title in tree cannot be null');
    }

    if (widget.node.children != null) {
      final iconCollpased = _collapsed ? Icons.expand_more : Icons.expand_less;
      final chidrenWidget = Padding(
        padding: EdgeInsets.only(left: 16.0),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: widget.node.children!
              .map((e) => TreeColumn(
                    node: e,
                    parentName: name,
                  ))
              .toList(),
        ),
      );

      return Column(
        mainAxisSize: MainAxisSize.min,
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          ButtonTheme.merge(
            data: ButtonThemeData(
              color: textTheme.textLow,
              bodyPadding: EdgeInsets.zero,
              trailingPadding: EdgeInsets.only(left: 12.0),
              buttonPadding: EdgeInsets.zero,
              // color: Theme.of(context).textTheme.textHigh,
            ),
            child: Button(
              body: Text(widget.node.title),
              trailing: Icon(iconCollpased),
              onPressed: () => setState(() => _collapsed = !_collapsed),
            ),
          ),
          Offstage(
            child: chidrenWidget,
            offstage: _collapsed,
          ),
        ],
      );
    } else {
      return ButtonTheme.merge(
        data: ButtonThemeData(
          color: textTheme.textLow,
          buttonPadding: EdgeInsets.zero,
          bodyPadding: EdgeInsets.zero,
          highlightColor: colorScheme.primary
        ),
        child: Button(
          body: Text(widget.node.title),
          color: Tree._of(context)!._current == name
              ? colorScheme.primary
              : null,
          onPressed: () {
            Tree._of(context)!.setPage(name);
          },
        ),
      );
    }
  }
}
