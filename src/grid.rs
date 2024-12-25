use bevy::prelude::*;

#[derive(Resource)]
pub struct GridConfig {
    pub width: usize,
    pub height: usize,
}

impl Default for GridConfig {
    fn default() -> Self {
        GridConfig {
            width: 10,
            height: 10,
        }
    }
}

#[derive(Default, Clone, Component, Hash, PartialEq, Eq, Debug)]
#[require(Transform)]
pub struct PositionInGrid {
    pub x: i32,
    pub y: i32,
}

fn update_transform_from_pos(
    mut query: Query<(&PositionInGrid, &mut Transform, &mut Sprite)>,
    config: Res<GridConfig>,
    window: Single<&Window>,
) {
    let window_width = window.width();
    let window_height = window.height();

    let x_step = window_width / config.width as f32;
    let y_step = window_height / config.height as f32;

    for (pos, mut transform, mut sprite) in query.iter_mut() {
        transform.translation = Vec3::new(
            (pos.x as f32 + 0.5) * x_step,
            (pos.y as f32 + 0.5) * y_step,
            0.0,
        );
        // transform.scale = Vec3::new(x_step, y_step, 1.0);
        // Change the size of the sprite to match the grid cell
        sprite.custom_size = Some(Vec2::new(x_step, y_step) * 0.8);
    }
}

pub struct GridPlugin;

fn startup(mut window: Single<&mut Window>) {
    // Set window size to 500x500
    window.resolution.set(500.0, 500.0);
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridConfig>();
        app.add_systems(Startup, startup);
        app.add_systems(Update, update_transform_from_pos);
    }
}
