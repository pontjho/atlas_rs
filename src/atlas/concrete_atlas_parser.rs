use crate::atlas::atlas_parser::AtlasParser;
use crate::atlas::page::Page;
use crate::atlas::page_format::PageFormat;
use crate::atlas::page_filter::PageFilter;
use crate::atlas::page_repeat::PageRepeat;
use crate::atlas::region::Region;

use itertools::Itertools;
use std::str::FromStr;

pub struct ConcreteAtlasParser
{
}

impl AtlasParser for ConcreteAtlasParser
{
    fn parse(&self, lines: std::slice::Iter<&str>) -> Vec<Page>
    {
        let initial_state: (Vec<Page>, Option<Page>, Option<Region>) = (vec![], None, None);
        let (rest_of_pages, current_page, current_region) = lines.fold(initial_state, |(rest_of_pages, current_page, current_region), line| {
            let trimmed_line = line.trim();

            match (&current_page, &current_region)
            {
                (None, None) if trimmed_line == "" => (rest_of_pages, current_page, current_region),
                (None, None) => (rest_of_pages, Some(Page::new(trimmed_line)), None),
                (Some(curr_page), None) => parse_current_page_section(rest_of_pages, curr_page.clone(), trimmed_line),
                (Some(curr_page), Some(curr_region)) if trimmed_line == "" => {
                    let page_updated_with_last_region = curr_page.clone().add_region(curr_region.clone());
                    let rest_of_pages = rest_of_pages.into_iter().chain(vec![page_updated_with_last_region]).collect();
                    (rest_of_pages, None, None)
                },
                (Some(curr_page), Some(curr_region)) => parse_current_region_section(rest_of_pages, curr_page.clone(), curr_region.clone(), trimmed_line),
                (None, Some(_)) => panic!("Invalid parse state")
            }
        });
        let last_page = match current_region { Some(current_region) => current_page.map(|cp| cp.add_region(current_region)), None => current_page };
        let the_return = rest_of_pages.into_iter().chain(last_page).collect();
        the_return
    }
}

fn parse_current_region_section(rest_of_pages: Vec<Page>, current_page: Page, current_region: Region, line: &str) -> (Vec<Page>, Option<Page>, Option<Region>)
{
    let (updated_region, new_region) = if line.starts_with("index:")
    {
        let index_str = extract_value(line);
        let i_index: isize = index_str.parse().unwrap();
        let index = if i_index < 0 { None } else { Some(i_index as usize) };
        (Region { index, ..current_region }, None)
    }
    else if line.starts_with("bounds:")
    {
        let bounds = extract_4usize(line);
        (Region { bounds, ..current_region }, None)
    }
    else if line.starts_with("offsets:")
    {
        let offsets = extract_4usize(line);
        (Region { offsets, ..current_region }, None)
    }
    else if line.starts_with("rotate:")
    {
        let rotate_str = extract_value(line);
        let rotate = if rotate_str == "false"
        { 0 }
        else if rotate_str == "true"
        { 90 }
        else
        { rotate_str.parse::<usize>().unwrap() };
        (Region { rotate, ..current_region }, None)
    }
    else if line.starts_with("split:")
    {
        let split = Some(extract_4usize(line));
        (Region { split, ..current_region }, None)
    }
    else if line.starts_with("pad:")
    {
        let pad = Some(extract_4isize(line));
        (Region { pad, ..current_region }, None)
    }
    else
    {
        (current_region, Some(Region::new(line)))
    };

    let (updated_page, updated_region) = match new_region
    {
        Some(new_r) => (current_page.add_region(updated_region), new_r),//(Page { regions: current_page.regions.clone().into_iter().chain(vec![updated_region]).collect(), ..current_page }, new_r),
        None => (current_page, updated_region)
    };

    (rest_of_pages, Some(updated_page), Some(updated_region))
}

fn parse_current_page_section(rest_of_pages: Vec<Page>, current_page: Page, line: &str) -> (Vec<Page>, Option<Page>, Option<Region>)
{
    let (updated_page, region) = if line.starts_with("size:")
    {
        let size = extract_2usize(line);
        (Page { size, ..current_page }, None)
    }
    else if line.starts_with("format:")
    {
        let format_str = extract_value(line);
        let format = PageFormat::from_str(&format_str).unwrap();
        (Page { format, ..current_page }, None)
    }
    else if line.starts_with("filter:")
    {
        let tuple_str = extract_value(line);
        let filter: (PageFilter, PageFilter) = tuple_str
            .split(",")
            .map(|v| PageFilter::from_str(v.trim()).unwrap())
            .next_tuple()
            .unwrap();
        (Page { filter, ..current_page }, None)
    }
    else if line.starts_with("repeat:")
    {
        let repeat_str = extract_value(line);
        let repeat = PageRepeat::from_str(&repeat_str).unwrap();
        (Page { repeat, ..current_page }, None)
    }
    else if line.starts_with("pma:")
    {
        let pre_multiplexed_alpha_enabled = extract_value(line).parse().unwrap();
        (Page { pre_multiplexed_alpha_enabled, ..current_page }, None)
    }
    else
    {
        (current_page.clone(), Some(Region::new(line)))
    };

    (rest_of_pages, Some(updated_page), region)
}

fn extract_value(line: &str) -> String
{
    let index = line.find(":");
    let rest: String = line.chars().skip(index.unwrap() + 1).collect();
    rest.trim().to_string()
}

fn extract_4usize(line: &str) -> (usize, usize, usize, usize)
{
    let tuple_str = extract_value(line);
    let sections = tuple_str
        .split(",")
        .map(|v| v.trim().parse::<usize>().unwrap())
        .next_tuple()
        .unwrap();
    sections
}

fn extract_2usize(line: &str) -> (usize, usize)
{
    let tuple_str = extract_value(line);
    let sections = tuple_str
        .split(",")
        .map(|v| v.trim().parse::<usize>().unwrap())
        .next_tuple()
        .unwrap();
    sections
}

fn extract_4isize(line: &str) -> (isize, isize, isize, isize)
{
    let tuple_str = extract_value(line);
    let sections = tuple_str
        .split(",")
        .map(|v| v.trim().parse::<isize>().unwrap())
        .next_tuple()
        .unwrap();
    sections
}
