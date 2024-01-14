# Server Resource Monitor

This is a simple program to provide an HTTP endpoint to see various server stats.
Right now, the stats shown include:

- Number of CPU cores
- Load over the last minute, five minutes, and fifteen minutes
- Free and available memory
- Uptime

Example output:

```
CPU cores: 12
CPU speed: 984 MHz
Load 1m:   0.37 %
Load 5m:   0.74 %
Load 15m:  0.54 %
-----------------------
Memory
-----------------------
Total Memory: 39959 MB
Free:         30622 MB
-----------------------
Total processes: 1162

==================
Uptime: 00 days 00:51:29.002
0.0
```

This is served over the dead simple `text/plain` MIME type.

# Usage
```
sudo docker build -t resource_monitor . && sudo docker run --rm -it -e ROCKET_CONFIG=/app/Rocket.toml -p 8001:8001 resource_monitor
```

To use in Docker Compose:
```
resource_monitor:
    container_name: resource_monitor
    build: https://github.com/steelswords/server_resource_monitor.git
    ports:
        - 8000:8001
    environment:
        - ROCKET_CONFIG=/app/Rocket.toml

```
