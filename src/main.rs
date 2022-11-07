use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("John".to_string()));
    commands.spawn().insert(Person).insert(Name("Jane".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello, {}!", name.0);
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(add_people)
    .add_system(hello_world)
    .add_system(greet_people)
    .run();
}