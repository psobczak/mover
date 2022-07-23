# Mover

Simple program that automatically moves images from your default `downloads` folder into default `pictures` folder. Should work on all platforms. 

Following image file extensions are supported: `png, jpg, jpeg, svg, tif, tiff, bmp, gif, eps`

Every file that program moves should be logged to:
* Linux: `~/home/$USER/.config/mover/mover.log`
* Windows: `C:\Users\$USER\AppData\Roaming\Sobczak\mover`
* macOS: `/Users/$USER/Library/Application Support/com.Sobczak.mover`

Sample log: 
```
20:57:05 [INFO] Moved 0347a9aa-e396-49a5-b0f1-31261704bab8-profile_image-70x70 to: /home/psobczak/Downloads/0347a9aa-e396-49a5-b0f1-31261704bab8-profile_image-70x70.jpeg
09:51:19 [INFO] Moved 0125ef17-4375-4fcd-b6ed-b1eadeb1297e-profile_image-70x70 to: /home/psobczak/Downloads/0125ef17-4375-4fcd-b6ed-b1eadeb1297e-profile_image-70x70.png
```

## Requirements:
Rust and cargo installed

## How to use
Install script sets up new cron job that runs on reboot. For obvoius reasons script works only on Linux and macOs.

1. `git clone`
2. `cd mover`
3. `chmod +x ./install.sh`
4. `./install.sh`

