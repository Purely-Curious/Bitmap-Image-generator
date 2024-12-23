
use std::{
    fs::{self, File},
    io::{self, Write},
};
pub struct BmpFileHeader
{
    signature: u16,
    file_size: u32,
    reserved_padding: u32,
    dataoffset: u32,
}
impl BmpFileHeader
{
    pub fn new(signature: u16, file_size: u32, reserved_padding: u32, dataoffset: u32) -> Self
    {
        Self
        {
            signature,
            file_size,
            reserved_padding,
            dataoffset,
        }
    }
    fn write_data_into_file(&self, file: &mut File)
    {
        let _ = file.write(&self.signature.to_be_bytes());
        _ = file.write(&self.file_size.to_be_bytes());
        _ = file.write(&self.reserved_padding.to_le_bytes());
        _ = file.write(&self.dataoffset.to_le_bytes());
    }
}

pub struct BmpInfoHeader
{
    size: u32,
    width: u32,
    height: u32,
    planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    image_size: u32,
    x_pixels_per_m: u32,
    y_pixels_per_m: u32,
    colors_used: u32,
    important_colors: u32,
    //color_table: ColorTable,
}

impl BmpInfoHeader
{
    pub fn new(size: u32, width: u32, height: u32, planes: u16,
        bits_per_pixel: u16, compression: u32, image_size: u32, x_pixels_per_m: u32,
        y_pixels_per_m: u32, colors_used: u32, important_colors: u32) -> Self
    {
        Self
        {
            size,
            width,
            height,
            planes,
            bits_per_pixel,
            compression,
            image_size,
            x_pixels_per_m,
            y_pixels_per_m,
            colors_used,
            important_colors,
        }
    }
    fn write_data_into_file(&self, file: &mut File)
    {
        let _ = file.write(&self.size.to_le_bytes());
        let _ = file.write(&self.width.to_le_bytes());
        let _ = file.write(&self.height.to_le_bytes());
        let _ = file.write(&self.planes.to_le_bytes());
        let _ = file.write(&self.bits_per_pixel.to_le_bytes());
        let _ = file.write(&self.compression.to_le_bytes());
        let _ = file.write(&self.image_size.to_le_bytes());
        let _ = file.write(&self.x_pixels_per_m.to_le_bytes());
        let _ = file.write(&self.y_pixels_per_m.to_le_bytes());
        let _ = file.write(&self.colors_used.to_le_bytes());
        let _ = file.write(&self.important_colors.to_le_bytes());
    }
}


pub struct BmpImage {
    fh: BmpFileHeader,
    ih: BmpInfoHeader,
    title: String,
    pub data: Vec<u32>,
}
impl BmpImage
{
    pub fn new(title: String, fh: BmpFileHeader, ih: BmpInfoHeader, data: Vec<u32>) -> Self
    {
        Self
        {
            fh,
            ih,
            title,
            data,
        }
    }
    pub fn write_data_into_file(&mut self, file: &mut File)
    {
        let len_of_data = self.data.len();
        for i in 0..len_of_data
        {
            for j in 1..=3
            {
                let item = self.data[i];
                let _ = file.write(&(((item >> (3 - j) * 8) as u8) & 0xff).to_be_bytes());
            }
             
            if (i+1) as u32 % self.ih.height == 0
            {
                let padding = (4 - ((self.ih.width * 3) % 4)) % 4;
                if padding != 0
                {
                    for _ in 0..padding
                    {
                        let _ = file.write(&(0 as u8).to_be_bytes());
                    }
                } 
            }
            
        }
    }
    pub fn to_file(&mut self) -> File
    {
        let mut file = File::create(self.title.as_str()).unwrap();
        self.fh.write_data_into_file(&mut file);
        self.ih.write_data_into_file(&mut file);
        self.write_data_into_file(&mut file);
        file
    }

    pub fn set_pixel_data(&mut self, x: u32, y: u32, color: u32)
    {
        let len = self.data.len()-1;
        self.data[len - (y * self.ih.width + x) as usize] = color;
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> u32
    {
        let len = self.data.len()-1;
        self.data[len - (y * self.ih.width + x) as usize]
    }

    pub fn create_bitmap_image(filename: &str, width: u32, height: u32, data: Vec<u32>) -> BmpImage {
        // signature: u16,
        // file_size: u32,
        // reserved_padding: u32,
        // dataoffset: u32,
        //
        // size: u32,
        // width: u32,
        // height: u32,
        // planes: u16,
        // bits_per_pixel: u16,
        // compression: u32,
        // image_size: u32,
        // Xpixels_per_m: u32,
        // Ypixels_per_m: u32,
        // colors_used: u32,
        // important_colors: u32,
        // color_table: ColorTable,
        let signature: u16 = 0x42 << 8 | 0x4d;
        let file_size = 54 + 16; //data.len() as u32;
        let reserved_padding: u32 = 0000;
        let data_offset: u32 = 54;
    
        let fh = BmpFileHeader::new(signature, file_size, reserved_padding, data_offset);
        let size: u32 = 40;
        let image_width: u32 = width;
        let image_height: u32 = height;
        let planes: u16 = 1;
        let bits_per_pixel: u16 = 24;
        let compression: u32 = 0;
        let image_size: u32 = image_height * image_width * 3;
        let Xpixels_per_m: u32 = 0;
        let Ypixels_per_m: u32 = 0;
        let colors_used: u32 = 0;
        let important_colors: u32 = 0;
        
        let ih = BmpInfoHeader::new(size, image_width, image_height, planes,
            bits_per_pixel, compression, image_size, Xpixels_per_m,
            Ypixels_per_m, colors_used, important_colors);
        
    
        //let mut image =
        BmpImage::new(filename.to_string(), fh, ih, data)
        // image.to_file()
    }

}
// 0000 0000
// 0000 56D0
// 0000 0001
// 0001 0000
