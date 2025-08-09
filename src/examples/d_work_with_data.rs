use std::fmt;

pub fn main() {
    let people = vec![
        Person::new("Alice", 28, 165.0, vec![Hobby::Reading, Hobby::Gardening, Hobby::Cooking], "New York"),
        Person::new("Bob", 35, 180.5, vec![Hobby::Cycling, Hobby::Running], "London"),
        Person::new("Charlie", 22, 175.2, vec![Hobby::Gaming, Hobby::Photography, Hobby::Traveling], "Berlin"),
        Person::new("Diana", 40, 168.4, vec![Hobby::Cooking, Hobby::Painting, Hobby::Reading], "Paris"),
        Person::new("Ethan", 31, 182.0, vec![Hobby::Fishing, Hobby::Hiking], "Oslo"),
        Person::new("Fiona", 27, 160.8, vec![Hobby::Singing, Hobby::Dancing, Hobby::PlayingPiano], "Tokyo"),
        Person::new("George", 50, 170.0, vec![Hobby::Knitting, Hobby::Reading, Hobby::PlayingGuitar], "Sydney"),
        Person::new("Hannah", 29, 164.7, vec![Hobby::Swimming, Hobby::Traveling], "Barcelona"),
        Person::new("Ian", 36, 178.0, vec![Hobby::Chess, Hobby::BirdWatching], "Toronto"),
        Person::new("Julia", 33, 158.5, vec![Hobby::Writing, Hobby::Photography, Hobby::Cycling], "Rome"),
    ];

    for (i, person) in people.iter().enumerate() {
        println!("Person {}: {}", i + 1, person);
    }

    let readers = people.iter().filter(|p| p.hobbies.contains(&Hobby::Reading))
                                       .map(|p| p.name.clone())
                                       .collect::<Vec<_>>()
                                       .join(", ");
    println!("\nFollowing people like to read: {}", readers);

    let smallest = people.iter().min_by(|a, b| a.height.partial_cmp(&b.height).unwrap())
                         .unwrap();
    println!("\n{} is the smallest person of the list.", smallest.name);

    let sum_age: u32 = people.iter().map(|p| p.age as u32).sum();
    let average_age = sum_age as f32/ people.len() as f32;
    println!("\nThe peoples average age is {}", average_age);

}


#[derive(Debug, PartialEq)]
enum Hobby { Reading, Writing, Cooking, Gardening, Painting, Photography, Traveling,
    Hiking, Swimming, Cycling, Running, PlayingGuitar, PlayingPiano, Dancing, Singing,
    Fishing, Gaming, Knitting, Chess, BirdWatching, }

impl fmt::Display for Hobby {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Person {
    name: String, 
    age: u8,
    height: f32,
    hobbies: Vec<Hobby>,
    city: String,
}

impl Person {
    fn new(name: &str, age: u8, height: f32, hobbies: Vec<Hobby>, city: &str) -> Self {
        Self { name: name.to_string(), age, height, hobbies, city: city.to_string(), }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hobbies_str = self.hobbies.iter()
                              .map(|h| h.to_string())
                              .collect::<Vec<_>>()
                              .join(", ");
        write!( f, "{} ({} years, {:.1} cm) from {} likes {}.",
            self.name, self.age, self.height, self.city, hobbies_str )
    }
}

