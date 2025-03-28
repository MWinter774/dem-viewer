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
}
