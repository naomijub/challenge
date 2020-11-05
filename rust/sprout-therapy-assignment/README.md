# About
This project uses Rust with Actix and Bastion. Actix is the current benchmark for Rust WebDevelopment alongside Rocket and my preferred framework. Bastion is a supervisor like system that helps your system become more fault tolerant as it restarts your system when crashes occur. For logging I am using `actix_web::middleware::Logger`, `log` and `env-Logger` with the log format being `"IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D X-REQUEST-ID:%{x-request-id}o"`. Basic log level is `INFO`. Parameter are self explanatory while `x-request-id` is created by `actix_web::middleware::DefaultHeaders` as being a simple `UUID`, this helps track requests when a monitoring system is available.

# Running the code

For docker:
`make run`

For local (Rust and Cargo required):
`make run-local`
* Make sure to kill the process after, due to bastion this process doesn't die with a simple `ctrl+c`.

# Basic route configuration:

1. GET on `/ping` returns a simple message with `pong`. This endpoint can be used as a Healthchek. When queried the log output is:
```
[2020-11-05T02:16:25Z INFO  actix_web::middleware::logger] IP:127.0.0.1:59533 DATETIME:2020-11-04T23:16:25-03:00 REQUEST:"GET /ping HTTP/1.1" STATUS: 200 DURATION:0.503000 X-REQUEST-ID:bfac3ccc-d273-4728-8661-1eb726f8b2ee
```

2. GET on  `/~/ready` returns Status Code 202 (`ACCEPTED`) or 500 (`INTERNAL SERVER ERROR`). This endpoint can be used as a readiness probe for kubernetes. When queried the log output is:
```
[2020-11-05T02:25:08Z INFO  actix_web::middleware::logger] IP:127.0.0.1:59621 DATETIME:2020-11-04T23:25:08-03:00 REQUEST:"GET /~/ready HTTP/1.1" STATUS: 202 DURATION:6.898000 X-REQUEST-ID:9498761c-ced5-4cf0-b9f2-2ee8804c748f
```

3. POST on `/data` 

4. GET on any other route will return Status Code 404 ( `NOT FOUND`). Log output for route `/~/sdgdfhs` is:
```
[2020-11-05T02:28:07Z INFO  actix_web::middleware::logger] IP:127.0.0.1:59653 DATETIME:2020-11-04T23:28:07-03:00 REQUEST:"GET /~/sdgdfhs HTTP/1.1" STATUS: 404 DURATION:0.178000 X-REQUEST-ID:a50719ca-0afa-47ab-a54c-dc4a6e337b8c
```