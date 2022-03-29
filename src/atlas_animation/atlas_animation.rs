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
                let (sub_image_x, sub_image_y, sub_image_width, sub_image_height) = region.bounds;
                let sub_image_x_normalised = sub_image_x as f32 / width as f32;
                let sub_image_y_normalised = sub_image_y as f32 / height as f32;
                let sub_image_width_normalised = sub_image_width as f32 / width as f32;
                let sub_image_height_normalised = sub_image_height as f32 / height as f32;
                let uv_x1 = sub_image_x_normalised;
                let uv_x2 = sub_image_x_normalised + sub_image_width_normalised;
                let uv_y1 = sub_image_y_normalised;
                let uv_y2 = sub_image_y_normalised + sub_image_height_normalised;

                let (pad_left, pad_top, orig_x, orig_y) = region.offsets;
                let pad_right = orig_x - sub_image_width;
                let pad_bottom = orig_y - sub_image_height;
                let pad_left_normalised = pad_left as f32 / width as f32;
                let pad_right_normalised = pad_right as f32 / width as f32;
                let pad_top_normalised = pad_top as f32 / height as f32;
                let pad_bottom_normalised = pad_bottom as f32 / height as f32;

                let vx1 = -1.0 + pad_left_normalised;
                let vx2 = 1.0 - pad_right_normalised;
                let vy1 = 1.0 - pad_top_normalised;
                let vy2 = -1.0 + pad_bottom_normalised;

                region_frames.push(vec![]);
                region_frames[region_id].push(AnimationFrame {
                    image_id: region_id,
                    vertices: [
                        [vx1, vy1, 0.0],
                        [vx2, vy1, 0.0],
                        [vx2, vy2, 0.0],
                        [vx1, vy2, 0.0]
                    ],
                    indices: [ 0, 1, 2, 0, 2, 3],
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
        todo!()
    }

    fn get_frame_exact(&self, region: &str, frame: usize) -> AnimationFrame
    {
        todo!()
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
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct AnimationFrame
{
    pub image_id: usize,
    pub vertices: [[f32; 3]; 4],
    pub indices: [u16; 6],
    pub uvs: [[f32; 2]; 4]
}
