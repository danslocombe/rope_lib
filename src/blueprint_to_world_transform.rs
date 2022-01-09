use crate::rope::Vec2;

pub trait BlueprintToWorldTransform {
    fn transform(&self, pos : Vec2) -> Vec2;
}

pub struct LinearTransform {
    offset : Vec2,
    tangent : Vec2,
    normal : Vec2,
    scale : f32,
}

impl LinearTransform {
    pub fn new(world_surface : Vec2, world_centre : Vec2, scale : f32) -> Self {
        let normal = world_surface.sub(world_centre).norm();
        let tangent = Vec2::new(-normal.y, normal.x);
        Self {
            offset: world_surface,
            scale,
            normal,
            tangent,
        }
    }
}

impl BlueprintToWorldTransform for LinearTransform {
    fn transform(&self, pos : Vec2) -> Vec2 {
        let mut p = self.offset;
        p = p.add(self.tangent.mult(pos.x * self.scale));
        p = p.add(self.normal.mult(pos.y * self.scale));
        p
    }
}

pub struct PolarTransform {
    theta : f32,
    world_radius : f32,
    world_centre : Vec2,
    scale : f32,
}

impl PolarTransform {
    pub fn new(world_surface : Vec2, world_centre : Vec2, scale : f32) -> Self {
        let world_radius = world_centre.dist(world_surface);
        let theta = scale.atan2(world_radius);

        Self {
            theta,
            world_radius,
            world_centre,
            scale
        }
    }
}

impl BlueprintToWorldTransform for PolarTransform {
    fn transform(&self, pos : Vec2) -> Vec2 {
        let polar_angle = pos.x * self.theta;
        let polar_r = self.world_radius + pos.y * self.scale;

        self.world_centre.add(Vec2::new(
            polar_r * polar_angle.cos(),
            polar_r * polar_angle.sin()))
    }
}

pub struct HybridTransform {
    theta : f32,
    world_radius : f32,
    world_centre : Vec2,
    angle_base : f32,
    scale : f32,
    normal : Vec2,
}

impl HybridTransform {
    pub fn new(world_surface : Vec2, world_centre : Vec2, scale : f32) -> Self {
        let world_radius = world_centre.dist(world_surface);
        let theta = scale.atan2(world_radius);
        let normal = world_surface.sub(world_centre).norm();
        let angle_base = normal.y.atan2(normal.x);

        Self {
            theta,
            world_radius,
            world_centre,
            scale,
            normal,
            angle_base,
        }
    }
}

impl BlueprintToWorldTransform for HybridTransform {
    fn transform(&self, pos : Vec2) -> Vec2 {
        let base_point_polar_angle =
            self.angle_base + (pos.x * self.theta) / (1.0 + 0.0125 * pos.y);
        let base_point = self.world_centre.add(Vec2::new(
            self.world_radius * base_point_polar_angle.cos(),
            self.world_radius * base_point_polar_angle.sin(),
        ));

        let p = base_point.add(self.normal.mult(pos.y * self.scale));
        p
    }
}