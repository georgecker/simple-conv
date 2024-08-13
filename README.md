```
   .dMMMb  dMP dMMMMMMMMb dMMMMb  dMP     dMMMMMP 
  dMP" VP amr dMP"dMP"dMPdMP.dMP dMP     dMP      
  VMMMb  dMP dMP dMP dMPdMMMMP" dMP     dMMMP     
dP .dMP dMP dMP dMP dMPdMP     dMP     dMP        
VMMMP" dMP dMP dMP dMPdMP     dMMMMMP dMMMMMP     
                                                  
   .aMMMb  .aMMMb  dMMMMb  dMP dMP                
  dMP"VMP dMP"dMP dMP dMP dMP dMP                 
 dMP     dMP dMP dMP dMP dMP dMP                  
dMP.aMP dMP.aMP dMP dMP  YMvAP"                   
VMMMP"  VMMMP" dMP dMP    VP"                     
                                                      
```  
<div align="right">
  <p align="center">
    >> A simple command line tool for image conversion <<
  </p>
</div>

## About The Project
This is a quick little and simple command line tool to convert almost any image format to almost any image format. I personally run it during the build process of my homemade Tamagotchi clone, to convert JPEGs and PNGs to BMP for the pet sprites. 

## Supported Image Formats

This tool is a wrapper of the [image crate](https://github.com/image-rs/image), and therefore supports the same formats and encodings.

| Format   | Decoding                                  | Encoding                                |
| -------- | ----------------------------------------- | --------------------------------------- |
| AVIF     | Yes (8-bit only) \*                       | Yes (lossy only)                        |
| BMP      | Yes                                       | Yes                                     |
| DDS      | Yes                                       | ---                                      |
| Farbfeld | Yes                                       | Yes                                     |
| GIF      | Yes                                       | Yes                                     |
| HDR      | Yes                                       | Yes                                     |
| ICO      | Yes                                       | Yes                                     |
| JPEG     | Yes                                       | Yes                                     |
| EXR      | Yes                                       | Yes                                     |
| PNG      | Yes                                       | Yes                                     |
| PNM      | Yes                                       | Yes                                     |
| QOI      | Yes                                       | Yes                                     |
| TGA      | Yes                                       | Yes                                     |
| TIFF     | Yes                                       | Yes                                     |
| WebP     | Yes                                       | Yes (lossless only)                     |

## Getting Started

1. Just clone the repo with
```sh
git clone https://github.com/georgecker/simple-conv.git
cd simple-conv
```

2. Now simple use cargo to build and run. The formate of the output file will be determined by the provided file extensions.
```sh
cargo run -- input_file.jpeg output_file.png
```
