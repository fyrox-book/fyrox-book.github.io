# Game Server Hosting 

The main issue with today's Internet is that it is mostly based on very old IPv4, which supports around 4 billion
IP addresses and pretty much all of them are already allocated between companies and individuals. Nowadays, internet
connection relies on network address translation (NAT) to be able to have multiple devices with their own IP addresses.
This fact makes it impossible to connect two devices directly to each other, because the devices simply do not
have a stable IP address anymore (it is defined by NAT). This could be solved by IPv6, which essentially has almost 
unlimited number of addresses, yet its adoption is quite slow. 

Unfortunately, there is no easy way to connect two or more devices directly. One of the simplest ways is to rent a
virtual personal server (VPS) with a stable IP address and use it as a sort of "proxy" server that will be able to
handle connections between the devices. This process is called [hole punching](https://en.wikipedia.org/wiki/Hole_punching_(networking)).
It is used all the time in modern internet, but sometimes it isn't enough and the only solution is to host 
a dedicated game server on a VPS (or a more powerful, yet costly, real web server).

To bypass all these issues once and for all, your game should be able to run a dedicated game server, which should be 
deployed on a VPS/real server. The rule of thumb here is to develop and test your game in your local network, and only
when its minimal valuable version is ready, rent a server and test it with players across the globe.