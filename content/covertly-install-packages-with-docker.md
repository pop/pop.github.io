+++
title = "Covertly Installing Packages with Docker"

date = "2015-10-07"

description = "A simple guide on install $PACKAGE with $CONTAINER_RUNTIME."

taxonomies.tags = [
    "docker"
]
+++

[Jump to the codey bits](#the-deets-dockerfile-and-commands)

## The Problem: Workstations

At the OSL I use workstations which are provisioned by
[Chef](https://en.wikipedia.org/wiki/Chef_%28software%29) to look more
or less identical (packages are the same and global config files are
consistent from machine to machine). This is nice because it allows
everybody at the lab to jump from workstation to workstation without
dreading the inevitable setup that usually comes with going to a new
computer.

The only downside here is that to install a package you have to make a
pull request to a GitHub Repository in which you modify a JSON file.
This is almost always for my own good, but as a dev I am prone to
avoiding things that the admins say are good for me (like broccoli,
milk, and consistent workstation environments).

For instance, yesterday I wanted to use
[youtube-dl](http://rg3.github.io/youtube-dl/) to grab a video I was
watching on repeat\*. I could have made a pull request to add the
package, waited until 30 after for the workstations to refresh, used the
package, and went about my day. This would have been the correct and
bureaucratic way to do things.

I *could* have, and probably *should* have, done that... but I didn't.

\* firefox kept crashing, I wasn't just stealing music for the sake of
it.

## Enter: Docker

In this case the solution to the 'problem' was
[Docker](https://en.wikipedia.org/wiki/Docker_%28software%29). I wrote a
Dockerfile which provisioned a container to have `youtube-dl` installed,
spun up the container with a shared directory (`$PWD:/home/`), and
executed the youtube-dl command. The .mp4 video was downloaded to my
current working directory and I was able to play it with my media player
of choice, all without installing `youtube-dl` locally.

## The Deets: Dockerfile and Commands

`Dockerfile`

```txt
FROM ubuntu:latest

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get -y install python-pip
RUN pip install youtube-dl

WORKDIR /home
RUN alias youtube-dl='/usr/local/bin/youtube-dl'
```

If you are in the directory containing the above Dockerfile, build the
container with:

```txt
$ docker build -t yt-dl .
```

and run it with:

```txt
$ docker run -v $PWD:/home/ yt-dl youtube-dl <YOUTUBE-VID-URL>
```

This will download any url's video into the current working directory.

## Pro-Tip: Add an `Alias`

Add the following content to your `~/.bashrc` file and then run
`source ~/.bashrc` to get rid the of long Docker bits of the above
command.

```bash
export $VID_DIR=/path/to/downloaded/videos/
alias yt-dl='docker run -v $VID_DIR:/home/ yt-dl youtube-dl'
```

## Wait This is Overly Complicated

You're probably thinking: "Hey, couldn't you have just run
`pip install --user youtube-dl`?

Yes. I could have. But that wouldn't be fun and wouldn't have given me
an excuse to write a blog, post now would it Ms. Smarty Pantz.

------------------------------------------------------------------------

**drink the coolaid. come to the docker side.**
