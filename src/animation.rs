use crate::tracer::Vec3;

#[derive(Copy, Clone)]
pub struct Orbit3D {
    pub center: Vec3,
    radius: f32,
    /// Orbital inclination in radians
    inclination: f32,
    /// Velocity in degrees per second
    velocity: f32,
}

impl Orbit3D {
    pub fn new(start_point: Vec3, center: Vec3, velocity: f32) -> Orbit3D {
        let rise = start_point.y - center.y;
        let run = ((start_point.x * start_point.x) + (start_point.z * start_point.z)).sqrt();
        let inclination = rise.atan2(run);
        Orbit3D {
            center,
            radius: (start_point - center).length(),
            inclination,
            velocity,
        }
    }

    /// Find the point at azimut in DEGREES
    pub fn point_at_azimuth(&self, azimuth: f32) -> Vec3 {
        let x = azimuth.to_radians().cos() * self.radius;
        let z = azimuth.to_radians().sin() * self.radius;
        let y = (x / self.inclination.cos()) * self.inclination.sin();
        Vec3::new(x, y, z) + self.center
    }

    /// Find the point at a given time.
    pub fn point_at_time(&self, time: f32) -> Vec3 {
        let degrees = time * self.velocity;
        self.point_at_azimuth(degrees)
    }
}

#[derive(Copy, Clone)]
/// Linear Movement at a constant velocity definition
pub struct ConstantMoveL {
    pub origin: Vec3,
    pub direction: Vec3,
    pub velocity: f32,
}

impl ConstantMoveL {
    pub fn point_at_time(&self, time: f32) -> Vec3 {
        let distance = self.velocity * time;
        self.origin + distance * self.direction
    }
}
