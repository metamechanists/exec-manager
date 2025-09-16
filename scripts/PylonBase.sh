url=`curl -s https://api.github.com/repos/pylonmc/pylon-base/releases/latest | jq -r ".assets" | jq -c '[ .[] | select( .name | contains("jar")) ]' | jq -r '.[0].browser_download_url'`
wget $url -nv -O $1/PylonBase.jar
