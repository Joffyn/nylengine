use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage, AllocateImageError};
use vulkano::format::Format;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::Validated;
use vulkano::VulkanError;
use std::sync::Arc;


pub fn create_image(malloc: Arc<StandardMemoryAllocator>) -> Result<Arc<Image>, Validated<AllocateImageError>>
{

    Image::new(
        malloc.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
}
