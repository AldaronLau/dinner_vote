import 'dart:io';
import 'main.dart';
import 'meal.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:path_provider/path_provider.dart';
// import 'db.dart';

enum Mode {
    Vote,
    Edit,
}

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
    int votes = 0;
    Mode mode = Mode.Vote;

    void sort_dinners(String by) {
        switch (by) {
            case "Alphabetical":
                _dinners.sort((a, b) {
                    return a.title.toLowerCase().compareTo(b.title.toLowerCase());
                });
                break;
            case "Votes":
                _dinners.sort((a, b) {
                    if (a.who == null) {
                        if (b.who != null) {
                            return 1;
                        }
                    } else {
                        if (b.who == null) {
                            return -1;
                        }
                    }
                    return a.title.toLowerCase().compareTo(b.title.toLowerCase());
                });
                break;
            default:
                break;
        }
    }

    void create_user(String user) {
        widget.storage.writeName(user);
        String body = "c " + user;
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) {
            get_votes();
        });
    }

    void create_dinner(String title) {
        String body = "n " + username + "\\" + title;
        print("Posting:" + body);
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) => {});
    }

    void set_votes(String username, String votes) {
        String body = "z $username\\$votes";
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((resp) {
            setState(() {
                get_votes();
            });
        });
    }

    void get_votes() {
        String body = "h " + username;
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((resp) {
            setState(() {
                print(resp.body);
                votes = int.parse(resp.body);
            });
        });
    }

    void get_dinners() {
        String body = "l";
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((resp) {
            setState(() {
                List<String> items = resp.body.split("\n");

                _dinners.clear();
                items.forEach((item) {
                    print('item: ' + item);
                    List<String> parts = item.split("\\");
                    if (parts.length == 3) {
                        _dinners.add(Meal(parts[0], parts[1], parts[2]));
                    } else {
                        _dinners.add(Meal(parts[0], parts[1], null));
                    }
                });
                sort_dinners("Votes");
            });
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
                    get_votes();
                });
            }
        });
    }

  @override
  Widget build(BuildContext context) {
    Text title;
    if (username == null) {
        title = Text('Loading MealVote…');
    } else {
        title = Text('MealVote: $username ($votes votes)');
    }

    List<String> menu_options = ["New Dinner…", "Sort…"];

    if (mode == Mode.Vote) {
        menu_options.add("Edit Mode");
    } else {
        menu_options.add("Vote Mode");
    }

    menu_options.add("Set # of Votes");

    // menu_options.add("Settings");

    return Scaffold(
      appBar: AppBar(
        title: title,
        actions: <Widget>[
          PopupMenuButton<String>(
            onSelected: (String choice) {
                switch (choice) {
                    case "New Dinner…":
                        showDialog(
                            context: context,
                            builder: (_) => NewDinner(),
                        ).then((var name) {
                            setState(() {
                                create_dinner(name);
                                get_dinners();
                            });
                        });
                        break;
                    case "Edit Mode":
                        setState(() {
                            mode = Mode.Edit;
                        });
                        break;
                    case "Vote Mode":
                        setState(() {
                            mode = Mode.Vote;
                        });
                        break;
                    case "Sort…":
                        showDialog(
                            context: context,
                            builder: (_) => SortDinners(),
                        ).then((var by) {
                            setState(() {
                                sort_dinners(by);
                            });
                        });
                        break;
                    case "Settings":
                        /*showDialog(
                            context: context,
                            builder: (_) {
                                Navigator.push(
                                    context,
                                    MaterialPageRoute(builder: (context) {
                                        return SelectFolder(mode: FolderMode.Move);
                                    }),
                                );
                            },
                        );*/
                        break;
                    case "Set # of Votes":
                        showDialog(
                            context: context,
                            builder: (_) => SetVotes(),
                        ).then((var number) {
                            if (number != null) {
                                setState(() {
                                    set_votes(username, number);
                                });
                            }
                        });
                        break;
                    default:
                        break;
                }
            },
            itemBuilder: (BuildContext context) {
                return menu_options.map((String choice) {
                    return PopupMenuItem<String>(
                        value: choice,
                        child: Text(choice),
                    );
                }).toList();
            }
          )
        ],
      ),
      body: _buildBody(),
      /*floatingActionButton: FloatingActionButton(
        child: Icon(Icons.add),
        onPressed: () => {},
      ),*/
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
    String meal_title = meal.title;
    String who = meal.who;
    String title;
    if (who == null) {
        title = "$meal_title";
    } else {
        title = "$meal_title ($who)";
    }
  
    return ListTile(
      title: Text(
        title,
        style: Theme.of(context).textTheme.title,
      ),
      subtitle: Text(meal.subtitle),
      onTap: () => _editMeal(context, meal),
    );
  }

  _editMeal(BuildContext context, Meal meal) async {
    String result = await Navigator.push(
      context,
      MaterialPageRoute(builder: (context) => MealPage(meal, mode, username)),
    );
    if (result == 'refresh') {
        get_dinners();
        get_votes();
    }
  }
}

class MealPage extends StatefulWidget {
    final Meal meal;
    final Mode mode;
    final String username;
    MealPage(this.meal, this.mode, this.username);

    @override
    State<StatefulWidget> createState() => _MealPageState();
}

class _MealPageState extends State<MealPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text(_screenTitle())),
      body: MealBody(widget.meal, widget.mode, widget.username),
    );
  }

  String _screenTitle() {
    if (widget.mode == Mode.Vote) {
        return widget.meal.title;
    } else {
        return 'Edit Meal';
    }
  }
}

class MealBody extends StatefulWidget {
  // This needs to be separate from MealPage to allow
  // SnackBars to work with the Scaffold in MealPage
  final Meal meal;
  final Mode mode;
  final String username;
  MealBody(this.meal, this.mode, this.username);

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
    _description = new TextEditingController(text: widget.meal.subtitle);
  }

  @override
  Widget build(BuildContext context) {
    List<Widget> children = [];
    
    if (widget.mode == Mode.Vote) {
        children.add(Text(widget.meal.subtitle));
    } else {
        children.add(
            TextField(
                controller: _title,
                decoration: InputDecoration(labelText: 'Title', isDense: true),
                maxLength: 22,
            )
        );
        children.add(Padding(padding: new EdgeInsets.all(2.0)));
        children.add(
            TextField(
              controller: _description,
              decoration: InputDecoration(labelText: 'Description', isDense: true),
              maxLength: 22,
            ),
        );
    }
    children.add(Padding(padding: new EdgeInsets.all(2.0)));
    children.add(_buildButtons(context));

    return ListView(
      shrinkWrap: true,
      padding: EdgeInsets.all(6.0),
      children: children,
    );
  }

  /*Widget _buildChips(Meal meal) {
    return Wrap(
      spacing: 6.0,
      runSpacing: 4.0,
      children: <Widget>[
        _buildChip(meal, 'Beef'),
        _buildChip(meal, 'Chicken'),
        _buildChip(meal, 'Mexican'),
      ],
    );
  }*/

  /*Widget _buildChip(Meal meal, String lbl) {
    return FilterChip(
      selectedColor: Theme.of(context).accentColor,
      selected: false, // meal.hasTag(lbl),
      label: Text(lbl),
      onSelected: (bool value) {
        setState(() {
          // FIXME: set tag on meal
          return value;
        });
      },
    );
  }*/

  Widget _buildButtons(BuildContext context) {
    if (widget.mode == Mode.Vote) {
      if (widget.meal.who == null) {
        return Row(
          children: <Widget>[
            Expanded(child: _buildButton(context)),
          ],
        );
      } else if (widget.meal.who == widget.username) {
        return Row(
          children: <Widget>[
            Expanded(child: _buildDeleteButton(context)),
          ],
        );
      } else {
        return Row(
          children: <Widget>[],
        );
      }
    } else {
      return Row(
        children: <Widget>[
          Expanded(child: _buildDeleteButton(context)),
          Expanded(child: _buildButton(context)),
        ],
      );
    }
  }

  Widget _buildDeleteButton(BuildContext context) {
    if (widget.mode == Mode.Vote) {
        return FlatButton.icon(
          label: Text('Unvote'),
          icon: Icon(Icons.undo),
          textColor: Theme.of(context).primaryColorDark,
          onPressed: () {
            _unvoteMeal(context);
          },
        );
    } else {
        return FlatButton.icon(
          label: Text('Delete'),
          icon: Icon(Icons.delete),
          textColor: Theme.of(context).primaryColorDark,
          onPressed: () {
            _deleteMeal(context);
          },
        );
    }
  }

    void rename_dinner(String user, String index, String title, String subtitle) {
        // Delete Dinner
        String body = "d $user\\$index";
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) {
            // New Dinner
            String body = "n " + user + "\\" + title;
            http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) {
                // Edit description
                String body = "t $user\\$title\\$subtitle";
                http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) {
                    Navigator.pop(context, 'refresh');
                });
            });
        });
    }
    
    void update_dinner(String user, String index, String subtitle) {
        String body = "t $user\\$index\\$subtitle";
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) {
            Navigator.pop(context, 'refresh');
        });
    }
    
    void delete_dinner(String user, String index) {
        String body = "d $user\\$index";
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) {
            Navigator.pop(context, 'refresh');
        });
    }
    
    void vote_dinner(String user, String index) {
        String body = "v $user\\$index";
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) {
            Navigator.pop(context, 'refresh');
        });
    }
    
    void unvote_dinner(String user, String index) {
        String body = "u $user\\$index";
        http.post('http://192.168.0.111:8080/meal_vote', body: body).then((_) {
            Navigator.pop(context, 'refresh');
        });
    }

  _deleteMeal(BuildContext context) {
    delete_dinner(widget.username, widget.meal.title);
  }

  _unvoteMeal(BuildContext context) {
    unvote_dinner(widget.username, widget.meal.title);
  }

  String _buttonTitle() {
    if (widget.mode == Mode.Vote) {
        return 'Vote';
    } else {
        return 'Update';
    }
  }

  IconData _buttonIcon() {
    return Icons.done; // (widget.meal.id != null) ?  : Icons.add_circle;
  }

  Widget _buildButton(BuildContext context) {
    return FlatButton.icon(
      label: Text(_buttonTitle()),
      icon: Icon(_buttonIcon()),
      textColor: Theme.of(context).primaryColorDark,
      onPressed: () {
        if (widget.mode == Mode.Vote) {
            vote_dinner(widget.username, widget.meal.title);
        } else {
            if (widget.meal.title != _title.text) {
                rename_dinner(widget.username, widget.meal.title, _title.text, _description.text);
            } else {
                update_dinner(widget.username, widget.meal.title, _description.text);
            }
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

class NewDinner extends StatefulWidget {
    NewDinner({Key key}): super(key: key);

    @override
    NewDinnerState createState() { return new NewDinnerState(); }
}

class NewDinnerState extends State<NewDinner> {
    TextEditingController text_controller;

    @override
    void initState() {
        super.initState();
        text_controller = TextEditingController(text: "");
    }

    @override
    Widget build(BuildContext context) {
        return new AlertDialog(
            title: const Text('New Dinner Option:'),
            content: TextField(controller: text_controller,
                toolbarOptions: ToolbarOptions(
                    copy: false, cut: false, paste: false, selectAll: false
                ),
                autofocus: true,
                maxLength: 22),
            actions: <Widget>[
                Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                        FlatButton(
                            onPressed: () {
                                Navigator.of(context).pop("");
                            },
                            child: Text("Cancel"),
                        ),
                        FlatButton(
                            onPressed: () {
                                Navigator.of(context).pop(text_controller.text);
                            },
                            child: Text("Create"),
                        ),
                    ]
                ),
            ],
        );
    }
}

class SortDinners extends StatefulWidget {
    SortDinners({Key key}): super(key: key);

    @override
    SortDinnersState createState() { return new SortDinnersState(); }
}

class SortDinnersState extends State<SortDinners> {
    @override
    void initState() {
        super.initState();
    }

    @override
    Widget build(BuildContext context) {
        return new AlertDialog(
            title: const Text('Sort by:'),
            content: Column(children: <Widget>[
                FlatButton(
                    onPressed: () {
                        Navigator.of(context).pop("Alphabetical");
                    },
                    child: Text("Alphabetical"),
                ),
                FlatButton(
                    onPressed: () {
                        Navigator.of(context).pop("Votes");
                    },
                    child: Text("Votes"),
                ),
            ]),
        );
    }
}

class SetVotes extends StatefulWidget {
    SetVotes({Key key}): super(key: key);

    @override
    SetVotesState createState() { return new SetVotesState(); }
}

class SetVotesState extends State<SetVotes> {
    TextEditingController text_controller;

    @override
    void initState() {
        super.initState();
        text_controller = TextEditingController(text: "");
    }

    @override
    Widget build(BuildContext context) {
        return new AlertDialog(
            title: const Text('Set # of Votes:'),
            content: TextField(controller: text_controller,
                toolbarOptions: ToolbarOptions(
                    copy: false, cut: false, paste: false, selectAll: false
                ),
                autofocus: true,
                maxLength: 22),
            actions: <Widget>[
                Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                        FlatButton(
                            onPressed: () {
                                Navigator.of(context).pop(null);
                            },
                            child: Text("Cancel"),
                        ),
                        FlatButton(
                            onPressed: () {
                                Navigator.of(context).pop(text_controller.text);
                            },
                            child: Text("Set Count"),
                        ),
                    ]
                ),
            ],
        );
    }
}
