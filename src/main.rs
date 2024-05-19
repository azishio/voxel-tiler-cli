use std::env::current_dir;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Write};
use std::path::Path;

use clap::Parser;
use coordinate_transformer::JprOrigin;
use inquire::{Confirm, MultiSelect, Select, Text};
use regex::Regex;
use tabled::col;
use voxel_tiler_core::{PlyStructs, Voxelizer};
use voxel_tiler_core::default_params::{Fit, Tile};

use crate::file_path_completer::{FilePathCompleter, to_plain_text};
use crate::las_info::LasInfo;
use crate::validators::{is_exist, is_jpr_origin, is_las_or_laz, zoom_level_selection_validator};

mod file_path_completer;
mod validators;
mod las_info;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {}

fn main() -> anyhow::Result<()> {
    Args::parse();

    let tiling = Confirm::new("Tiling?")
        .with_default(false)
        .prompt()?;

    let input_path = to_plain_text(&Text::new("Input File Path: ")
        .with_autocomplete(FilePathCompleter::new(Regex::new(r".*\.(las|laz)$").unwrap()))
        .with_initial_value(current_dir().unwrap().to_str().unwrap())
        .with_formatter(&to_plain_text)
        .with_validator(is_exist)
        .with_validator(is_las_or_laz)
        .prompt()?);

    // XYを入れ替えるかどうか
    let swap = Confirm::new("Swap X and Y?")
        .with_default(false)
        .prompt()?;

    // lasファイルの情報を表示
    println!("\nLas File Info\n");
    let info = LasInfo::from_path(&input_path, swap);
    info.print_info();
    println!("\n");

    // 平面直角座標系の原点
    let origin = Text::new("JPR Origin: ")
        .with_validator(is_jpr_origin).prompt()?.parse::<JprOrigin>().unwrap();

    let zoom_lv = MultiSelect::new("Select zoom levels", info.resolution_list(origin))
        .with_starting_cursor(17)
        .with_validator(zoom_level_selection_validator)
        .prompt()?;

    let output_path = to_plain_text(
        &Text::new("Output Directory Path: ")
            .with_autocomplete(FilePathCompleter::new(Regex::new(r"^$").unwrap()))
            .with_initial_value(current_dir().unwrap().to_str().unwrap())
            .with_formatter(&to_plain_text)
            .prompt()?);


    let file_format = Select::new(
        "Select file format",
        vec![
            "Ascii",
            "Binary (little endian)",
            "Binary (big endian)",
        ]).prompt()?;

    let table = col![
      format!("InputFile  : {}", input_path),
      format!("OutputDir  : {}/", output_path),
      format!("Ply Format : {}", file_format),
      format!("Tiling     : {}", tiling),
    ];
    println!("\nParams\n\n{}\n", table);

    if !Confirm::new("Continue?")
        .with_default(false)
        .prompt()?
    {
        println!("Canceled");
        return Ok(());
    }

    create_dir_all(&output_path).expect("Failed to create output directory");

    let input_file_name = Path::new(&input_path).file_name().unwrap().to_str().unwrap();
    zoom_lv.into_iter().for_each(|z| {
        let input_file = BufReader::new(File::open(&input_path).unwrap());

        let voxel = if tiling {
            Voxelizer::<Tile>::voxelize_from_jpr_las(input_file, origin, z.zoom_lv, swap)
        } else {
            Voxelizer::<Fit>::voxelize_from_jpr_las(input_file, origin, z.zoom_lv, swap)
        };

        voxel.into_iter().for_each(|(tile_idx, voxel_mesh)| {
            let export_file_name = format!("{}-{}-{}-{}.ply", input_file_name, z.zoom_lv as u8, tile_idx[0], tile_idx[1]);
            let export_file_path = Path::new(&output_path).join(export_file_name);
            let mut output_file = File::create(&export_file_path).unwrap();

            let buf = match file_format {
                "Ascii" => PlyStructs::from_voxel_mesh(voxel_mesh).to_ascii_ply_buf(),
                "Binary (little endian)" => PlyStructs::from_voxel_mesh(voxel_mesh).to_binary_little_endian_ply_buf(),
                "Binary (big endian)" => PlyStructs::from_voxel_mesh(voxel_mesh).to_binary_big_endian_ply_buf(),
                _ => panic!("Invalid file format"),
            };

            output_file.write_all(&buf).expect("Failed to write to file");
            output_file.flush().expect("Failed to flush the file");
            println!("[log] Exported to \"{}\"", export_file_path.to_str().unwrap());
        });
    });

    println!("Finished");
    Ok(())
}

