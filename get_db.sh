#t/usr/bin/env bash

if ! test -d ./local_data/; then 
   printf "It seems like you do not have a local_data/ directory, please create this directory and ignore it in version control" >&2
   exit 100
fi

if ! docker ps >& /dev/null; then
    printf "Cannot find docker daemon, this could mean you do not have permission to use docker with your user
    Hint: Start the docker daemon with dockerd if you haven't already. 
    You can give yourself the permission (go rootless) or use sudo\n" >&2
    exit 2
fi

   # old
   # ip=""
   # local count=0
   # while [[ -z "$ip" ]];
   # do
   #    sleep 0.1
   #    ((count++))
   # done
   
function url_stuff() {
   # You can change the port number if you wish
   echo "postgresql://postgres:$1@0.0.0.0:65432"
}

id=$(cat ./local_data/id)
if 
   [[ -f ./local_data/id ]] && 
   docker ps --no-trunc --all | grep -q "$id" &&
   [[ "$(docker container inspect -f '{{.State.Running}}' "$id")" = "true" ]]; 
then
   password="$(cat ./local_data/db_password)"
   url_stuff "$password" 
else
   # The password is not secure because its purpose is to prevent accedental local connections
   # Also I do not want to work with more dependencies
   password=$RANDOM
   id=$(docker run --detach -p "65432:5432" -e POSTGRES_PASSWORD=$password postgres )

   echo "$id" > ./local_data/id
   echo $password > ./local_data/db_password
fi


