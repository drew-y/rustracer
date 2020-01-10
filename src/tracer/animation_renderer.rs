use crate::tracer::render::{Image, Renderer};

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

        let mut frame = (time / time_step) as i32 + 1;
        while time <= self.end_time {
            let renderer = Renderer::from((self.image_fn)(time));
            renderer.render(format!("./{}/frame-{}.png", path, frame));
            time += time_step;
            frame += 1;
        }
    }

    pub fn render_with_progress_bar(&self, path: impl std::fmt::Display) {
        let time_step = 1.0 / self.fps;
        let mut time = self.start_time;

        let mut frame = (time / time_step) as i32 + 1;
        while time <= self.end_time {
            let mut renderer = Renderer::from((self.image_fn)(time));
            renderer.render_with_progress_bar(format!("./{}/frame-{}.png", path, frame));
            time += time_step;
            frame += 1;
        }
    }
}
