Before you start this binary, create a .env file in the same
directory to configure the application

```env
WEB_PORT=8080
IP_PATH=config/ip.conf
TARGET_PORT=7777
AUTH=CHANGEME
```

- WEB_PORT sets the port where the site will be served
- IP_PATH is the (absolute or relative) path to a file which should
contain the IP-address of the Server running the data connection part
- TARGET_PORT Port where the data connection is listening
- AUTH The password which you set for the data connection
