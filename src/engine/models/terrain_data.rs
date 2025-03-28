pub struct TerrainData {
    ds: gdal_win::Dataset,
    width: usize,
    height: usize,
    data: Vec<f64>,
}

impl TerrainData {
    pub fn from_geotiff_file(geotiff_file_path: &str) -> Self {
        let ds = gdal_win::Dataset::open(geotiff_file_path).unwrap();
        let rasterband = ds.rasterband(1).unwrap();
        let (width, height) = rasterband.size();
        let mut data = vec![0.0; width * height];
        rasterband
            .read_into_slice(
                (0, 0),
                rasterband.size(),
                (width, height),
                data.as_mut_slice(),
                None,
            )
            .unwrap();
        Self {
            ds,
            width,
            height,
            data,
        }
    }

    pub fn get_rasterband(&self) -> gdal_win::raster::RasterBand<'_> {
        self.ds.rasterband(1).unwrap()
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_geo_transform(&self) -> [f64; 6] {
        self.ds.geo_transform().unwrap()
    }

    pub fn get_data(&self) -> &Vec<f64> {
        &self.data
    }
}
