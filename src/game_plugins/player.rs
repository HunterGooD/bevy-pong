use crate::prelude::*;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Playing), spawn_player)
            .add_systems(
                Update,
                move_player
                    .run_if(in_state(GameStates::Playing))
                    .run_if(in_state(MenuStates::Disable)),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        Sprite::from_image(textures.bevy.clone()),
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        StateScoped(GameStates::Playing),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    intent: Res<MovementIntent>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let speed = 200.;
    if intent.0 == Vec2::ZERO {
        return;
    }

    let movement = Vec3::new(
        intent.0.x * speed * time.delta_secs(),
        intent.0.y * speed * time.delta_secs(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
