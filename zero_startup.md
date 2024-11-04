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

### Forget WiFi Networks
sudo rm /etc/NetworkManager/system-connections/*  

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
sudo iw dev wlan0 interface add mesh0 type mp  
sudo ip link set mesh0 up  
sudo iw dev mesh0 mesh join Xmesh freq 2437  
sudo ip addr add 10.1.100.10/24 dev mesh0  

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

### Reboot
sudo reboot  

