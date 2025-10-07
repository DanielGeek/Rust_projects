# Running Your First Docker Container

Great, we have Docker up and running on our system. let's go ahead and use it to run an Ubuntu container on our computer. It doesn't matter what computer we're running, we can run Ubuntu on it!

Run `docker run -it ubuntu` command to get started

```sh
$ docker run -it ubuntu
Unable to find image 'ubuntu:latest' locally
latest: Pulling from library/ubuntu
7bdf644cff2e: Pull complete 
Digest: sha256:728785b59223d755e3e5c5af178fab1be7031f3522c5ccd7a0b32b80d8248123
Status: Downloaded newer image for ubuntu:latest
root@8d5d2a2ae2b3:/#
```

After pulling down the Ubuntu image we're dropped into an Ubuntu interactive shell. Ubuntu is a popular Linux [distro](https://en.wikipedia.org/wiki/Linux_distribution), and we are now running it inside of our own system, regardless of what Operating System we're currently using! If you're familiar with Linux you can poke around using commands like `ls` and `cd`. Please note that this is a bare bones version of Ubuntu. Many commands are not included like vim or even nano.

When you're finished poking around you can exit the container with the command `exit`. This will drop you back into your normal shell on your system.

```sh
root@8d5d2a2ae2b3:/# exit
exit
```

Congratulations, your have now run your first Docker container on your computer!
