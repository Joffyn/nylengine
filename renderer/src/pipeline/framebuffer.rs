use crate::pipeline::prelude::*;


//pub fn create_framebuffer(
//    renderpass: Arc<RenderPass>,
//    image: Arc<Image>)
//    -> Result<Arc<Framebuffer>, Validated<VulkanError>>
//{
//    let view = ImageView::new_default(image.clone())?;
//
//    Ok(Framebuffer::new(
//        renderpass.clone(),
//        FramebufferCreateInfo
//        {
//            attachments: vec![view],
//            ..Default::default()
//        },
//    ))
//}

pub fn get_framebuffers(
    images: &[Arc<Image>],
    render_pass: &Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> 
{
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}
