# Image dimmer

## Overview

Takes an image ither from file or fetch one from the given url and create a new image with a color gradient
as an overlay. The gradient go form non transparent at the bottom up to 60% of the image height where it is
fully transparent and then add overlay color on its way up to the top of the image.

There are three options to select what color to use for the overlay:

- The most dominat color in the image using k-means
- The most dominat color from the bottom row of the image
- User supplied RGB values

The output image encoding is defined by it's suffix for example png or webp

## Usage

```
cargo run -- --url "https://some.image.url.jpg" --output-file test.png --gradient-variant user-defined --rgb 161,49,44

cargo run -- --url "https://some.image.url" --output-file test.png
```
