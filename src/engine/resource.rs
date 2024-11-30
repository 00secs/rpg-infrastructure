use crate::EError;
use png::Decoder;
use std::fs::File;

/// 外部リソースを管理するオブジェクト。
//
// NOTE: 将来的にリソースをすべて.datファイルにまとめて
//       常にオープンしておいて適宜シークして各リソースを読み出すために
//       このオブジェクトを介す。
pub struct ResourceManager;

impl ResourceManager {
    pub fn load_png(&self, id: &str) -> Result<(Vec<u8>, u32, u32), EError> {
        // TODO: .datファイルから読み出す
        let file = File::open(format!("img/{id}.png"))?;

        let mut reader = Decoder::new(file).read_info()?;
        let mut buffer = vec![0; reader.output_buffer_size()];
        let output_info = reader.next_frame(&mut buffer)?;

        Ok((buffer, output_info.width, output_info.height))
    }
}
