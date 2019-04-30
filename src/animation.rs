use crate::tracer::Vec3;

#[derive(Copy, Clone)]
pub struct Orbit3D {
    center: Vec3,
    radius: f32,
    /// Orbital inclination in degrees
    inclination: f32,
    start_azimuth: f32,
    /// Velocity in degrees per second
    velocity: f32,
}

impl Orbit3D {
    pub fn new(start_point: Vec3, center: Vec3, velocity: f32) -> Orbit3D {
        let origin = start_point - center;
        let radius = origin.length();
        let inclination = (origin.z / radius).acos().to_degrees();
        let start_azimuth = origin.x.atan2(origin.y).to_degrees();
        Orbit3D {
            center,
            radius,
            inclination,
            start_azimuth,
            velocity,
        }
    }

    /// Find the point at azimut in DEGREES
    pub fn point_at_azimuth(&self, azimuth: f32) -> Vec3 {
        let sin_inclination = self.inclination.to_radians().sin();
        let x = self.radius * sin_inclination * azimuth.to_radians().cos();
        let y = self.radius * sin_inclination * azimuth.to_radians().sin();
        let z = self.radius * self.inclination.to_radians().cos();
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
