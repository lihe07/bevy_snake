use crate::grid::PositionInGrid;
use bevy::{prelude::*, utils::HashSet};

#[derive(Component)]
#[require(Sprite, PositionInGrid)]
pub struct Food;

#[derive(Resource, Clone)]
pub struct FoodPlugin {
    pub food_num: usize,
}

fn can_run_spawn(remaining_foods: Query<&Food>) -> bool {
    remaining_foods.iter().count() == 0
}

fn spawn(
    mut commands: Commands,
    config: Res<FoodPlugin>,
    snake_parts_pos: Query<&PositionInGrid, With<crate::SnakePart>>,
    snake_head_pos: Query<&PositionInGrid, With<crate::SnakeHead>>,
) {
    let food_sprite = Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::default());

    let mut snake_pos = HashSet::from_iter(snake_parts_pos.iter());
    snake_pos.insert(snake_head_pos.iter().next().unwrap());

    for _ in 0..config.food_num {
        // Generate a random non-zero x
        let mut pos = PositionInGrid { x: 0, y: 0 };
        while snake_pos.contains(&pos) {
            // Range is from -5 to 4
            let x = rand::random::<i32>() % 5;
            let y = rand::random::<i32>() % 5;
            pos = PositionInGrid { x, y };
        }

        info!("Spawning food at ({}, {})", pos.x, pos.y);

        commands.spawn((Food, food_sprite.clone(), pos));
        // Todo: no overlapping foods
    }
}

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.clone());
        app.add_systems(
            Update,
            spawn.run_if(can_run_spawn).in_set(crate::GamePlaySet),
        );
    }
}
