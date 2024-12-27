+++
title = "Running Firefox in Docker"

date = "2016-03-05"

description = "You got Firefox in my Docker! You got Docker in my Firefox!"

taxonomies.tags = [
    "docker"
]
+++

Turns out you can run Firefox in Docker.
It's actually pretty easy:

## Code

**From** [my github
gist](https://gist.github.com/ElijahCaine/29e7e829341d58abe370).

<script src="https://gist.github.com/ElijahCaine/29e7e829341d58abe370.js"></script>

Installation instructions (on Linux):

```txt
# install docker and start the docker daemon
$ git clone https://gist.github.com/29e7e829341d58abe370.git docker-firefox
$ ln docker-firefox/ff-docker /some/path/for/binaries
$ ff-docker -b  # -b pulls & builds container, etc used for first time startup.
```

## Notes

I attempted to use [Alpine Linux](http://alpinelinux.org/) as a
proof-of-usability, since it's the hip new kid on the containerization
block[^1], but as it turns out Alpine's Firefox package is pretty
fucked[^2].

I use [dmenu](https://wiki.archlinux.org/index.php/Dmenu), this means I
can run `ff-docker` from my desktop without opening a terminal,
effectively making it a super-private-yet-convenient drop-in for
Firefox. Pretty neat right?

## Conclusion

This was pretty simple but effective in terms of giving me the option to
have more privacy. I can see it being extended to add even more
security; e.g., routing all of the traffic in the container through a
VPN would be easy enough. Food for thought \[citation needed\].

[^1]: 5mb containers sounds pretty nice to me.
    <http://gliderlabs.viewdocs.io/docker-alpine/>

[^2]: I get a segfault every time I try to start Firefox. More info
    here: <https://bugzilla.mozilla.org/show_bug.cgi?id=724227#c11>
