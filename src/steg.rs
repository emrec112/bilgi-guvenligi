use steganography::util::str_to_bytes;

fn write_mes_to_file((mes, destination_img) : (String, String)){
    
    let loadbyte = str_to_bytes(&msg);
    let enc = Encoder::new(loadbyte, destination_img);
    let result = enc.encode_alpha();

    save_image_buffer
}