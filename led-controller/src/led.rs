// rainbow
// let mut hue = unsafe { esp_random() } as u8;
// loop {
//     let pixels = std::iter::repeat(hsv2rgb(Hsv {
//         hue,
//         sat: 255,
//         val: 8,
//     }))
//     .take(25);
//     ws2812.write_nocopy(pixels).unwrap();

//     delay.delay_ms(100);

//     hue = hue.wrapping_add(10);
// }
