# Configure Systemd
* Check paths in `start.sh` and `zeroweather.service`
* Copy `zeroweather.service` to `/lib/systemd/system/`
* Run `sudo systemctl enable zeroweather.service`
* Run `sudo systemctl start zeroweather.service`
* Check status by running `sudo systemctl status zeroweather.service`

Output should be something like:
```
● zeroweather.service - Temperature logger for Rapberry Pi Zero 2 W
     Loaded: loaded (/lib/systemd/system/zeroweather.service; enabled; preset: enabled)
     Active: active (running) since Fri 2025-07-25 12:09:48 CEST; 47s ago
   Main PID: 510 (bash)
      Tasks: 3 (limit: 173)
        CPU: 41ms
     CGroup: /system.slice/zeroweather.service
             ├─510 /bin/bash /home/petste/ZeroWeather/start.sh
             └─518 /home/petste/ZeroWeather/zeroweather --config=/home/petste/ZeroWeather/config/config.toml

Jul 25 12:09:48 zeroeast systemd[1]: Started zeroweather.service - Temperature logger for Rapberry Pi Zero 2 W.
```