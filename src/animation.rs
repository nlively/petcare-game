use raylib::prelude::{Texture2D, Rectangle};
use std::sync::Arc;

/// High-level facing for sprites.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Facing {
    Left,
    Right,
    Front,
    Back,
}

/// Pose/state that affects which animation to player.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Pose {
    Sitting,
    Walking,
    Sleeping,
    Standing,
    HindLegs,
}

/// Emotion or "flavor" of the sprite (changes face, etc)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Emotion {
    Neutral,
    Happy,
    Sad,
}

/// Descriptor: texture = frames + timing. Shared across instances.
#[derive(Clone)]
pub struct AnimationDescriptor {
    pub texture: Arc<Texture2D>,
    pub frames: Vec<Rectangle>, // source rects in texture
    pub frame_duration_secs: f32,
    pub looped: bool,
}

impl AnimationDescriptor {
    pub fn len(&self) -> usize { self.frames.len() }
}

/// Per-instance player: current frame index + elapsed time
pub struct AnimationPlayer {
    pub descriptor: AnimationDescriptor,
    pub current_frame: usize,
    pub elapsed: f32, // seconds into current frame
    pub playing: bool,
}

impl AnimationPlayer {
    pub fn new(desc: AnimationDescriptor) -> Self {
        Self { descriptor: desc, current_frame: 0, elapsed: 0.0, playing: true }
    }

    /// Advance animation by dt seconds
    pub fn update(&mut self, dt: f32) {
        if !self.playing || self.descriptor.len() <= 1 { return; }
        self.elapsed += dt;
        while self.elapsed >= self.descriptor.frame_duration_secs {
            self.elapsed -= self.descriptor.frame_duration_secs;
            self.current_frame += 1;
            if self.current_frame >= self.descriptor.len() {
                if self.descriptor.looped {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.descriptor.len().saturating_sub(1);
                    self.playing = false;
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.elapsed = 0.0;
        self.playing = true;
    }

    pub fn current_frame_rect(&self) -> Rectangle {
        self.descriptor.frames[self.current_frame]
    }

    pub fn texture(&self) -> &Texture2D {
        &self.descriptor.texture
    }
}