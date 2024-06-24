
use vtf::ImageFormat;
//use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;


fn main() {
    let sample_vmt = include_str!("sample.vmt");

    let input_folder = String::from("input/");
    let output_folder = String::from("output/");

    println!("Skybuild - skybox builder for valve`s source, by expl01t");
    println!("[skybuild]: building vtfs...");

    let paths = std::fs::read_dir("input").unwrap();
    for source_image in paths {
        let image_name = source_image.unwrap().file_name().clone().into_string().unwrap();

        let input_path = input_folder.clone() + image_name.as_str();
        let output_path = output_folder.clone() + image_name.replace(".png", ".vtf").as_str();
        let vmt_output_path = output_folder.clone() + image_name.replace(".png", ".vmt").as_str();
        let vft_name = image_name.replace(".png", "");

        println!("[skybuild]: building {}", output_path);
        
        let image = image::open(input_path).unwrap();
        let vtf_data = vtf::create(image, ImageFormat::Dxt5).unwrap();
    
        let path = Path::new(&output_path);
        let mut file = File::create(path).unwrap();
        file.write_all(&vtf_data).unwrap();

        println!("[skybuild]: building {}", vmt_output_path);
        
        let path = Path::new(&vmt_output_path);
        let vmt = sample_vmt.to_string().replace("$name", vft_name.as_str());

        let mut file = File::create(path).unwrap();
        file.write_all(vmt.as_bytes()).unwrap();
        
    }
    println!("[skybuild]: done!");
}
