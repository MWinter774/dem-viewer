extern crate tiff;
use std::fs::File;
use tiff::decoder::{Decoder, DecodingResult};

pub struct TerrainModelData {
    width: usize,
    height: usize,
    data: Vec<f32>,
}

impl TerrainModelData {
    pub fn from_geotiff_file(geotiff_file_path: &str) -> Self {
        let mut decoder = Decoder::new(File::open(geotiff_file_path).unwrap())
            .expect("Cannot create GeoTIFF decoder");
        decoder = decoder.with_limits(tiff::decoder::Limits::unlimited());
        let (width, height) = decoder.dimensions().unwrap();
        let (width, height) = (width as usize, height as usize);
        let mut data = vec![0.0; width * height];
        if let DecodingResult::F32(img) = decoder.read_image().unwrap() {
            data = img;
        } else {
            panic!("Wrong data type");
        }
        Self {
            width,
            height,
            data,
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_data(&self) -> &Vec<f32> {
        &self.data
    }
}
