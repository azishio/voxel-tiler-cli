use std::fmt::Display;

use coordinate_transformer::{jpr2ll, JprOrigin, pixel_resolution, ZoomLv};
use las::{Read, Vector};
use tabled::{row, Tabled};

#[derive(Tabled, Clone)]
struct Bounds {
    coord: String,
    x: f64,
    y: f64,
    z: f64,
}

impl Bounds {
    pub fn from_vec(name: &str, vec: Vector<f64>, rotate: bool) -> Self {
        if rotate {
            Self {
                coord: name.to_string(),
                x: vec.y,
                y: vec.x,
                z: vec.z,
            }
        } else {
            Self {
                coord: name.to_string(),
                x: vec.x,
                y: vec.y,
                z: vec.z,
            }
        }
    }
}

pub struct LasInfo {
    num_of_points: u64,
    max: Bounds,
    min: Bounds,
    center: Bounds,
}

impl LasInfo {
    pub fn from_path(path: &str, rotate: bool) -> Self {
        let reader = las::Reader::from_path(path).unwrap();
        let header = reader.header();
        let num_of_points = header.number_of_points();
        let bounds = header.bounds();


        let max = Bounds::from_vec("max [m]", bounds.max, rotate);
        let min = Bounds::from_vec("min [m]", bounds.min, rotate);

        let center = Vector {
            x: (max.x + min.x) / 2.0,
            y: (max.y + min.y) / 2.0,
            z: (max.z + min.z) / 2.0,
        };
        let center = Bounds::from_vec("center [m]", center, rotate);

        Self {
            num_of_points,
            max,
            min,
            center,
        }
    }

    pub fn print_info(&self) {
        let table = tabled::Table::new(vec![self.max.clone(), self.min.clone(), self.center.clone()]);
        println!("{}", row!["Number of Points", self.num_of_points]);
        println!("{}", table);
    }


    pub fn resolution_list(&self, jpr_origin: JprOrigin) -> Vec<ZoomLevel> {
        let (_long, lat) = jpr2ll((self.center.y, self.center.x), jpr_origin);

        (ZoomLv::Lv0 as u8..=ZoomLv::Lv24 as u8).map(|lv| {
            let zoom_lv = ZoomLv::parse(lv).unwrap();

            let resolution = pixel_resolution(lat, zoom_lv);

            ZoomLevel { zoom_lv, resolution }
        }).collect()
    }
}

type Resolution = f64;

pub struct ZoomLevel {
    pub zoom_lv: ZoomLv,
    pub resolution: Resolution,
}

impl Display for ZoomLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZoomLevel: {:2} ({:6.2} m/voxel)", self.zoom_lv as u8, self.resolution)
    }
}
