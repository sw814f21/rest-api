rsync -r --info=progress2 --exclude target/ --exclude .env  ./ p8:/var/smiley_rest_api/
ssh p8 "
    source /opt/rust/env &&
    cd /var/smiley_rest_api &&
    cargo build --release &&
    sudo -S systemctl restart smiley_rest_api.service
"

clear
echo Done with deployment.