class Meal {
  int id;
  String title;
  String subtitle;
  String description;
  String tags;

  Meal(this.title, this.subtitle, this.description);

  Map<String, dynamic> toMap() {
    var map = <String, dynamic>{
      'title': title,
      'description': description,
      'subtitle': subtitle
    };
    if (id != null) {
      map['id'] = id;
    }
    return map;
  }

  Meal.fromMap(Map<String, dynamic> map) {
    id = map['id'];
    title = map['title'];
    subtitle = map['subtitle'];
    description = map['description'];
  }

  bool hasTag(String tag) {
    return (tag == 'Chicken');
  }
}
