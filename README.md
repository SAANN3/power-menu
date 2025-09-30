
# Power menu ![Power menu](readme/icon.svg) 

![alt text](readme/preview.gif)

Power menu built in Rust with [Dioxus](https://dioxuslabs.com/)!

[![GitHub release](https://img.shields.io/github/v/release/SAANN3/color-replacer?label=Download)](https://github.com/SAANN3/power-menu/releases/latest)
## How to build
```bash
git clone https://github.com/SAANN3/power-menu.git
cd power-menu
cargo build --release
```
The Built binary will be located in ```target/release/power-menu```

After that you can move the binary anywhere

## Usage

Add it to autostart and then, when you need to launch it, click on the power icon in the system tray

## Styling
On first start (or if config dir is empty), the program creates all files in config directory. Then you simply edit them.
Open ```$HOME/.config/power-menu/assets/main.css``` and edit the css class
```css
:root {
    --global-color: rgb(255, 255, 255); // icons, text, borders,  basically all 
    --global-bg-color: #28292a; // background color
}
```

## Migration
If you encountered some issues after updating to newer version, please see [CHANGELOG.md](CHANGELOG.md)