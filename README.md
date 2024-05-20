English | [日本語](README.ja.md)

# voxel-tiler-cli

![voxel](https://github.com/azishio/voxel-tiler-core-rs/assets/127939746/2c1402c1-03a1-4c05-af64-daa3ea2976a0)

Convert point cloud data in LAS or LAZ format to voxel-based meshes in PLY format. This tool
wraps [voxel-tiler-core](https://crates.io/crates/voxel-tiler-core) with a CLI interface.

## Installation

Download the binary for your environment from the Release page and pass it through, or follow the steps below to install
it.

### Install from Crates.io

```sh
cargo install voxel-tiler-cli
```

> [!NOTE]
> The executable installed by `cargo install` will be placed in the directory specified by Cargo's bin directory, which
> is typically `$HOME/.cargo/bin` on Unix-like systems and `%USERPROFILE%\.cargo\bin` on Windows.
> To run the executable, you can either specify the full path or add Cargo's bin directory to your `PATH` environment
> variable.
>
> ```shell
> # On Unix-like systems
> ~/.cargo/bin/voxel-tiler-cli
> 
> # or
> export PATH=$PATH:$HOME/.cargo/bin
> voxel-tiler-cli
> 
> # On Windows
> %USERPROFILE%\.cargo\bin\voxel-tiler-cli
> 
> # or
> set PATH=%PATH%;%USERPROFILE%\.cargo\bin
> voxel-tiler-cli
> ```
>
> reference: [Installing Binaries with cargo install](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html)

### Clone the repository and build

```sh
git clone git@github.com:azishio/voxel-tiler-cli.git
cd voxel-tiler-cli

cargo build --release
```

## Usage

```shell
voxel-tiler-cli
```

When you run the executable, you will be prompted to input the following information, and after the final confirmation,
the conversion will begin.

+ Split the output file based on tile coordinates
+ Path to the input file
+ Swap X and Y
+ Plane rectangular coordinate system origin
+ Zoom level for tile/pixel coordinates
+ Output directory
+ PLY file output format

### Split the output file based on tile coordinates

Default: No

```shell
? Tiling? (y/N)
```

Choose whether to split the output PLY files by tile coordinates or to combine them into a single file.

If you choose to split, the origin of each PLY file will be the top-left of the tile it belongs to (with the minimum
pixel coordinates).

If you choose not to split, the origin of the single PLY file will be the minimum coordinates of the element's
axis-aligned bounding box (AABB), meaning all vertices will be offset to have positive and minimal coordinates.

### Path to the input file

Default: None
Starting location: Current directory

```shell
? Input File Path:  /path/to/some_directory/
  /path/to/some_directory/filename1.las
  /path/to/some_directory/filename2.las
  /path/to/some_directory/filename3.laz
  /path/to/some_directory/child_directory  
[↑↓ to move, tab to autocomplete, enter to submit]
```

Enter the absolute path to the .las/.laz file you want to convert. As you type the path, valid .las/.laz files and
subdirectories within the specified directory will be displayed as suggestions. You can select these suggestions using
the up/down arrow keys and autocomplete with the Tab key.

The part of the path up to the last `/` will be treated as a directory during suggestion calculations.

### Swap X and Y

Default: No

```shell
? Swap X and Y? (y/N)
```

Choose whether to swap the X and Y coordinates of the input file.

### Display file information

The following is an example output:
x and y coordinates are swapped based on the previous question.

```shell
Las File Info

+------------------+---------+
| Number of Points | 6037715 |
+------------------+---------+
+------------+--------------------+-----------+----------+
| coord      | x                  | y         | z        |
+------------+--------------------+-----------+----------+
| max [m]    | -94283.511         | 23761.122 | 309.762  |
+------------+--------------------+-----------+----------+
| min [m]    | -94584.235         | 23482.154 | 269.359  |
+------------+--------------------+-----------+----------+
| center [m] | -94433.87299999999 | 23621.638 | 289.5605 |
+------------+--------------------+-----------+----------+
```

### Plane rectangular coordinate system origin

Default: None

```shell
? JPR Origin:  
```

Enter the origin of the plane rectangular coordinate system that the input file is based on.

> [!TIP]
> This tool is designed for .las/.laz files with point clouds based on a plane rectangular coordinate system.
>
> However, since a plane rectangular coordinate system is a metric orthogonal coordinate system, you might get
> reasonable results with any metric point cloud data.
> Note that the coordinate accuracy of the output file will be lost in this case.
>
> It is recommended to set `Tiling` to `No` to combine the output into a single file in this scenario.

### Zoom level for tile/pixel coordinates

Default: None

```shell
? Select zoom levels  
^ [ ] ZoomLevel: 14 (  8.09 m/voxel)
  [ ] ZoomLevel: 15 (  4.04 m/voxel)
  [ ] ZoomLevel: 16 (  2.02 m/voxel)
> [ ] ZoomLevel: 17 (  1.01 m/voxel)
  [ ] ZoomLevel: 18 (  0.51 m/voxel)
  [ ] ZoomLevel: 19 (  0.25 m/voxel)
v [ ] ZoomLevel: 20 (  0.13 m/voxel)
[↑↓ to move, space to select one, → to all, ← to none, type to filter]
```

Select the zoom level at which the voxels will be generated. The corresponding edge length of each voxel at each zoom
level is displayed for reference.

The edge length of the voxel is calculated based on the center latitude of the input file.

### Output directory

Default: None
Starting location: Current directory

```shell
? Output Directory:  /path/to/some_directory/
  /path/to/some_directory/child_directory1
  /path/to/some_directory/child_directory2
[↑↓ to move, tab to autocomplete, enter to submit]
```

Enter the absolute path to the directory where the output files will be saved.

If the directory does not exist, it will be created automatically.

> [!WARNING]
> If `Tiling` is set to `Yes`, choose the destination directory carefully, as large numbers of files may be output.
> We recommend that you create a new directory.

### PLY file output format

Default: Ascii

```shell
? Select file format  
> Ascii
  Binary (little endian)
  Binary (big endian)
[↑↓ to move, enter to select, type to filter]
```

Select the format of the output PLY file.

### Final confirmation

Default: No

```shell
Params

+------------------------------------------------+
| InputFile  : /path/to/source/las_file.las      |
+------------------------------------------------+
| OutputDir  : /path/to/output/dir/              |
+------------------------------------------------+
| Ply Format : Ascii                             |
+------------------------------------------------+
| Tiling     : false                             |
+------------------------------------------------+

? Continue? (y/N)  
```

Review the input settings and confirm to start the conversion.

## License

Licensed under either of the following licenses:

+ Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

(Documentation comments and README files in English have been translated by DeepL and ChatGPT)
