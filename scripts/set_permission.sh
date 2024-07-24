#!/bin/bash -eu

cd /etc/udev/rules.d

sudo touch ./99-hidraw-permissions.rules

sudo echo "KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0660", GROUP="plugdev"" > ./99-hidraw-permissions.rules

