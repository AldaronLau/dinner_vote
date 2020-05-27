class Meal {
  String title;
  String subtitle;
  String who;

  Meal(this.title, this.subtitle, this.who);

  Map<String, dynamic> toMap() {
    var map = <String, dynamic>{
      'title': title,
      'subtitle': subtitle,
      'who': who,
    };
    return map;
  }

  Meal.fromMap(Map<String, dynamic> map) {
    title = map['title'];
    subtitle = map['subtitle'];
    who = map['who'];
  }
}
