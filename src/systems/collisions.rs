use crate::prelude::*;

#[system(simple)]
#[read_component(Position)]
#[read_component(CollisionBox)]
#[read_component(Velocity)]
#[read_component(StaticRigidBody)]
pub fn collision(world: &mut SubWorld, buffer: &mut CommandBuffer) {
    let static_bodies: Vec<AABB> = <(&Position, &CollisionBox)>::query()
        .filter(component::<StaticRigidBody>())
        .iter(world)
        .map(|(position, collision_box)| AABB::from_centre_and_size(**position, **collision_box))
        .collect();

    let dynamic_bodies: Vec<(Entity, AABB, Vector)> = <(Entity, &Position, &CollisionBox, &Velocity)>::query()
        .iter(world)
        .map(|(entity, position, collision_box, velocity)| (*entity, AABB::from_centre_and_size(**position, **collision_box), **velocity))
        .collect();


    for (dynamic_entity, dynamic_aabb, velocity) in dynamic_bodies.iter() {
        for static_aabb in static_bodies.iter() {
            let (collision_time, collision_normal) = dynamic_aabb.sweep_check_collision(static_aabb, velocity);
            
            if collision_time < 1.0 {    
                let mut new_velocity = Velocity::default();
                
                if collision_normal.y != 0.0 {
                    *new_velocity = Vector::new(velocity.x, velocity.y * collision_time);
                }

                if collision_normal.x != 0.0 {
                    *new_velocity = Vector::new(velocity.x * collision_time, velocity.y);
                }

                buffer.add_component(*dynamic_entity, new_velocity)
            }
        }    
    }
}