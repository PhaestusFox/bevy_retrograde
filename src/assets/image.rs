use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use fixedbitset::FixedBitSet;
use image::{io::Reader as ImageReader, RgbaImage};

use crate::*;

/// An LDtk map asset
#[derive(TypeUuid)]
#[uuid = "48d2e3c8-2f48-4330-b7fe-fac3e81c60f3"]
#[derive(Clone, Debug)]
pub struct Image {
    pub image: RgbaImage,
    pub collision: FixedBitSet,
}

impl From<RgbaImage> for Image {
    fn from(image: RgbaImage) -> Self {
        // Calculate collision bitset
        let mut collision = FixedBitSet::with_capacity(image.pixels().len());
        for (i, pixel) in image.pixels().enumerate() {
            // For every non-fully transparent pixel add a collision indicator to the bitset
            if pixel.0[3] != 0 {
                collision.set(i, true);
            }
        }

        Image { image, collision }
    }
}

/// An error that occurs when loading an image file
#[derive(thiserror::Error, Debug)]
pub enum ImageLoaderError {
    #[error("Error parsing image: {0}")]
    ImageError(#[from] image::ImageError),
}

/// An image asset loader
#[derive(Default)]
pub(crate) struct ImageLoader;

impl AssetLoader for ImageLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        // Create a future for the load function
        Box::pin(async move { Ok(load_image(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        &[
            #[cfg(feature = "gif")]
            "gif",
            #[cfg(feature = "jpeg")]
            "jpeg",
            #[cfg(feature = "jpeg")]
            "jpg",
            #[cfg(feature = "png")]
            "png",
            #[cfg(feature = "tga")]
            "tga",
            #[cfg(feature = "tiff")]
            "tiff",
            #[cfg(feature = "webp")]
            "webp",
            #[cfg(feature = "bmp")]
            "bmp",
        ]
    }
}

async fn load_image<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<(), ImageLoaderError> {
    // Create a cursor over our bytes to let the image reader `Seek` insdie of them
    let reader = std::io::Cursor::new(bytes);

    // Load the image
    let image = ImageReader::new(reader)
        .with_guessed_format()
        .unwrap() // Unwrap because we know the `&[u8]` will return no IO Error
        .decode()?
        .to_rgba8();

    load_context.set_default_asset(LoadedAsset::new(Image::from(image)));

    Ok(())
}
