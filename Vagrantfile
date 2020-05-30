# reedis development environment
Vagrant.configure("2") do |config|
  # Base Image 
  config.vm.box = "ubuntu/trusty64"
  # Network
  config.vm.network "private_network", type: "dhcp"
  # Shared directories to the host machine
  config.vm.synced_folder "Server/", "/home/vagrant/Server", create: true
  config.vm.synced_folder "TSClient/", "/home/vagrant/TSClient", create: true
  config.vm.synced_folder "ssh/", "/home/vagrant/.ssh", create: true
  # Hardware config
  config.vm.provider "virtualbox" do |v|
    v.name = "dev_machine"
    v.memory = 1024
    v.cpus = 1
  end
  # Necessary shell execution
   config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y git curl
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    
    curl -sL https://deb.nodesource.com/setup_14.x | sudo -E bash
    apt-get install -y nodejs
    npm i typescript -g
   SHELL
end
