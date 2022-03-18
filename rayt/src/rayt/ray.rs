use crate::rayt::*;

#[derive(Debug,Clone,Copy)]


/**
 * 始点o -(d)->へ向かう光
 * at(t)で時間t後の位置をもとめられる
 */
pub struct Ray{
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin:Point3,direction:Vec3) -> Self{
        Self {origin,direction}
    }

    pub fn at(&self,t:f64) -> Point3{
        self.origin + t * self.direction
    }
}