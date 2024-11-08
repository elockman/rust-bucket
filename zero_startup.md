# Image and update for mesh
### Flash the microSD drive with the following image:  
https://github.com/radxa-build/radxa-zero3/releases/download/b6/radxa-zero3_debian_bullseye_xfce_b6.img.xz

### Boot with this image
Login un=radxa, pw=radxa  
Connect to local WiFi  

### Update packages
sudo apt update  
sudo apt install -y net-tools  
sudo apt install -y wireless-tools  
sudo apt install -y isc-dhcp-client  
sudo apt install -y openssh-server openssh-sftp-server  
sudo apt install -y firmware-misc-nonfree  

sudo dhclient usb0  
sudo systemctl enable --now radxa-otgutils@udc0  
  
sudo systemctl restart ssh  
sudo systemctl enable ssh  
sudo systemctl restart networking  

sudo ip link set usb0 up  
sudo ip addr add 192.168.1.100/24 dev usb0  
sudo ip addr show usb0
sudo ip a  
  
### /etc/network/interfaces.d/usb  
auto usb0  
allow-hotplug usb0  
iface usb0 inet dhcp  
  
### Windows 10 PC  
Go to Control Panel > Network and Sharing Center > Change adapter settings.  
Right-click on the USB network adapter and select Properties.  
Select "Internet Protocol Version 4 (TCP/IPv4)" and click Properties.  
Choose "Obtain an IP address automatically" and Obtain DNS server address automatically.  
Click OK to save the settings.  
  
### Update hostname (choose one)
sudo hostnamectl set-hostname radxa-red  
sudo hostnamectl set-hostname radxa-white  
sudo hostnamectl set-hostname radxa-blue  

### /home/radxa/mesh_startup.sh (keep IP's unique)
#!/bin/bash  
sudo ip link set wlan0 down  
sudo ip addr flush dev wlan0  
sudo iw dev wlan0 interface add mesh0 type mp  
sudo ip link set mesh0 up  
sudo iw dev mesh0 mesh join Xmesh  
sudo ip addr add 10.1.100.10/24 dev mesh0  
echo "Connected to mesh network xmesh as mesh0 with IP address 10.1.100.10"  

#!/bin/bash  
sudo ip link set wlan0 down  
sudo iw dev wlan0 set type mp  
sudo ip link set wlan0 up  
sudo iw dev wlan0 mesh join Xmesh freq 2437 HT40+  
sudo ip addr add 10.1.100.10/24 dev wlan0  

### Forget WiFi Networks
sudo rm /etc/NetworkManager/system-connections/*  

### /etc/NetworkManager/system-connections/mesh-network.nmconnection
[connection]  
id=mesh-network  
type=wifi  
interface-name=mesh0  
autoconnect=true  
  
[wifi]  
ssid=Xmesh  
mode=mesh  
frequency=2437  
  
[wifi-security]  
key-mgmt=NONE  
  
[ipv4]  
method=manual  
address1=10.1.100.10/24,10.1.100.1  
dns=8.8.8.8;8.8.4.4  
  
[ipv6]  
method=ignore  
  
### /etc/wpa_supplicant/wpa_supplicant.conf
ctrl_interface=DIR=/var/run/wpa_supplicant GROUP=netdev  
update_config=1  
ap_scan=1  
  
network={  
    ssid="Xmesh"  
    mesh_id="Xmesh"  
    key_mgmt=SAE  
    psk="supersecretpass"  
    mode=5  
    frequency=2437  
}  

### Start WPA Supplicant
sudo wpa_supplicant -B -i mesh0 -c /etc/wpa_supplicant/wpa_supplicant.conf

### Verify Mesh Network Connection
sudo iw dev mesh0 link
 
### /etc/rc.local
#!/bin/sh -e  
/home/radxa/mesh_startup.sh  
exit 0  

### Make files executable
sudo chmod +x /etc/rc.local  
sudo chmod +x /home/radxa/mesh_startup.sh  

### Verify rc-local service is enabled
sudo systemctl enable rc-local  
sudo systemctl start rc-local  

### Set User & PW  
build/conf/local.conf  
EXTRA_USERS_PARAMS = "usermod -p $(openssl passwd -1 my_user_password) my_user_name;"  
  
### Reboot
sudo reboot  

### Build Driver for Debian  
sudo apt update  
sudo apt install -y build-essential dkms linux-headers-$(uname -r) git  
git clone https://github.com/openwrt/mt76.git  
cd mt76  
sudo mkdir /usr/src/mt76-1.0  
sudo cp -r * /usr/src/mt76-1.0  
sudo nano /usr/src/mt76-1.0/dkms.conf  
  
PACKAGE_NAME="mt76"  
PACKAGE_VERSION="1.0"  
BUILT_MODULE_NAME[0]="mt76"  
DEST_MODULE_LOCATION[0]="/updates/dkms"  
AUTOINSTALL="yes"  
MAKE[0]="make -C /lib/modules/$(uname -r)/build M=${PWD} modules"  
CLEAN="make -C /lib/modules/$(uname -r)/build M=${PWD} clean"  
  
sudo dkms add -m mt76 -v 1.0  
sudo dkms build -m mt76 -v 1.0  
sudo dkms install -m mt76 -v 1.0  

sudo modprobe mt76  
lsmod | grep mt76  
  
