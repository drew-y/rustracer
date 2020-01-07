use crate::tracer::helpers::make_progress_bar;
use crate::tracer::render::{Image, Renderer};
use indicatif::MultiProgress;

#[derive(Clone)]
pub struct Animation {
    pub fps: f32,
    pub start_time: f32,
    pub end_time: f32,
    /// A function that returns a scene when passed time in seconds
    pub image_fn: &'static dyn Fn(f32) -> Image,
}

#[derive(Clone)]
pub struct AnimationRenderer {
    fps: f32,
    start_time: f32,
    end_time: f32,
    /// A function that returns a scene when passed time in seconds
    image_fn: &'static dyn Fn(f32) -> Image,
}

impl From<Animation> for AnimationRenderer {
    fn from(animation: Animation) -> Self {
        AnimationRenderer {
            fps: animation.fps,
            start_time: animation.start_time,
            end_time: animation.end_time,
            image_fn: animation.image_fn,
        }
    }
}

impl AnimationRenderer {
    pub fn render(&self, path: impl std::fmt::Display) {
        let time_step = 1.0 / self.fps;
        let mut time = self.start_time;
        let mpb = MultiProgress::new();
        // let pb = mpb.add(make_progress_bar(
        //     "Animating",
        //     ((self.end_time - self.start_time) * self.fps) as i32,
        // ));
        let renderer_pb = mpb.add(make_progress_bar("Rendering", 10));

        // pb.tick();
        // pb.enable_steady_tick(1000);

        let mut frame = (time / time_step) as i32 + 1;
        while time <= self.end_time {
            let image = (self.image_fn)(time);
            renderer_pb.set_length(image.height as u64);
            renderer_pb.set_position(0);
            let mut renderer = Renderer::from(image);
            renderer.override_progress_bar(renderer_pb.clone());
            renderer.render(format!("./{}/frame-{}.png", path, frame));
            time += time_step;
            frame += 1;
            // pb.inc(1);
        }

        mpb.join_and_clear().unwrap();
    }
}
