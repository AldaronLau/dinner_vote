import 'package:flutter/material.dart';
import 'mealui.dart';
import 'person_ui.dart';
import 'dart:io';
import 'package:http/http.dart' as http;
import 'package:path_provider/path_provider.dart';

void main() => runApp(MyApp());

final key = GlobalKey<DinnerVoteAppState>();

class MyApp extends StatelessWidget {
    @override
    Widget build(BuildContext context) {
        return new MaterialApp(
            title: "MealVote",
            theme: ThemeData(
                primaryColor: Color(0xFF3832AC),
                primaryColorLight: Color(0xFF705DDF),
                primaryColorDark: Color(0xFF000A7C),
                accentColor: Color(0xFFFFA866),
                cardColor: Color(0xFFDDDDDD),
            ),
            debugShowCheckedModeBanner: false,
            home: new MealListPage(key, storage: LocalStorage()),
        );
    }
}

class DinnerVoteApp extends StatefulWidget {
    final LocalStorage storage;

    DinnerVoteApp({@required this.storage}): super(key: key);

    @override
    State<StatefulWidget> createState() => DinnerVoteAppState();
}

class DrawerItem {
    final String title;
    final IconData icon;
    final Widget page;

    DrawerItem(this.title, this.icon, this.page);
}

class DinnerVoteAppState extends State<DinnerVoteApp> {


    final items = [
        DrawerItem('Meals', Icons.info, MealListPage(key)),
        DrawerItem('People', Icons.person, PersonListPage(key)),
    ];

    var item;

    DinnerVoteAppState() {
        item = items[0];
    }
    
    @override
    void initState() {
        super.initState();
    }
  
    void get_dinners() {
        String body = "l";
        http.post('192.168.0.111:8080/meal_vote', body: body).then((resp) => {
            print(resp.body)
            // Do something with the response.
        });
    }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'MealVote',
      theme: ThemeData(
        primaryColor: Color(0xFF3832AC),
        primaryColorLight: Color(0xFF705DDF),
        primaryColorDark: Color(0xFF000A7C),
        accentColor: Color(0xFFFFA866),
        cardColor: Color(0xFFDDDDDD),
      ),
      home: item.page,
    );
  }

  Drawer getDrawer(BuildContext context) {
    final tiles = <Widget>[];
    tiles.add(DrawerHeader(
      child: Text('Pages'),
      decoration: BoxDecoration(
        color: Theme.of(context).primaryColorLight,
      ),
    ));
    for (var i = 0; i < items.length; i++) {
      final di = items[i];
      tiles.add(
        ListTile(
          leading: Icon(di.icon),
          title: Text(di.title),
          selected: item == items[i],
          onTap: () => _onSelectItem(context, i),
        )
      );
    }
    return Drawer(
      child: ListView(
        padding: EdgeInsets.zero,
        children: tiles,
      ),
    );
  }

  _onSelectItem(BuildContext context, int index) {
    setState(() => item = items[index]);
    Navigator.of(context).pop();
  }
}
