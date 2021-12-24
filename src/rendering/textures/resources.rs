#[derive(Debug, Copy, Clone)]
pub enum TextureResources {
    Ball,
    Table,
    Bat00,
    Bat10,
    Impact0,
    Impact1,
    Impact2,
    Impact3,
    Impact4,
    Effect0,
    Effect1
}

pub fn get_texture_resource(texture: TextureResources) -> &'static[u8] {
    match texture {
        TextureResources::Ball => &include_bytes!("../../../images/ball.png")[..],
        TextureResources::Table => &include_bytes!("../../../images/table.png")[..],
        TextureResources::Bat00 => &include_bytes!("../../../images/bat00.png")[..],
        TextureResources::Bat10 => &include_bytes!("../../../images/bat10.png")[..],
        TextureResources::Impact0 => &include_bytes!("../../../images/impact0.png")[..],
        TextureResources::Impact1 => &include_bytes!("../../../images/impact1.png")[..],
        TextureResources::Impact2 => &include_bytes!("../../../images/impact2.png")[..],
        TextureResources::Impact3 => &include_bytes!("../../../images/impact3.png")[..],
        TextureResources::Impact4 => &include_bytes!("../../../images/impact4.png")[..],
        TextureResources::Effect0 => &include_bytes!("../../../images/effect0.png")[..],
        TextureResources::Effect1 => &include_bytes!("../../../images/effect1.png")[..],
    }
}