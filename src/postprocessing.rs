use glam::Vec3;
use rayon::prelude::*;

pub struct FrameData {
    pub width: usize,
    pub height: usize,
    pub colors: Vec<Vec3>,
    pub depths: Vec<f32>,
}

pub trait PostProcessStep {
    fn apply(&self, frame: &mut FrameData);
}

pub struct PostProcessingPipeline {
    steps: Vec<Box<dyn PostProcessStep>>,
}

impl PostProcessingPipeline {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn add_step(mut self, step: impl PostProcessStep + 'static) -> Self {
        self.steps.push(Box::new(step));
        self
    }

    pub fn execute(&self, frame: &mut FrameData) {
        for pass in &self.steps {
            pass.apply(frame);
        }
    }
}


pub struct GammaCorrection {
    pub gamma: f32
}

impl PostProcessStep for GammaCorrection {
    fn apply(&self, frame: &mut FrameData) {
        let inv_gamma = 1.0 / self.gamma;
        frame.colors.iter_mut().for_each(|color| {
            color.x = color.x.powf(inv_gamma);
            color.y = color.y.powf(inv_gamma);
            color.z = color.z.powf(inv_gamma);
        }); 
    }
}


pub struct Clipping;

impl PostProcessStep for Clipping {
    fn apply(&self, frame: &mut FrameData) {
        frame.colors.iter_mut().for_each(|color| {
            color.x = color.x.min(1.0);
            color.y = color.y.min(1.0);
            color.z = color.z.min(1.0);
        }); 
    }
}


pub struct BilateralFilterStep {
    pub sigma_spatial: f32,
    pub sigma_depth: f32,
}

impl PostProcessStep for BilateralFilterStep {
    fn apply(&self, frame: &mut FrameData) {
        let width = frame.width;
        let height = frame.height;
        let radius = (2.0 * self.sigma_spatial).ceil() as i32;

        let mut filtered_colors = vec![Vec3::ZERO; width * height];

        for (y, row) in filtered_colors.chunks_mut(width).enumerate() {
            for x in 0..width {
                let idx_p = y * width + x;
                let depth_p = frame.depths[idx_p];

                if depth_p == f32::INFINITY {
                    row[x] = frame.colors[idx_p];
                    continue;
                }

                let mut color_sum = Vec3::ZERO;
                let mut weight_sum = 0.0;

                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            let idx_q = (ny * width as i32 + nx) as usize;
                            let depth_q = frame.depths[idx_q];

                            let dist_sq = (dx * dx + dy * dy) as f32;
                            let weight_s = (-dist_sq / (2.0 * self.sigma_spatial * self.sigma_spatial)).exp();

                            let depth_diff = depth_p - depth_q;
                            let weight_d = (-(depth_diff * depth_diff) / (2.0 * self.sigma_depth * self.sigma_depth)).exp();

                            let w = weight_s * weight_d;
                            color_sum += frame.colors[idx_q] * w;
                            weight_sum += w;
                        }
                    }
                }
                row[x] = color_sum / weight_sum;
            }
        }

        frame.colors = filtered_colors;
    }
}
