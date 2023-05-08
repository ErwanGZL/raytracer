use crate::{
    math::Vector3D,
    primitive::{self, Intersection, Primitive},
};

#[derive(Debug)]
pub struct Ray {
    origin: Vector3D,
    direction: Vector3D,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Vector3D::default(),
            direction: Vector3D::default(),
        }
    }
}

impl Ray {
    pub fn new(origin: Vector3D, direction: Vector3D) -> Self {
        Self { origin, direction }
    }
    pub fn from_points(origin: Vector3D, target: Vector3D) -> Self {
        Self {
            origin,
            direction: (target - origin).normalize(),
        }
    }
    pub fn origin(&self) -> Vector3D {
        self.origin
    }
    pub fn direction(&self) -> Vector3D {
        self.direction
    }
    pub fn intersect<'a>(&self, primitive: &'a Vec<Box<dyn Primitive>>) -> Option<Intersection<'a>> {
        let mut v: Vec<Intersection<'a>> = Vec::new();
        for primitive in primitive {
            v.extend(primitive.hits(self));
        }
        match v
            .iter()
            .map(|i| (i.point().distance(self.origin), i))
            .min_by(|l, i| l.0.total_cmp(&i.0))
        {
            Some((_, i)) => Some(*i),
            None => None,
        }
    }
}
