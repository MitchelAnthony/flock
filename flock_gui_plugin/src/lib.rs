use bevy::prelude::*;

pub struct FlockGuiPlugin;

impl Plugin for FlockGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<events::Tick>()
            .insert_resource(resources::Test(0))
            .add_startup_system(systems::setup)
            .add_startup_system(systems::hello_world)
            .add_system(systems::show_count)
            .add_system(systems::handle_tick)
            .add_system(systems::button_interaction);
    }
}

mod systems {
    use super::*;
    use crate::resources::Test;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn_bundle(UiCameraBundle::default());
        commands.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Counter: 0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        });

        // Button
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    // center button
                    margin: Rect::all(Val::Auto),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..default()
                });
            });
    }

    pub fn hello_world() {
        println!("hello world!");
    }

    pub fn button_interaction(
        mut query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
        mut tick_writer: EventWriter<events::Tick>,
    ) {
        for interaction in query.iter_mut() {
            if *interaction == Interaction::Clicked {
                tick_writer.send(events::Tick);
            }
        }
    }

    pub fn show_count(test_resource: Res<Test>, mut query: Query<&mut Text>) {
        for mut text in query.iter_mut() {
            if !text.sections[0].value.contains("Counter") {
                continue;
            }

            text.sections[0].value = format!("Counter: {}", test_resource.0);
        }
    }

    pub fn handle_tick(
        mut tick_reader: EventReader<events::Tick>,
        mut test_resource: ResMut<Test>,
    ) {
        for _ in tick_reader.iter() {
            test_resource.0 += 1;
        }
    }
}

mod resources {
    #[derive(Debug)]
    pub struct Test(pub usize);
}

mod components {
    use super::*;

    #[derive(Component)]
    pub struct Label(String);
}

mod events {
    pub struct Tick;
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
