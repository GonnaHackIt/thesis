# Benchserv

Server agnostic application for testing servers latency.

## Demonstration

![](showcase.mp4)

## How to run

To run the application you need to:

1. Run the server you want to benchmark. For example:
`cargo run -p single_sync`
2. Compile the plugin of your choice and [place it](#plugins-path).
3. Compile the `app` crate and run it (or use provided binary for windows - app.exe). Assuming we are in the `benchserv` folder:
`cargo run -p app`
4. Set settings in the app and click button `Run`

## Plugins

Plugins are responsible for creating connection, sending and receving data from the server they communicate with. Every connection is spawned onto new async task so plugins shouldn't do it themselves. Plugins must implement the (`interface`)[/interface] in order to be compatible. To measure latency that will be shown on the chart, plugins must use `start` and `stop` methods on the `ConnectionTimer` object provided in the function arguments.

To implement the plugin you can use the (template)[/plugin_template]. You can check example implementations here:
- (basic_tcp)[/plugin]
- (web)[/plugin_web]

## Plugins Path

Program checks for `PLUGINS_PATH` environmental variable and if it doesn't exist it tries to read the `plugins` directory in the same path that program runs.

## Running Modes

There are two modes:

- Constant connections - program starts with specified amount of connections, spawning a new one when previous one ends. The X-axis of chart is time. Latency is averaged from all latencies across last second.
- Increasing connections - program starts with one connection spawning a new one every 10 requests (request = start/stop) or when the previous one ends. The X-axis of chart is number of connections. Latency is averaged across last 10 requests.



