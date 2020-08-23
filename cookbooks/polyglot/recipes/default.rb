ubuntu_mirror = 'http://mirror.csclub.uwaterloo.ca/ubuntu/'
ubuntu_release = 'bionic'
ubuntu_version = '18.04'
username = 'vagrant'
user_home = '/home/' + username
project_home = user_home + '/project/pokepalette' 
frontend_path = '/home/vagrant/project/pokepalette/pokepalette_frontend'

python3_packages = '/usr/local/lib/python3.6/dist-packages'
ruby_gems = '/var/lib/gems/2.5.0/gems/'

template '/etc/apt/sources.list' do
  variables(
    :mirror => ubuntu_mirror,
    :release => ubuntu_release
  )
  notifies :run, 'execute[apt-get update]', :immediately
end
execute 'apt-get update' do
  action :nothing
end
execute 'apt-get upgrade' do
  command 'apt-get dist-upgrade -y'
  only_if 'apt list --upgradeable | grep -q upgradable'
end
directory '/opt'
directory '/opt/installers'

package ['build-essential', 'cmake']

execute 'sudo apt-get install -y pkg-config'

execute 'sudo apt-get install -y libssh-dev' 

package ['python3', 'python3-pip', 'python3-dev']
execute 'pip3 install numpy'
execute 'pip3 install scikit-learn'
execute 'pip3 install scikit-image'

remote_file '/opt/installers/node-setup.sh' do
  source 'https://deb.nodesource.com/setup_14.x'
  mode '0755'
end
execute '/opt/installers/node-setup.sh' do
  creates '/etc/apt/sources.list.d/nodesource.list'
  notifies :run, 'execute[apt-get update]', :immediately
end
package ['nodejs']

execute "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable -y" do
  user username
  environment 'HOME' => user_home
end

execute '. $HOME/.cargo/env' do
  user username
  environment 'HOME' => user_home
end

execute '$HOME/.cargo/bin/cargo install wasm-pack' do
  user username
  environment 'HOME' => user_home
end

execute 'npm install' do
  cwd project_home
  user username
  environment 'HOME' => user_home
end