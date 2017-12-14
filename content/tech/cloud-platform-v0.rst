Building a Cloud Platform: v0
=============================

:date: 2017-07-12
:slug: cloud-platform-v0
:status: draft
:summary: Let's put this shit through the wash.
:tags: CoreOS, Kubernetes, Terraform, Cloud, Adventure

If you go to a tech-ey container centric conference you'll probably hear something about how ephemeral Kubernetes (K8s) is.
It can scale up, scale down, self heal, and *in theory* recover from losing entire nodes if you've got enough redundancy.
I've even hosted talks at the OSU LUG where these exact things were claimed and prove in a *live demo*.

That's fine and dandy, but how hard is it to get up an running with a Kubernetes cluster that has all of those features?
Specifically *these* features:

- A [secure]_ cloud-hosted K8s cluster.
- The cluster has [fault-tolerence]_.
- The cluster uses distributed storage.
- The cluster can scale the number of nodes it runs on.
- The cluster is modular, so I can swap out the old shiny for the new shiny.
- I shouldn't need to ssh into a server *ever*.
- As close to 1 click deployment/upgrade/tear-down as possible.
- Backups would be nice.

TLDR: my goal is to get a K8s cluster up and running which is as awesome as everybody says it *can* be.


How...?
-------

*Thanks for asking kid.*

Here's the tools I'm expecting to use:

- `Terraform`_ is a configuration management tool which I like because it spins up resources on my behalf and enforces [statelessness]_ in an intuitive (and very 'cloud native') way.

- `Container Linux`_ is an operating system which I am pretty comfortable. It also includes some awesome things like `ignition`_ which I can use to provision things like etcd and my container runtime automagically.

- **Anything bug AWS** for my hosting. I get that it's popular, but I have this thing where I'm not a fan of monopolies and AWS really [smells]_ like a monopoly , or at least it's trying to be. So instead I'm going to use `DigitalOcean`_ and other `Federated`_ cloud providers to see how provider-agnostic I can get.

These will be used to provision arbitrary numbers of hosts on whatever cloud provider(s) I choose.
In theory I can scale the number of clusters up, down, left, and right with a few changes to the Terraform config files... in theory.

- `Rook`_ is a K8s `controller`_ for creating an abstraction for storage. TLDR you throw some block devices at it and it gives you a pool of storage bits to use for whatever you want. Plus you get storage redundancy for free!

- `Ingress`_ is a K8s resource for managing which urls and endpoints go to which K8s services; it's usually backed by Nginx. Ingress mixed with some `other components`_ and I can get Letsencrypt SSL termination on all of my public network traffic with very little manual configuration of new services.

These should give me a robust-enough K8s platform which I can throw services like `Nextcloud`_, `Emby`_ and this website onto.

There's definitely a lot of holes in this plan, but it's at least a starting place.
I'm not sure the best way to handle backups yet; I better figure that out quick...


Bored. Not enough gifs.
-----------------------

*So pushy...*

In the next part I'll actually try implementing this idea, document the pitfalls, and hopefully get something up and running.
Expect another *loooooong* post.


Errata
------

.. [secure]
    What?

    Good question.

    By this I mean each node in the cluster communicates with every other node over a secure (encrypted) channel.
    Communication for things like etcd, administration, etc. are all encrypted.

.. [fault-tolerence]
    I am arbitrarily deciding that this means we can "lose" (read: kill on purpose in a demo) 1/3 of our nodes and maintain all services.
    Plus or minus a few network hiccups.

.. [statelessness]
    I *love* statelessness. Can't get enough of it.

.. [smells]
    Yes I am aware this might cost me a job at Amazon.

    No I do not care.


.. _Terraform:
.. _Container Linux:
.. _ignition:
.. _Rook:
.. _Ingress:
.. _controller:
.. _other components: https://github.com/jetstack/kube-lego
.. _DigitalOcean:
.. _Federated:
.. _Nextcloud:
.. _Emby:
