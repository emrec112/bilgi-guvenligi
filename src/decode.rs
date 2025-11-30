use steganography::{decoder::Decoder, util::{bytes_to_str, file_as_image_buffer}};


pub fn decode_img_to_string(hidden_msg_path:String){
    let encoded_img = file_as_image_buffer(hidden_msg_path.to_string());
    let dec = Decoder::new(encoded_img);
    let out_buffer = dec.decode_alpha();

    let clean_buffer: Vec<u8> = out_buffer.into_iter()
                                    .filter(|b| {
                                        *b != 0xff_u8
                                    })
                                    .collect();
    let mes = bytes_to_str(&clean_buffer);
    println!("{:?}", mes);
}