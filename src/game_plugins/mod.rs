pub mod camera;
pub mod player;

use crate::game_plugins::{camera::CameraPlugin, player::PlayerPlugin};
use crate::prelude::*;

pub struct InGamePlugins;

impl Plugin for InGamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, CameraPlugin, HanabiPlugin))
            .add_systems(OnEnter(GameStates::Menu), setup);
    }
}

fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(4.0, 4.0, 4.0, 0.0));

    // Keep the size large so we can more visibly see the particles for longer, and
    // see the effect of alpha blending.
    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec3::ONE);
    size_gradient1.add_key(0.1, Vec3::ONE);
    size_gradient1.add_key(1.0, Vec3::ZERO);

    let writer = ExprWriter::new();

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.2)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(2.).uniform(writer.lit(3.)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add constant downward acceleration to simulate gravity
    let accel = writer.lit(Vec3::Y * -8.).expr();
    let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(5.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(600.).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Give a bit of variation by randomizing the initial speed
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(20.) + writer.lit(60.)).expr(),
    };

    let effect = effects.add(
        EffectAsset::new(4048, SpawnerSettings::rate(128.0.into()), writer.finish())
            .with_name("firework")
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
            .update(update_accel)
            // Note: we (ab)use the ColorOverLifetimeModifier to set a fixed color hard-coded in the
            // render shader, without having to store a per-particle color. This is an optimization.
            .render(ColorOverLifetimeModifier::new(color_gradient1))
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient1,
                screen_space_size: false,
            }),
    );
    // Spawn an instance of the particle effect, and override its Z layer to
    // be above the reference white square previously spawned.
    commands.spawn((ParticleEffect::new(effect), Name::new("effect:2d")));
}
