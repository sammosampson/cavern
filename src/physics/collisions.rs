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
    pub fn sweep_check_collision(&self, to_check: &AABB, velocity: &Vector) -> (f32, Vector) {
        let mut inv_entry = Vector::default(); 
        let mut inv_exit = Vector::default(); 

        if velocity.x > 0.0 { 
            inv_entry.x = to_check.position.x - (self.position.x + self.dimensions.width);  
            inv_exit.x = (to_check.position.x + to_check.dimensions.width) - self.position.x;
        } else { 
            inv_entry.x = (to_check.position.x + to_check.dimensions.width) - self.position.x;  
            inv_exit.x = to_check.position.x - (self.position.x + self.dimensions.width);  
        } 

        if velocity.y > 0.0 { 
            inv_entry.y = to_check.position.y - (self.position.y + self.dimensions.height);  
            inv_exit.y = (to_check.position.y + to_check.dimensions.height) - self.position.y;  
        } else { 
            inv_entry.y = (to_check.position.y + to_check.dimensions.height) - self.position.y;  
            inv_exit.y = to_check.position.y - (self.position.y + self.dimensions.height);  
        }

        let mut entry = Vector::default(); 
        let mut exit = Vector::default(); 

        if velocity.x == 0.0 { 
            entry.x = INFINITY; 
            exit.x = INFINITY; 
        } else { 
            entry.x = inv_entry.x / velocity.x; 
            exit.x = inv_exit.x / velocity.x; 
        } 

        if velocity.y == 0.0 { 
            entry.y = INFINITY; 
            exit.y = INFINITY; 
        } else { 
            entry.y = inv_entry.y / velocity.y; 
            exit.y = inv_exit.y / velocity.y; 
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
                if inv_entry.x < 0.0 { 
                    normal = Vector::new(1.0, 0.0); 
                } else { 
                    normal = Vector::new(-1.0, 0.0); 
                } 
            } else { 
                if inv_entry.y < 0.0 { 
                    normal = Vector::new(0.0, 1.0); 
                } else { 
                    normal = Vector::new(0.0, -1.0); 
                } 
            }
        }

        return (entry_time, normal);
    }
}

impl From<(Vector, Dimensions)> for AABB {
    fn from(from: (Vector, Dimensions)) -> Self {
        Self{ position: from.0, dimensions: from.1 }
    }
}