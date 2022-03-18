use crate::rayt::*;
use image::{Rgb,RgbImage};
use rayon::prelude::*;
use std::{fs,path::Path};

const IMAGE_HEIGHT:u32 = 400;
const IMAGE_WIDTH:u32 = 400;
const OUTPUT_FILENAME: &str = "render.png";
const BACKUP_FILENAME: &str = "render_bak.png";

const SAMPLES_PER_PIXEL:usize = 1;// 解像度が上がる
const GAMMA_FACTOR:f64 = 2.2;// 元の明るさが上がる
const MAX_RAY_BOUNCE_DEPTH:usize = 10;

fn backup(filename:&str){
    let output_path = Path::new(OUTPUT_FILENAME);
    if output_path.exists(){
        println!("backup {:?} -> {:?}",filename,BACKUP_FILENAME);
        fs::rename(filename,BACKUP_FILENAME).unwrap();
    }
}

pub trait Scene {
    fn camera(&self) -> Camera;
    fn trace(&self, ray:Ray) -> Color;
    fn width(&self) -> u32 {IMAGE_WIDTH}
    fn height(&self) -> u32 {IMAGE_HEIGHT}
    fn spp(&self) -> usize{ SAMPLES_PER_PIXEL }
    fn aspect(&self) -> f64 {self.width() as f64 / self.height() as f64}
}

pub trait SceneWithDepth {
    fn camera(&self) -> Camera;
    fn trace(&self, ray: Ray, depth: usize) -> Color;
    fn width(&self) -> u32 { IMAGE_WIDTH }
    fn height(&self) -> u32 { IMAGE_HEIGHT }
    fn spp(&self) -> usize { SAMPLES_PER_PIXEL }
    fn aspect(&self) -> f64 { self.width() as f64 / self.height() as f64 }
}

pub fn render(screne:impl Scene + Sync){
    backup(&("as"));

    let camera = screne.camera();
    let mut img = RgbImage::new(screne.width(),screne.height());
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32,u32,&mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x,y,pixel)|{
            let u = *x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = (IMAGE_HEIGHT - *y - 1) as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = camera.ray(u,v);
            let rgb = screne.trace(ray).to_rgb();
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
        });
        img.save(OUTPUT_FILENAME).unwrap();
        draw_in_window(BACKUP_FILENAME,img).unwrap();
}


pub fn render_aa(screne:impl Scene + Sync){
    backup(&("as"));

    let camera = screne.camera();
    let mut img = RgbImage::new(screne.width(),screne.height());
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32,u32,&mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x,y,pixel)|{
            let mut pixel_color = (0..screne.spp()).into_iter().fold(Color::zero(),|acc, _|{
                let [rx,ry,_] = Float3::random().to_array();
                let u = (*x as f64 + rx) / (screne.width() - 1) as f64;
                let v = ((screne.height() - *y - 1) as f64 + ry) / (screne.height() - 1) as f64;
                let ray = camera.ray(u,v);
                acc + screne.trace(ray)
            });
            pixel_color /= screne.spp() as f64;
            let rgb = pixel_color.gamma(GAMMA_FACTOR).to_rgb();
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
        });
    img.save(OUTPUT_FILENAME).unwrap();
    draw_in_window(BACKUP_FILENAME,img).unwrap();
}

pub fn render_aa_with_depth(scene: impl SceneWithDepth + Sync,filename:&str) {
    backup(filename);

    let camera = scene.camera();
    let mut img = RgbImage::new(scene.width(), scene.height());
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let mut pixel_color = (0..scene.spp()).into_iter().fold(Color::zero(), |acc, _| {
                let [rx, ry, _] = Float3::random().to_array();
                let u = (*x as f64 + rx) / (scene.width() - 1) as f64;
                let v = ((scene.height() - *y - 1) as f64 + ry) / (scene.height() - 1) as f64;
                let ray = camera.ray(u, v);
                acc + scene.trace(ray, MAX_RAY_BOUNCE_DEPTH)
            });
            pixel_color /= scene.spp() as f64;
            // let rgb = pixel_color.to_rgb();
            let rgb = pixel_color.gamma(GAMMA_FACTOR).to_rgb();
            // let rgb = pixel_color.gamma(GAMMA_FACTOR).saturate().to_rgb();
            // let rgb = nan_check(pixel_color).to_rgb();
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
        });
    img.save(filename).unwrap();
    print!("{}",filename);
    //draw_in_window(BACKUP_FILENAME, img).unwrap();
}