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
    pub region_frames: Vec<Vec<AnimationFrame>>,
    // pub base_image_index: usize
}

impl ConcreteAtlasAnimation
{
    pub fn new(pages: Vec<Page>, base_image_index: usize) -> ConcreteAtlasAnimation
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

        for (page_id, page) in pages.into_iter().enumerate()
        {
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

                let pad_right = orig_x - sub_image_width;
                let pad_top = orig_y - sub_image_height;


                // println!("--------{}---------{}------------------", page_id, base_image_index);

                let animation_frame = AnimationFrame {
                    image_id: base_image_index + page_id,
                    default_dimensions: (sub_image_width, sub_image_height),
                    padding: [pad_left, pad_top, pad_right, pad_bottom],
                    uvs: [
                        [uv_x1, uv_y1],
                        [uv_x2, uv_y1],
                        [uv_x2, uv_y2],
                        [uv_x1, uv_y2]
                    ]
                };

                region_frames.push(vec![]);
                region_frames[region_id].push(animation_frame);
            }
        }

        // println!("{:#?}", region_lookup);
        //panic!();

        ConcreteAtlasAnimation {
            region_lookup,
            region_frames,
           // base_image_index
        }
    }
}

impl AtlasAnimation for ConcreteAtlasAnimation
{
    fn get_frame(&self, region: &str, frame: f32) -> AnimationFrame
    {
        println!("{}", region);
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
