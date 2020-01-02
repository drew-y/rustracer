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
        let run = Vec3::new(start_point.x, 0.0, start_point.z)
            .distance_from(Vec3::new(center.x, 0.0, center.z));
        let inclination = rise.atan2(run);
        Orbit3D {
            center,
            radius: run,
            inclination,
            velocity,
        }
    }

    /// Find the point at azimut in DEGREES
    pub fn point_at_azimuth(&self, azimuth: f32) -> Vec3 {
        let x = self.radius * azimuth.to_radians().cos();
        let z = self.radius * azimuth.to_radians().sin();
        let y = x * self.inclination.tan();
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
pub struct MoveL {
    start: Vec3,
    end: Vec3,
    velocity: f32,
}

impl MoveL {
    pub fn new(start: Vec3, end: Vec3, velocity: f32) -> MoveL {
        MoveL {
            start,
            end,
            velocity,
        }
    }

    pub fn point_at_time(&self, time: f32) -> Vec3 {
        let progress_to_end =
            ((self.velocity * time) / self.start.distance_from(self.end)).min(1.0);
        self.start + progress_to_end * (self.end - self.start)
    }
}
