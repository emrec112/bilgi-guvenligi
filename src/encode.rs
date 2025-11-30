use steganography::{encoder::{ Encoder}, util::{file_as_dynamic_image, save_image_buffer, str_to_bytes}};

pub fn write_mes_to_file(mes:String, img_path:String){
    
    let loadbyte = str_to_bytes(&mes);
    let destination_img = file_as_dynamic_image(img_path.to_string());
    let enc = Encoder::new(loadbyte, destination_img);
    let result = enc.encode_alpha();

    save_image_buffer(result, "saklanmis_mesaj.png".to_string())
}