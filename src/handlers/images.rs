//! Contains handlers for all images-related functionality.

use std::{env::var, sync::Arc};

use log::{error, info};
use teloxide::{
    net::Download,
    prelude::{Requester, ResponseResult},
    types::{Message, PhotoSize},
};
use tokio::fs::File;

use crate::{
    bot::Bot,
    models::image::{NewImage, create_image},
    modules::database::PgPool,
};

/// Handles all incoming images (saves them).
pub async fn images_handler(
    bot: Bot,
    message: Message,
    images_sizes: Vec<PhotoSize>,
    pool: Arc<PgPool>,
) -> ResponseResult<()> {
    let largest_image = match images_sizes.last() {
        Some(image) => image,
        None => return Ok(()),
    };

    let file_info = match bot.get_file(&largest_image.file.id).await {
        Ok(file_info) => file_info,
        Err(e) => {
            error!(target: "images_handler", "Failed to get file info for file_id {}: {e}", largest_image.file.id);
            return Ok(());
        }
    };

    let path = var("IMAGES_PATH").unwrap_or_else(|_| "images".to_owned());
    let ext = file_info.path.rsplit('.').next().unwrap_or("jpg");
    let filename = format!("{}/{}.{}", path, largest_image.file.id, ext);

    let mut file = match File::create(&filename).await {
        Ok(file) => file,
        Err(e) => {
            error!(target: "images_handler", "Unable to create a file {}: {e}", filename);
            return Ok(());
        }
    };

    match bot.download_file(&file_info.path, &mut file).await {
        Ok(()) => {
            info!("Saved image from {}: {}", message.chat.id, filename);
        }
        Err(e) => {
            error!(target: "images_handler", "Failed to download file {}: {e}", file_info.id);
            return Ok(());
        }
    }

    let new_image = NewImage {
        chat_id: message.chat.id.0,
        image_id: &largest_image.file.id,
    };

    if let Err(e) = create_image(pool, new_image).await {
        error!(target: "images_handler", "Failed to save the image {}: {e}", filename);
    }

    Ok(())
}
