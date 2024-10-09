use bevy::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct SpriteAnimation {
    frames: Vec<Handle<Image>>,
    current_frame: usize,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Load all sprite frames
    let mut frames = Vec::new();
    for i in 0..20 {
        let path = format!("2BlueWizardWalk/Chara_BlueWalk{:05}.png", i);
        frames.push(asset_server.load(path));
    }

    commands.spawn((
        SpriteBundle {
            texture: frames[0].clone(),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        SpriteAnimation {
            frames,
            current_frame: 0,
        },
        AnimationTimer(Timer::new(Duration::from_millis(50), TimerMode::Repeating)),
    ));
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut SpriteAnimation, &mut Handle<Image>)>,
) {
    for (mut timer, mut animation, mut texture) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
            *texture = animation.frames[animation.current_frame].clone();
        }
    }
}