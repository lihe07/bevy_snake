use bevy::prelude::*;
mod food;
mod grid;
mod ui;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, States)]
enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GamePlaySet;

const LEFT: Vec2 = Vec2::new(-1.0, 0.0);
const RIGHT: Vec2 = Vec2::new(1.0, 0.0);
const UP: Vec2 = Vec2::new(0.0, 1.0);
const DOWN: Vec2 = Vec2::new(0.0, -1.0);

#[derive(Component)]
#[require(grid::PositionInGrid, Sprite)]
struct SnakeHead {
    direction: Vec2,
}

#[derive(Component)]
#[require(Camera2d)]
struct MyCamera;

#[derive(Resource)]
struct SnakeParts {
    parts: Vec<Entity>,
}

#[derive(Default, Component)]
#[require(grid::PositionInGrid, Sprite)]
struct SnakePart;

#[derive(Component)]
#[require(SnakePart)]
/// This is for spawning food
struct SnakePseudoPart;

fn snake_setup(mut commands: Commands, mut parts: ResMut<SnakeParts>) {
    info!("Setting up snake");

    // Spawn camera
    commands.spawn(MyCamera);

    // Spawn snake head
    let sprite = Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::new(10.0, 10.0));
    commands.spawn((
        SnakeHead { direction: UP },
        Transform::default(),
        grid::PositionInGrid { x: 0, y: 0 },
        sprite,
    ));
    let init_len = 5;

    // Spawn snake body
    let sprite = Sprite::from_color(Color::srgba(0.0, 0.9, 0.0, 0.5), Vec2::new(10.0, 10.0));

    for i in 1..init_len {
        let part = commands
            .spawn((
                SnakePart,
                Transform::default(),
                grid::PositionInGrid { x: 0, y: -i },
                sprite.clone(),
            ))
            .id();
        parts.parts.push(part);
    }
    let part = commands
        .spawn((
            SnakePart,
            SnakePseudoPart,
            Transform::default(),
            grid::PositionInGrid { x: 0, y: -init_len },
            Sprite::from_color(Color::srgba(0.0, 0.9, 0.0, 0.0), Vec2::new(10.0, 10.0)),
        ))
        .id();
    parts.parts.push(part);
}

fn fix_pos(pos: &mut grid::PositionInGrid) {
    // pos.x = pos.x.clamp(-5, 5);
    // pos.y = pos.y.clamp(-5, 5);
    // If the snake goes out of bounds, wrap it around
    if pos.x < -5 {
        pos.x = 4;
    }
    if pos.x > 4 {
        pos.x = -5;
    }
    if pos.y < -5 {
        pos.y = 4;
    }
    if pos.y > 4 {
        pos.y = -5;
    }
}

#[derive(Resource)]
struct MovementTimer(Timer);

fn snake_movement(
    mut timer: ResMut<MovementTimer>,
    time: Res<Time>,
    mut query: Query<(&SnakeHead, &mut grid::PositionInGrid), Without<SnakePart>>,
    mut query_parts: Query<&mut grid::PositionInGrid, With<SnakePart>>,

    mut next_state: ResMut<NextState<GameState>>,

    parts: Res<SnakeParts>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    timer.0.reset();

    let (head, mut pos) = query.iter_mut().next().unwrap();

    let mut last_pos = pos.clone();

    pos.x += head.direction.x as i32;
    pos.y += head.direction.y as i32;

    let head_new_pos = pos.clone();

    fix_pos(&mut pos);

    // Move the body parts, each move to front
    for part in parts.parts.iter() {
        let mut my_part = query_parts.get_mut(*part).unwrap();

        // Modify the position of the part to the last position of the previous part
        let my_part_old_pos = my_part.clone();
        my_part.x = last_pos.x;
        my_part.y = last_pos.y;

        last_pos = my_part_old_pos;
    }

    // Check if game over

    // Skip checking the pseudo part
    for i in 1..(parts.parts.len() - 1) {
        let my_part = query_parts.get(parts.parts[i]).unwrap();

        if my_part.x == head_new_pos.x && my_part.y == head_new_pos.y {
            info!("Game over");
            // Set game state
            next_state.set(GameState::GameOver);
            return;
        }
    }
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut SnakeHead>,
    mut timer: ResMut<MovementTimer>,
) {
    for mut head in query.iter_mut() {
        let mut flag = false;
        if keyboard_input.just_pressed(KeyCode::KeyH) && head.direction != RIGHT {
            head.direction = LEFT;
            flag = true;
        } else if keyboard_input.just_pressed(KeyCode::KeyL) && head.direction != LEFT {
            head.direction = RIGHT;
            flag = true;
        } else if keyboard_input.just_pressed(KeyCode::KeyK) && head.direction != DOWN {
            head.direction = UP;
            flag = true;
        } else if keyboard_input.just_pressed(KeyCode::KeyJ) && head.direction != UP {
            head.direction = DOWN;
            flag = true;
        }

        if flag {
            // Directly finishes the timer
            let duration = timer.0.duration();
            timer.0.tick(duration);
        }
    }
}

fn check_food(
    food_pos: Query<(Entity, &grid::PositionInGrid), With<food::Food>>,
    head_pos: Query<&grid::PositionInGrid, With<SnakeHead>>,
    pseudo_part_pos: Query<&grid::PositionInGrid, With<SnakePseudoPart>>,
    mut commands: Commands,
    mut parts: ResMut<SnakeParts>,
) {
    let pseudo_part_pos = pseudo_part_pos.iter().next().unwrap();

    let head_pos = head_pos.iter().next().unwrap();

    for (entity, food) in food_pos.iter() {
        if head_pos.x == food.x && head_pos.y == food.y {
            info!("Eating food");
            // Destroy the food
            commands.entity(entity).despawn();
            // Increase the length of the snake
            let pseudo_part = parts.parts.pop().unwrap();

            let part = commands
                .spawn((
                    SnakePart,
                    Transform::default(),
                    pseudo_part_pos.clone(),
                    Sprite::from_color(Color::srgba(0.0, 0.9, 0.0, 0.5), Vec2::new(10.0, 10.0)),
                ))
                .id();

            parts.parts.push(part);
            parts.parts.push(pseudo_part);
        }
    }
}

struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        // Add plugins
        app.add_plugins((grid::GridPlugin, ui::MyUiPlugin));

        app.insert_resource(SnakeParts { parts: Vec::new() });

        app.insert_resource(MovementTimer(Timer::from_seconds(0.5, TimerMode::Once)));

        app.configure_sets(Update, GamePlaySet.run_if(in_state(GameState::Running)));

        app.add_systems(Startup, snake_setup);
        app.add_systems(
            Update,
            (check_food, handle_input, snake_movement).in_set(GamePlaySet),
        );

        // Foods
        app.add_plugins(food::FoodPlugin { food_num: 3 });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SnakePlugin)
        .run();
}
