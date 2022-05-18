use bevy::prelude::*;

pub struct FlockStatePlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Main,
    Settings,
}

impl Plugin for FlockStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Main);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
