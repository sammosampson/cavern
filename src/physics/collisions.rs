use crate::prelude::*;

pub struct StaticRigidBody;

pub struct CollisionBox(Dimensions);

impl From<Dimensions> for CollisionBox {
    fn from(from: Dimensions) -> Self {
        Self(from)
    }
}

impl Deref for CollisionBox {
    type Target = Dimensions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AABB {
    position: Vector,
    dimensions: Dimensions
}

impl AABB {
    pub fn from_centre_and_size(centre_position: Vector, dimensions: Dimensions) -> Self {
        Self{
            position: centre_position - (dimensions / 2.0).into(),
            dimensions
        }
    }

    pub fn sweep_check_collision(&self, to_check: &AABB, velocity: &Vector) -> (f32, Vector) {
        let mut delta_entry = Vector::default(); 
        let mut delta_exit = Vector::default(); 

        if velocity.x > 0.0 { 
            delta_entry.x = to_check.position.x - (self.position.x + self.dimensions.width);  
            delta_exit.x = (to_check.position.x + to_check.dimensions.width) - self.position.x;
        } else { 
            delta_entry.x = (to_check.position.x + to_check.dimensions.width) - self.position.x;  
            delta_exit.x = to_check.position.x - (self.position.x + self.dimensions.width);  
        } 

        if velocity.y > 0.0 { 
            delta_entry.y = to_check.position.y - (self.position.y + self.dimensions.height);  
            delta_exit.y = (to_check.position.y + to_check.dimensions.height) - self.position.y;  
        } else { 
            delta_entry.y = (to_check.position.y + to_check.dimensions.height) - self.position.y;  
            delta_exit.y = to_check.position.y - (self.position.y + self.dimensions.height);  
        }

        let mut entry = Vector::default(); 
        let mut exit = Vector::default(); 

        if velocity.x == 0.0 { 
            entry.x = -INFINITY; 
            exit.x = INFINITY; 
        } else { 
            entry.x = delta_entry.x / velocity.x; 
            exit.x = delta_exit.x / velocity.x; 
        } 

        if velocity.y == 0.0 { 
            entry.y = -INFINITY; 
            exit.y = INFINITY; 
        } else { 
            entry.y = delta_entry.y / velocity.y; 
            exit.y = delta_exit.y / velocity.y; 
        }

        let entry_time = max(entry.x, entry.y); 
        let exit_time = min(exit.x, exit.y);
        let mut normal = Vector::default();

        if entry_time > exit_time
            || entry.x < 0.0 && entry.y < 0.0
            || entry.x > 1.0
            || entry.y > 1.0 { 
            return (1.0, normal); 
        } else { 
            if entry.x > entry.y { 
                if delta_entry.x < 0.0 { 
                    normal = Vector::new(1.0, 0.0); 
                } else { 
                    normal = Vector::new(-1.0, 0.0); 
                } 
            } else { 
                if delta_entry.y < 0.0 { 
                    normal = Vector::new(0.0, 1.0); 
                } else { 
                    normal = Vector::new(0.0, -1.0); 
                } 
            }
        }

        return (entry_time, normal);
    }
}

#[test]
fn it_collides_horizontally() {
    let sut = AABB::from_centre_and_size(Vector::new(100.0, 100.0), Dimensions::from(64.0));
    let overlaps = AABB::from_centre_and_size(Vector::new(165.0, 100.0), Dimensions::from(16.0));
    
    let (time, normal) = sut.sweep_check_collision(
        &overlaps, 
        &Vector::new(10.0, 0.0)
    );

    assert!(normal.x == -1.0 && normal.y == 0.0, "normal = {:?}", normal);
    assert!(time == 0.1, "time = {}", time);
}

#[test]
fn it_collides_horizontally_having_passed_through() {
    let sut = AABB::from_centre_and_size(Vector::new(100.0, 100.0), Dimensions::from(64.0));
    let overlaps = AABB::from_centre_and_size(Vector::new(165.0, 100.0), Dimensions::from(16.0));
    
    let (time, normal) = sut.sweep_check_collision(
        &overlaps, 
        &Vector::new(100.0, 0.0)
    );

    assert!(normal.x == -1.0 && normal.y == 0.0, "normal = {:?}", normal);
    assert!(time == 0.01, "time = {}", time);
}

#[test]
fn it_collides_horizontally_in_reverse() {
    let sut = AABB::from_centre_and_size(Vector::new(117.0, 100.0), Dimensions::from(64.0));
    let overlaps = AABB::from_centre_and_size(Vector::new(100.0, 100.0), Dimensions::from(16.0));
    
    let (time, normal) = sut.sweep_check_collision(
        &overlaps, 
        &Vector::new(-10.0, 0.0)
    );

    assert!(normal.x == 1.0 && normal.y == 0.0, "normal = {:?}", normal);
    assert!(time == 0.1, "time = {}", time);
}

#[test]
fn it_collides_vertically() {
    let sut = AABB::from_centre_and_size(Vector::new(100.0, 100.0), Dimensions::from(64.0));
    let overlaps = AABB::from_centre_and_size(Vector::new(100.0, 165.0), Dimensions::from(16.0));
    
    let (time, normal) = sut.sweep_check_collision(
        &overlaps, 
        &Vector::new(0.0, 10.0)
    );

    assert!(normal.x == 0.0 && normal.y == -1.0, "normal = {:?}", normal);
    assert!(time == 0.1, "time = {}", time);
}

#[test]
fn it_collides_vertically_having_passed_through() {
    let sut = AABB::from_centre_and_size(Vector::new(100.0, 100.0), Dimensions::from(64.0));
    let overlaps = AABB::from_centre_and_size(Vector::new(100.0, 165.0), Dimensions::from(16.0));
    
    let (time, normal) = sut.sweep_check_collision(
        &overlaps, 
        &Vector::new(0.0, 100.0)
    );

    assert!(normal.x == 0.0 && normal.y == -1.0, "normal = {:?}", normal);
    assert!(time == 0.01, "time = {}", time);
}

#[test]
fn it_collides_vertically_in_reverse() {
    let sut = AABB::from_centre_and_size(Vector::new(100.0, 117.0), Dimensions::from(64.0));
    let overlaps = AABB::from_centre_and_size(Vector::new(100.0, 100.0), Dimensions::from(16.0));
    
    let (time, normal) = sut.sweep_check_collision(
        &overlaps, 
        &Vector::new(0.0, -10.0)
    );

    assert!(normal.x == 0.0 && normal.y == 1.0, "normal = {:?}", normal);
    assert!(time == 0.1, "time = {}", time);
}

#[test]
fn it_collides_horizontally_and_vertically() {
    let sut = AABB::from_centre_and_size(Vector::new(100.0, 100.0), Dimensions::from(64.0));
    let overlaps = AABB::from_centre_and_size(Vector::new(165.0, 165.0), Dimensions::from(16.0));
    
    let (time, normal) = sut.sweep_check_collision(
        &overlaps, 
        &Vector::new(10.0, 10.0)
    );

    assert!(normal.x == 0.0 && normal.y == -1.0, "normal = {:?}", normal);
    assert!(time == 0.1, "time = {}", time);
}

#[test]
fn it_does_not_collide() {
    let sut = AABB::from_centre_and_size(Vector::new(100.0, 90.0), Dimensions::from(64.0));
    let overlaps = AABB::from_centre_and_size(Vector::new(100.0, 165.0), Dimensions::from(16.0));
    
    let (time, normal) = sut.sweep_check_collision(
        &overlaps, 
        &Vector::new(0.0, 10.0)
    );

    assert!(normal.x == 0.0 && normal.y == 0.0, "normal = {:?}", normal);
    assert!(time == 1.0, "time = {}", time);
}
