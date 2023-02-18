# server discovery

## Resolving server names
all servers have a name, and the name is used to identify the server by DNS.

### IP address and port as server name
if the server name is an IP address and a port, the server is resolved by the IP address and the port.

if no port is specified, the default port is `8848`.

### Domain name as server name
if the server name is a domain name, the server is resolved by the domain name. such as `CNAME` or `A` record.

### Web3 contract address as server name
if the server name is a web3 contract address, it means the server is adaptive, using the contract to find the real server.