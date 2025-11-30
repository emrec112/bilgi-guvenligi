use crate::{decode::decode_img_to_string, encode::write_mes_to_file};
mod web;
mod encode;
mod decode;
fn main() {
    //start_server();
    write_mes_to_file("mesaj.emre".to_string(), "/home/emrecc/dev/bilgi-guvenligi/src/miyabiicon.png".to_string());
    decode_img_to_string("/home/emrecc/dev/bilgi-guvenligi/saklanmis_mesaj.png".to_string());
}
