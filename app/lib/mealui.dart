import 'dart:io';
import 'main.dart';
import 'meal.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:path_provider/path_provider.dart';
// import 'db.dart';

class MealListPage extends StatefulWidget {
    final GlobalKey appKey;
    final LocalStorage storage;

    MealListPage(appKey, {@required this.storage}) : appKey = appKey;

    @override
    _MealListPageState createState() => _MealListPageState();
}

class _MealListPageState extends State<MealListPage> {
    final _dinners = new List<Meal>();
    
    String username = null;

    void create_user(String user) {
        widget.storage.writeName(user);
        String body = "c " + user;
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) => {});
    }

    void get_dinners() {
        String body = "l";
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((resp) {
            _dinners.clear();
            _dinners.add(Meal("ChickEN!", "short desc.", "looong dsc"));

            print(resp.body);
            // Do something with the response.
        });
    }

    @override
    void initState() {
        super.initState();
        get_dinners();
        // Read name from file, or make user set name.
        widget.storage.readName().then((String name) {
            if (name == null) {
                showDialog(
                    context: context,
                    builder: (_) => SetName(),
                ).then((var name) {
                    setState(() {
                        username = name;
                        create_user(username);
                    });
                });
            } else {
                setState(() {
                    username = name;
                });
            }
        });
        
        _loadMeals();
    }

  @override
  Widget build(BuildContext context) {
    Text title;
    if (username == null) {
        title = Text('Loading MealVote…');
    } else {
        title = Text('MealVote: ' + username);
    }
  
    return Scaffold(
      appBar: AppBar(title: title),
      body: _buildBody(),
      floatingActionButton: FloatingActionButton(
        child: Icon(Icons.add),
        onPressed: () => _createMeal(context),
      ),
    );
  }

  Widget _buildBody() {
    return ListView.builder(
        padding: const EdgeInsets.all(6.0),
        itemCount: _dinners.length,
        itemBuilder: (context, i) {
          return _buildRow(_dinners[i]);
    });
  }

  Widget _buildRow(Meal meal) {
    return ListTile(
      title: Text(
        meal.title,
        style: Theme.of(context).textTheme.title,
      ),
      subtitle: Text(meal.subtitle),
      onTap: () => _editMeal(context, meal),
    );
  }

  _createMeal(BuildContext context) async {
    String result = await Navigator.push(
      context,
      MaterialPageRoute(builder: (context) => MealPage(Meal("", "", ""))),
    );
    if (result == 'save') {
      _loadMeals();
    }
  }

  _loadMeals() async {
    print("LOAFAF");
    /*_db.getAllMeals().then((meals) {
      setState(() {
        _dinners.clear();
        meals.forEach((meal) {
          _dinners.add(Meal.fromMap(meal));
        });
      });
    });*/
  }

  _editMeal(BuildContext context, Meal meal) async {
    String result = await Navigator.push(
      context,
      MaterialPageRoute(builder: (context) => MealPage(meal)),
    );
    if (result == 'update' || result == 'delete') {
      _loadMeals();
    }
  }
}

class MealPage extends StatefulWidget {
    final Meal meal;
    MealPage(this.meal);

    @override
    State<StatefulWidget> createState() => _MealPageState();
}

class _MealPageState extends State<MealPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(_screenTitle())),
      body: MealBody(widget.meal),
    );
  }

  String _screenTitle() {
    return (widget.meal.id != null) ? 'Edit Meal' : 'Add Meal';
  }
}

class MealBody extends StatefulWidget {
  // This needs to be separate from MealPage to allow
  // SnackBars to work with the Scaffold in MealPage
  final Meal meal;
  MealBody(this.meal);

  @override
  State<StatefulWidget> createState() => _MealBodyState();
}

class _MealBodyState extends State<MealBody> {
  // DbHelper db = new DbHelper();

  TextEditingController _title;
  TextEditingController _description;

  @override
  initState() {
    super.initState();

    _title = new TextEditingController(text: widget.meal.title);
    _description = new TextEditingController(text: widget.meal.description);
  }

  @override
  Widget build(BuildContext context) {
    return ListView(
      shrinkWrap: true,
      padding: EdgeInsets.all(6.0),
      children: <Widget>[
        TextField(
          controller: _title,
          decoration: InputDecoration(labelText: 'Title', isDense: true),
          maxLength: 22,
        ),
        Padding(padding: new EdgeInsets.all(2.0)),
        TextField(
          controller: _description,
          decoration: InputDecoration(labelText: 'Description', isDense: true),
          maxLength: 22,
        ),
        Padding(padding: new EdgeInsets.all(2.0)),
        _buildChips(widget.meal),
        _buildButtons(context),
      ],
    );
  }

  Widget _buildChips(Meal meal) {
    return Wrap(
      spacing: 6.0,
      runSpacing: 4.0,
      children: <Widget>[
        _buildChip(meal, 'Beef'),
        _buildChip(meal, 'Chicken'),
        _buildChip(meal, 'Mexican'),
      ],
    );
  }

  Widget _buildChip(Meal meal, String lbl) {
    return FilterChip(
      selectedColor: Theme.of(context).accentColor,
      selected: meal.hasTag(lbl),
      label: Text(lbl),
      onSelected: (bool value) {
        setState(() {
          // FIXME: set tag on meal
          return value;
        });
      },
    );
  }

  Widget _buildButtons(BuildContext context) {
    if (widget.meal.id != null) {
      return Row(
        children: <Widget>[
          Expanded(child: _buildDeleteButton(context)),
          Expanded(child: _buildButton(context)),
        ],
      );
    } else {
      return _buildButton(context);
    }
  }

  Widget _buildDeleteButton(BuildContext context) {
    return FlatButton.icon(
      label: Text('Delete'),
      icon: Icon(Icons.delete),
      textColor: Theme.of(context).primaryColorDark,
      onPressed: () {
        _deleteMeal(context);
      },
    );
  }

  _deleteMeal(BuildContext context) {
    /*db.deleteMeal(widget.meal.id).then((_) {
      Navigator.pop(context, 'delete');
    });*/
  }

  String _buttonTitle() {
    return (widget.meal.id != null) ? 'Update' : 'Add';
  }

  IconData _buttonIcon() {
    return (widget.meal.id != null) ? Icons.done : Icons.add_circle;
  }

  Widget _buildButton(BuildContext context) {
    return FlatButton.icon(
      label: Text(_buttonTitle()),
      icon: Icon(_buttonIcon()),
      textColor: Theme.of(context).primaryColorDark,
      onPressed: () {
        if (widget.meal.id != null) {
          _updateMeal(context);
        } else {
          _saveMeal(context);
        }
      },
    );
  }

  String _getTitle() {
    return _title.text.trim();
  }

  String _getDescription() {
    return _description.text.trim();
  }

  _updateMeal(BuildContext context) {
    /*db
        .updateMeal(Meal.fromMap({
      'id': widget.meal.id,
      'title': _getTitle(),
      'description': _getDescription(),
    }))
        .then((_) {
      Navigator.pop(context, 'update');
    }).catchError((e) {
      final snackBar = SnackBar(content: Text('Title must be unique'));
      Scaffold.of(context).showSnackBar(snackBar);
    });*/
  }

  _saveMeal(BuildContext context) {
    final title = _getTitle();
    if (title.length == 0)
      return;
    /*db.saveMeal(Meal(title, _getDescription())).then((_) {
      Navigator.pop(context, 'save');
    }).catchError((e) {
      final snackBar = SnackBar(content: Text('Title must be unique'));
      Scaffold.of(context).showSnackBar(snackBar);
    });*/
  }
}

// Set user's name
class SetName extends StatefulWidget {
    SetName({Key key}): super(key: key);

    @override
    SetNameState createState() { return new SetNameState(); }
}

class SetNameState extends State<SetName> {
    TextEditingController text_controller;

    @override
    void initState() {
        super.initState();
        text_controller = TextEditingController(
            text: "",
        );
    }

    @override
    Widget build(BuildContext context) {
        return new AlertDialog(
            title: const Text('Enter your first name:'),
            content: TextField(controller: text_controller,
                toolbarOptions: ToolbarOptions(
                    copy: false, cut: false, paste: false, selectAll: false
                ),
                autofocus: true),
            actions: <Widget>[
                Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                        FlatButton(
                            onPressed: () {
                                Navigator.of(context).pop(text_controller.text);
                            },
                            child: Text("Set Name"),
                        ),
                    ]
                ),
            ],
        );
    }
}

class LocalStorage {
    Future<String> get _localPath async {
        final directory = await getApplicationDocumentsDirectory();

        return directory.path;
    }

    Future<File> get _localFile async {
        final path = await _localPath;
        return File('$path/user.txt');
    }

    Future<String> readName() async {
        try {
            final file = await _localFile;

            // Read the file
            String contents = await file.readAsString();

            return contents;
        } catch (e) {
            // If encountering an error, return 0
            return null;
        }
    }

    Future<File> writeName(String name) async {
        final file = await _localFile;

        // Write the file
        return file.writeAsString('$name');
    }
}
