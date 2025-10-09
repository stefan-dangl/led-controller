# esp32-led-controller

Fully functional IoT LED Controller. Allows you to control the colors of an LED strip via smartphone or PC.

## Required hardware

This project is based on the ESP-S3 MCU. The following components were used:
- Seeed Studio XIAO ESP32-S3
- WS2812 Neo Pixel Led Light Strip

The code should work with similar hardware as well, requiering more or less adaptation.

<figure id="fig1-schematics">
  <img src="docs/images/schematics.jpg" alt="Circuit schematic" width="600" height="200">
  <figcaption>Figure 1: Schematics drawn with Wokwi.</figcaption>
</figure>


## How to flash

1) This project heavily uses the ESP-IDF framework. You can find useful information about it [here](https://docs.esp-rs.org/std-training/01_intro.html).
2) Execute the following commands in the project's root directory for building the project and flashing it on the connected ESP32-S3. Hint: If flashing doesn't work, you may have to modify the permissions of the device descriptor.

``` Bash
cd led-controller
cargo build --release
espflash flash target/xtensa-esp32s3-espidf/debug/led-controller --monitor
```


## How to use

Once the ESP32 is flashed, you can use it as follows:

1) When powered on, the ESP will open a WiFi access point with an SSID like *LED Controller 000* (configurable). Please don't forget to connect the antenna to your ESP.

<figure id="fig1-schematics">
  <img src="docs/images/ESP-AP.png" alt="LED Controller AP" width="400" height="350">
  <figcaption>Figure 2: LED Controller Access Point</figcaption>
</figure>

2) When connected to the network provided by the ESP, enter the IP address **192.168.71.1** in your browser. This will lead you to a welcome page where you can either connect the ESP to your WiFi or control the colors directly.

<figure id="fig1-schematics">
  <img src="docs/images/welcome_page.png" alt="Welcome Page" width="450" height="600">
  <figcaption>Figure 3: Welcome Page, shown when entering 192.168.71.1 in your browser.</figcaption>
</figure>

3) When clicking on *Connect to WiFi*, the ESP will take a moment to scan the available WiFi networks and list them. To connect, choose a WiFi from the list. If successful, the IP address where you can find the ESP in your WiFi network will appear. Hint: Bookmark this IP address so you can always find it easily.

<figure id="fig1-schematics">
  <img src="docs/images/connect_to_wifi.png" alt="Connect to WiFi" width="450" height="600">
  <figcaption>Figure 4: Connect to WiFi</figcaption>
</figure>

4) You can modify the colors using the color panel, which provides three functionalities: freely choosing a color, activating rainbow mode, or turning off the LEDs.

<figure id="fig1-schematics">
  <img src="docs/images/color_panel.png" alt="Color Panel" width="450" height="620">
  <figcaption>Figure 5: Color Panel, control the colors of your Led Strip.</figcaption>
</figure>



## Notes

I built some LED Controllers for my family and friends. If you need help assembling them or want pre-soldered ones, let me know.
