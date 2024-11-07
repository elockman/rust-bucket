IMAGE_FEATURES += "ssh-server-openssh package-management"

#KERNEL_MODULE_AUTOLOAD += "ath9k_htc"

# DISTRO_FEATURES:append = " systemd"

IMAGE_INSTALL = "\
    packagegroup-core-boot \
    packagegroup-core-full-cmdline \
    packagegroup-base-wifi \
    packagegroup-base-usbgadget \
    kernel-modules \
    lrzsz \
    setserial \
    opkg \
    iperf3 \
    \
    nbench-byte \
    lmbench \
    \
    linux-firmware \
    linux-firmware-sd8686 \
    linux-firmware-sd8688 \
    linux-firmware-sd8787 \
    linux-firmware-sd8797 \
    linux-firmware-sd8801 \
    linux-firmware-sd8887 \
    linux-firmware-sd8897 \
    linux-firmware-ralink \
    linux-firmware-rtl8188 \
    linux-firmware-rtl8723 \
    linux-firmware-rtl8821 \
    linux-firmware-rtl8192cu \
    linux-firmware-rtl8192ce \
    linux-firmware-rtl8192su \
    linux-firmware-rtl8723 \
    mchp-wireless-firmware \
    \
    alsa-utils \
    i2c-tools \
    devmem2 \
    libgpiod-tools \
    dosfstools \
    libdrm-tests \
    mtd-utils \
    mtd-utils-ubifs \
    dtc \
    dtc-misc \
    iproute2 \
    iptables \
    bridge-utils \
    can-utils \
    evtest \
    python3-pyserial \
    python3-smbus \
    python3-ctypes \
    python3-pip \
    gdb \
    usbutils \
    wget \
    hostapd \
    9bit \
    rng-tools \
    bluez5 \
    wireless-regdb-static \
    libdrm \
    net-tools \
    nftables \
    phytool \
    tcpdump \
    kea \
    ${CORE_IMAGE_EXTRA_INSTALL} \
    "

IMAGE_INSTALL += "\
    packagegroup-base-usbhost \
    packagegroup-core-buildessential \
    apt \
    aws-c-iot \
    aws-cli \
    bc \
    bridge-utils \
    c-periphery \
    cjson \
    cmake \
    curl \
    czmq \
    dhcpcd \
    dnsmasq \
    dpkg \
    e2fsprogs \
    git \
    gnupg \
    gnutls \
    gssdp \
    inotify-tools \
    iw \
    keepalived \
    kernel-devsrc \
    libmicrohttpd \
    libmodbus \
    libnss-mdns \
    linuxptp \
    lshw \
    mbedtls \
    mdns \
    mosquitto \
    mpg123 \
    mpio \
    ntp \
    openssh \
    openssl \
    python3 \
    python3-dev \
    python3-periphery \
    python3-psutil \
    quagga \
    rsync \
    sqlite3 \
    usb-modeswitch \
    usb-modeswitch-data \
    zlib \
    \
    "
    
inherit core-image

#    kernel-modules \
#
