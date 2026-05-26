use glam::Vec3;

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

    pub fn add_step(mut self, pass: impl PostProcessStep + 'static) -> Self {
        self.steps.push(Box::new(pass));
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
