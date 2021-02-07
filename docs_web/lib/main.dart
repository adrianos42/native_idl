import 'dart:js';
import 'dart:ui';
import 'package:desktop/desktop.dart';
import 'package:desktop/document.dart';
import 'package:url_launcher/url_launcher.dart';

import 'overview.dart';
import 'tree.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MyHomePage();
  }
}

class MyHomePage extends StatefulWidget {
  MyHomePage({Key? key}) : super(key: key);

  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  ThemeData themeData = ThemeData(brightness: Brightness.dark);

  @override
  Widget build(BuildContext context) {
    final githubImage;
    if (themeData.brightness == Brightness.dark) {
      githubImage = Image.asset(
        'assets/GitHub-Mark-Light-120px-plus.png',
        width: 19.0,
        height: 19.0,
      );
    } else {
      githubImage = Image.asset(
        'GitHub-Mark-120px-plus.png',
        width: 19.0,
        height: 19.0,
      );
    }

    return DesktopApp(
      theme: themeData,
      home: Container(
        alignment: Alignment.center,
        padding: EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisSize: MainAxisSize.max,
          children: [
            Row(
              children: [
                Header(
                  'Native Idl',
                  foreground: Theme.of(context).colorScheme.primary,
                ),
                Spacer(),
                Button(
                  body: githubImage,
                  onPressed: () async {
                    final urlRepository =
                        'https://github.com/adrianos42/native_idl';
                    if (await canLaunch(urlRepository)) {
                      await launch(urlRepository);
                    }
                  },
                ),
                ThemeToggle(onPressed: () {
                  setState(() {
                    if (themeData.brightness == Brightness.dark) {
                      themeData = ThemeData(brightness: Brightness.light);
                    } else {
                      themeData = ThemeData(brightness: Brightness.dark);
                    }
                  });
                }),
              ],
            ),
            Expanded(
              child: Padding(
                padding: EdgeInsets.all(8.0),
                child: Tree(
                  title: Text(
                    'Documentation',
                    style: Theme.of(context).textTheme.body2,
                  ),
                  nodes: [
                    TreeNode(
                      'Overview',
                      builder: (context) => Center(
                        child: Text('Overview'),
                      ),
                    ),
                    TreeNode('Idl language', children: [
                      TreeNode(
                        'Concepts',
                        builder: (context) => Center(
                          child: Text('Concepts'),
                        ),
                      ),
                      TreeNode(
                        'Syntax',
                        builder: (context) => Center(
                          child: Text('Syntax'),
                        ),
                      ),
                    ]),
                    TreeNode('Layers', children: [
                      TreeNode(
                        'FFI',
                        builder: (context) => Center(
                          child: Text('FFI'),
                        ),
                      ),
                      TreeNode(
                        'WebSocket',
                        builder: (context) => Center(
                          child: Text('WebSocket'),
                        ),
                      ),
                    ]),
                    TreeNode('Languages', children: [
                      TreeNode(
                        'Rust',
                        builder: (context) => Center(
                          child: Text('Rust'),
                        ),
                      ),
                      TreeNode(
                        'Dart',
                        builder: (context) => Center(
                          child: Text('Dart'),
                        ),
                      ),
                    ])
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class ThemeToggle extends StatefulWidget {
  ThemeToggle({required this.onPressed, Key? key}) : super(key: key);

  final VoidCallback onPressed;

  @override
  _ThemeToggleState createState() => _ThemeToggleState();
}

class _ThemeToggleState extends State<ThemeToggle> {
  @override
  Widget build(BuildContext context) {
    final themeData = Theme.of(context);
    final iconForeground = themeData.textTheme.textForeground;
    switch (themeData.brightness) {
      case Brightness.dark:
        return Button(
          onPressed: widget.onPressed,
          body: Icon(
            IconData(0x61, fontFamily: 'mode'),
            color: iconForeground,
          ),
        );
      case Brightness.light:
        return Button(
          onPressed: widget.onPressed,
          body: Icon(
            IconData(0x62, fontFamily: 'mode'),
            color: iconForeground,
          ),
        );
    }
  }
}
