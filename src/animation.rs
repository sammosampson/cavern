use crate::prelude::*;

pub fn create_animation(total_animation_time: f32, started_at_time: f32) -> Animation {
    Animation::new(total_animation_time, started_at_time)
}

pub fn remove_animation(buffer: &mut CommandBuffer, entity: Entity) {
    buffer.remove_component::<Animation>(entity)
}

#[derive(Default, Clone, Debug)]
pub struct Animation {
    pub total_animation_time: f32,
    pub started_at_time: f32,
    pub frames: SmallVec<[Texture; 10]>
}

impl Animation {
    pub fn new(total_animation_time: f32, started_at_time: f32) -> Self {
        Self {
            total_animation_time,
            started_at_time,
            frames: SmallVec::default()
        }
    }

    pub fn add_frame(&mut self, frame: Texture) {
        self.frames.push(frame);
    }

    pub fn get_frame(&self, total_game_time: f32) -> Option<usize> {
        let time_into_animation = total_game_time - self.started_at_time;
        
        if time_into_animation > self.total_animation_time {
            return None;
        }
    
        let time_per_frame = self.total_animation_time / self.frames.len() as f32;
    
        Some((time_into_animation / time_per_frame) as usize)
    }
    
    pub fn get_frame_texture(&self, frame: usize) -> Texture {
        self.frames
            .get(frame)
            .expect("No animation frame found")
            .clone()
    }
    
}