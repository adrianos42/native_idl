import 'dart:ui';
import 'package:desktop/desktop.dart';
import 'package:url_launcher/url_launcher.dart';

import 'overview.dart';

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
          crossAxisAlignment: CrossAxisAlignment.center,
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
                child: Tab(
              items: [
                TabItem(
                    title: Text('Overview'),
                    builder: (context) => Padding(
                          padding: EdgeInsets.all(8.0),
                          child: OverviewPage(),
                        )),
                TabItem(
                    title: Text('page2'),
                    builder: (context) => ConstrainedBox(
                          constraints:
                              BoxConstraints(maxHeight: 60.0, maxWidth: 60.0),
                          child: Center(
                            child: Text('page2'),
                          ),
                        )),
              ],
            ))
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
