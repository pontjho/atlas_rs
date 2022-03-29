use crate::atlas::region::Region;
use crate::atlas::page_repeat::PageRepeat;
use crate::atlas::page_filter::PageFilter;
use crate::atlas::page_format::PageFormat;

#[derive(Clone, Debug)]
pub struct Page
{
    pub name: String,
    pub size: (usize, usize),
    pub format: PageFormat,
    pub filter: (PageFilter, PageFilter),
    pub repeat: PageRepeat,
    pub pre_multiplexed_alpha_enabled: bool,
    pub regions: Vec<Region>
}

impl Default for Page
{
    fn default() -> Self
    {
        Page {
            name: "".to_string(),
            size: (0, 0),
            format: PageFormat::RGBA8888,
            filter: (PageFilter::Nearest, PageFilter::Nearest),
            repeat: PageRepeat::None,
            pre_multiplexed_alpha_enabled: false,
            regions: vec![]
        }
    }
}

impl Page
{
    pub fn new(name: &str) -> Self
    {
        Page { name: name.to_string(), ..Page::default() }
    }

    pub fn add_region(self, region: Region) -> Self
    {
        Page { regions: self.regions.clone().into_iter().chain(vec![region.finalise()]).collect(), ..self }
    }
}
