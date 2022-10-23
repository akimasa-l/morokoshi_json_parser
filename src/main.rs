use morokoshi_json_parser;
fn main() {
    let json = r#"  {
        "squadName": "Super hero squad",
        "homeTown": "Metro City",
        "secretBase": "Super tower",
        "active": true,
        "members": [
          {
            "name": "Molecule Man",
            "secretIdentity": "Dan Jukes",
            "powers": [
              "Radiation resistance",
              "Turning tiny",
              "Radiation blast"
            ]
          },
          {
            "name": "Madame Uppercut",
            "secretIdentity": "Jane Wilson",
            "powers": [
              "Million tonne punch",
              "Damage resistance",
              "Superhuman reflexes"
            ]
          },
          {
            "name": "Eternal Flame",
            "secretIdentity": "Unknown",
            "powers": [
              "Immortality",
              "Heat Immunity",
              "Inferno",
              "Teleportation",
              "Interdimensional travel"
            ]
          }
        ]
      }"#;
    let mut parser = morokoshi_json_parser::morokoshi::MorokoshiJsonParser::new(String::from(json));
    let result = parser.parse();
    println!("{:?}", result);
}
