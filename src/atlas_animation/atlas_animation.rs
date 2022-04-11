use crate::atlas::page::Page;
use std::collections::HashMap;
use itertools::Itertools;

pub trait AtlasAnimation
{
    fn get_frame(&self, region: &str, frame: f32) -> AnimationFrame;
    fn get_frame_exact(&self, region: &str, frame: usize) -> AnimationFrame;
    fn get_indexed_frame(&self, region: usize, frame: f32) -> AnimationFrame;
    fn get_indexed_frame_exact(&self, region: usize, frame: usize) -> AnimationFrame;
}

pub struct ConcreteAtlasAnimation
{
    pub region_lookup: HashMap<String, usize>,
    pub region_frames: Vec<Vec<AnimationFrame>>
}

impl ConcreteAtlasAnimation
{
    pub fn new(pages: Vec<Page>) -> ConcreteAtlasAnimation
    {
        let mut region_lookup: HashMap<String, usize> = pages
            .iter()
            .map(|page|
                page
                    .regions
                    .iter()
                    .map(|region| region.name.clone())
                    .collect::<Vec<_>>()
            )
            .flatten()
            .unique()
            .enumerate()
            .map(|(index, region_name)| (region_name, index))
            .collect();
        let mut region_frames: Vec<Vec<AnimationFrame>> = vec![];

        for page in pages
        {
            //let page_frames: Vec<AnimationFrame> = Default::default();

            for region in page.regions
            {
                let region_id = region_lookup[&region.name];

                let (width, height) = page.size;
                let (pad_left, pad_bottom, orig_x, orig_y) = region.offsets;
                let (sub_image_x, sub_image_y, sub_image_width, sub_image_height) = region.bounds;

                let (width, height) = (width as f32, height as f32);
                let (pad_left, pad_bottom, orig_x, orig_y) = (pad_left as f32, pad_bottom as f32, orig_x as f32, orig_y as f32);
                let (sub_image_x, sub_image_y, sub_image_width, sub_image_height) = (sub_image_x as f32, sub_image_y as f32, sub_image_width as f32, sub_image_height as f32);

                let sub_image_x_normalised = sub_image_x / width;
                let sub_image_y_normalised = sub_image_y / height;
                let sub_image_width_normalised = sub_image_width / width;
                let sub_image_height_normalised = sub_image_height / height;
                let uv_x1 = sub_image_x_normalised;
                let uv_x2 = sub_image_x_normalised + sub_image_width_normalised;
                let uv_y1 = sub_image_y_normalised;
                let uv_y2 = sub_image_y_normalised + sub_image_height_normalised;

                // let scale_x = 1.0 / sub_image_width;
                // let scale_y = 1.0 / sub_image_height;

                let pad_right = orig_x - sub_image_width;
                let pad_top = orig_y - sub_image_height;
                // let pad_left_normalised = pad_left / width;
                // let pad_right_normalised = pad_right / width;
                // let pad_top_normalised = pad_top / height;
                // let pad_bottom_normalised = pad_bottom / height;

                // let (half_width, half_height) = (sub_image_width / 2.0, sub_image_height / 2.0);
                // let (left, right) = (-half_width + pad_left, half_width - pad_right);
                // let (top, bottom) = (half_height - pad_top, -half_height + pad_bottom);
                // let vertices = [
                //     [left, top, 1.0],
                //     [right, top, 1.0],
                //     [right, bottom, 1.0],
                //     [left, bottom, 1.0]
                // ];

                // let vx1 = (-1.0 + pad_left_normalised) * width;
                // let vx2 = (1.0 - pad_right_normalised) * width;
                // let vy1 = (1.0 - pad_top_normalised) * height;
                // let vy2 = (-1.0 + pad_bottom_normalised) * height;

                region_frames.push(vec![]);
                region_frames[region_id].push(AnimationFrame {
                    image_id: region_id,
                    default_dimensions: (sub_image_width, sub_image_height),
                    //transform: cgmath::Matrix3::from_nonuniform_scale(scale_x, scale_y).into(),
                    // vertices,
                    padding: [pad_left, pad_top, pad_right, pad_bottom],
                    uvs: [
                        [uv_x1, uv_y1],
                        [uv_x2, uv_y1],
                        [uv_x2, uv_y2],
                        [uv_x1, uv_y2]
                    ]
                });
            }
        }

        println!("{:#?}", region_lookup);

        ConcreteAtlasAnimation {
            region_lookup,
            region_frames
        }
    }
}

impl AtlasAnimation for ConcreteAtlasAnimation
{
    fn get_frame(&self, region: &str, frame: f32) -> AnimationFrame
    {
        let frame_index = self.region_lookup[region];
        self.get_indexed_frame(frame_index, frame)
    }

    fn get_frame_exact(&self, region: &str, frame: usize) -> AnimationFrame
    {
        let frame_index = self.region_lookup[region];
        self.get_indexed_frame_exact(frame_index, frame)
    }

    fn get_indexed_frame(&self, region: usize, frame: f32) -> AnimationFrame
    {
        let ref region = self.region_frames[region];
        let frame = ((region.len() as f32) * frame) as usize;
        let frame = if frame >= region.len() { region.len() - 1 } else { frame };
        region[frame].clone()
    }

    fn get_indexed_frame_exact(&self, region: usize, frame: usize) -> AnimationFrame
    {
        let ref region = self.region_frames[region];
        // let frame = ((region.len() as f32) * frame) as usize;
        let frame = if frame >= region.len() { region.len() - 1 } else { frame };
        region[frame].clone()
    }
}

#[derive(Clone, Debug)]
pub struct AnimationFrame
{
    pub image_id: usize,
    pub padding: [f32; 4],
    pub default_dimensions: (f32, f32),
    // pub vertices: [[f32; 3]; 4],
    //pub indices: [u16; 6],
    // pub transform: [[f32; 3]; 3],
    pub uvs: [[f32; 2]; 4]
}
