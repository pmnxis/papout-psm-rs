# OpenOCD Issue Notes

## OpenOCD 0.11
This project debugged with STLINK-V3 , STLINK-V3 is capable with OpenOCD 0.11.

https://github.com/xpack-dev-tools/openocd-xpack/releases

```shell
# Tested in Ubuntu 20.04
# if you don't have xpm install it. 
# https://xpack.github.io/xpm/install/
sudo apt purge openocd
xpm install --global @xpack-dev-tools/openocd@latest
sudo ln -s /home/$USER/.local/xPacks/@xpack-dev-tools/openocd/0.11.*/.content/bin/openocd /usr/bin/openocd
```

## STLink Permission Issue on Linux
- Add or Edit `/etc/udev/rules.d/50-usb-stlink.rules` and write below contents
```
#FT232
#STLINK V1
ATTRS{idProduct}=="3744", ATTRS{idVendor}=="0483", MODE="666", GROUP="plugdev"
#STLINK V2 and V2.1
ATTRS{idProduct}=="3748", ATTRS{idVendor}=="0483", MODE="666", GROUP="plugdev"
#STLINK V3
ATTRS{idProduct}=="374f", ATTRS{idVendor}=="0483", MODE="666", GROUP="plugdev"
```

- Restart udev
```shell
sudo service udev restart
```

- Reconect STLINK
