version=`curl -s https://ci.loohpjames.com/job/InteractiveChat/lastSuccessfulBuild/api/json | jq -r '.artifacts[0].fileName'`
wget https://ci.loohpjames.com/job/InteractiveChat/lastSuccessfulBuild/artifact/target/$version  -nv -O $1/InteractiveChat.jar