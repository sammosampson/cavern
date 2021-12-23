use crate::prelude::*;

#[system(for_each)]
pub fn exit_if_requested(
    event: &SystemEvent, 
    #[resource] game_state: &mut GameState,
) {
    match event {
        SystemEvent::CloseRequested => game_state.transition_to(GameStatus::Exiting),
        _ => {}
    }
}

#[system(simple)]
pub fn transition_state_to_playing(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    buffer: &mut CommandBuffer,
) {
    if game_state.has_entered() {
        return;
    }

    match game_state.previous_status() {
        GameStatus::Scoring(_) => {
            add_ball(buffer);
        },
        GameStatus::None => {
            add_table(buffer);
            add_ball(buffer);
            add_bat0(buffer);
            add_bat1(buffer);
        }
        _ => {},
    }
    
    game_state.enter(game_timer.total_game_time());
}

#[system(simple)]
#[read_component(Ball)]
pub fn transition_state_to_scored(
    #[resource] game_state: &mut GameState,
    #[resource] game_timer: &mut GameTimer,
    buffer: &mut CommandBuffer,
    world: &SubWorld
) {  
    if game_state.has_entered() {
        if game_timer.total_game_time() - game_state.entered_on() >= 3.0 {
            game_state.transition_to(GameStatus::Playing)
        }
        return;
    }

    match game_state.previous_status() {
        GameStatus::Playing => {
            remove_ball(buffer, world);
        },
        _ => {},
    }
    
    game_state.enter(game_timer.total_game_time());
}

fn add_table(buffer: &mut CommandBuffer) {
    buffer.push((
        Texture(TextureResources::Table), 
        Layer(0),
        Position(centre_screen()),
        WorldEntityId::from("arena")
    ));
}

fn add_bat0(buffer: &mut CommandBuffer) {
    add_bat(buffer,0,TextureResources::Bat00,BAT_POSITION_OFFSET,"player1");
}

fn add_bat1(buffer: &mut CommandBuffer) {
    add_bat(buffer,1,TextureResources::Bat10,SCREEN_WIDTH - BAT_POSITION_OFFSET,"player2");
}

fn add_bat(buffer: &mut CommandBuffer, index: u8, texture: TextureResources, x: f32, name: &str) {
    buffer.push((
        Bat(index),
        Texture(texture), 
        Layer(1), 
        Position(Vector::new(x, HALF_SCREEN_HEIGHT)), 
        MaximumVelocity(140.0),
        Heading::default(),
        WorldEntityId::from(name),
        Player
    ));
}

fn add_ball(buffer: &mut CommandBuffer) {
    buffer.push((
        Ball,
        Texture(TextureResources::Ball), 
        Layer(1), 
        Position(centre_screen()), 
        MaximumVelocity(150.0),
        Heading(Angle::from_degrees(135.0).into()),
        WorldEntityId::from("ball")
    ));
}

fn remove_ball(buffer: &mut CommandBuffer, world: &SubWorld) {
    <Entity>::query()
        .filter(component::<Ball>())
        .iter(world)
        .for_each(|entity| buffer.add_component(*entity, Remove));
}

fn centre_screen() -> Vector {
    Vector::new(HALF_SCREEN_WIDTH, HALF_SCREEN_HEIGHT)
}
