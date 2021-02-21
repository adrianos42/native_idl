import 'dart:ui';
import 'package:desktop/desktop.dart';
import 'package:url_launcher/url_launcher.dart';

import 'overview.dart';

void main() => runApp(MyApp());

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
  ThemeData themeData = ThemeData(
      brightness: Brightness.dark, primaryColor: PrimaryColors.redPink);

  @override
  Widget build(BuildContext context) {
    final githubImage;
    if (themeData.brightness == Brightness.dark) {
      githubImage = Image.asset(
        'assets/GitHub-Mark-Light-120px-plus.png',
        filterQuality: FilterQuality.high,
        width: 19.0,
        height: 19.0,
      );
    } else {
      githubImage = Image.asset(
        'assets/GitHub-Mark-120px-plus.png',
        filterQuality: FilterQuality.high,
        width: 19.0,
        height: 19.0,
      );
    }

    return DesktopApp(
      theme: themeData,
      home: Builder(
        builder: (context) => Container(
          alignment: Alignment.center,
          padding: EdgeInsets.all(16.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisSize: MainAxisSize.max,
            children: [
              Row(
                children: [
                  Container(
                    alignment: Alignment.centerLeft,
                    padding: EdgeInsets.symmetric(vertical: 8.0),
                    child: Builder(
                      builder: (context) {
                        return Text(
                          'Native Idl',
                          style: Theme.of(context).textTheme.title.copyWith(
                              color: Theme.of(context)
                                  .colorScheme
                                  .primary
                                  .toColor()),
                        );
                      },
                    ),
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
                  Builder(builder: (context) {
                    return ThemeToggle(
                      onPressed: () => setState(() {
                        themeData = Theme.invertedThemeOf(context);
                      }),
                    );
                  }),
                ],
              ),
              Expanded(
                child: Tree(
                  title: Builder(
                    builder: (context) => Text(
                      'Documentation',
                      style: Theme.of(context).textTheme.body2,
                    ),
                  ),
                  nodes: [
                    TreeNode(
                      'Overview',
                      builder: (context) => OverviewPage(),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class ThemeToggle extends StatefulWidget {
  ThemeToggle({
    required this.onPressed,
    Key? key,
  }) : super(key: key);

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
            color: iconForeground.toColor(),
          ),
        );
      case Brightness.light:
        return Button(
          onPressed: widget.onPressed,
          body: Icon(
            IconData(0x62, fontFamily: 'mode'),
            color: iconForeground.toColor(),
          ),
        );
    }
  }
}
