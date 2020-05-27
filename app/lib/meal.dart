class Meal {
  String title;
  String subtitle;

  Meal(this.title, this.subtitle);

  Map<String, dynamic> toMap() {
    var map = <String, dynamic>{
      'title': title,
      'subtitle': subtitle
    };
    return map;
  }

  Meal.fromMap(Map<String, dynamic> map) {
    title = map['title'];
    subtitle = map['subtitle'];
  }
}
