# Yet Another Image Resizer

Yet Another Image Resizer is a super simple image resizing tool, developed as part of my Rust journey on [https://learning.accelerant.dev/](https://learning.accelerant.dev/).

## Usage

To use the Yet Another Image Resizer tool, follow the syntax below:

```
yair myimage.extension percentage
```

For example, to create a resized version of the image "bear-combat.jpg" that is 25% of the original size, use the following command:

```
yair .\sample\bear-combat.jpg 25
```

## Note

Please note that when running in debug mode, the resize process may be slow. This is a known behavior of the resize method implemented in the `image` crate.