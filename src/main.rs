use bevy::{
    prelude::{App, Commands, Component, Plugin, Query, Res, ResMut, Resource, With},
    time::{Time, Timer, TimerMode},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn _hello_world() {
    println!("Hello World");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Name("Elaina Proctor".to_string()), Person));

    // commands.spawn((Person, Name("Elaina Proctor".to_string())));
    // commands.spawn((Person, Name("Renzo Hume".to_string())));
    // commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            // .add_system(hello_world)
            .add_system(greet_people);
    }
}
