import 'package:desktop/desktop.dart';

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
  @override
  Widget build(BuildContext context) {
    return DesktopApp(
      home: Container(
        alignment: Alignment.center,
        padding: EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisSize: MainAxisSize.max,
          children: [
            Header(
              'Native Idl',
              foreground: Theme.of(context).colorScheme.primary,
            ),
            Expanded(
                child: Tab(
              items: [
                TabItem(
                    title: Text('page1'),
                    builder: (context) => ConstrainedBox(
                          constraints:
                              BoxConstraints(maxHeight: 60.0, maxWidth: 60.0),
                          child: Center(
                            child: Text('page1'),
                          ),
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
