+++
title = "Kubernetes as an external service proxy"

date = "2018-10-13"

description = "An adventure where we learn `Kubernetes` by doing *a very straight forward project*; documenting every success, failure, and quip along the way"

taxonomies.tags = [
    "europe 2015", "travel", "archive"
]
+++

Say you have a firewall restriction that creates the following situation:

1. App1 cannot communicate directly with App2.
2. App1 and App2 can both talk to a Kubernets cluster.
3. Neither app is hosted on the Kubernetes cluster.
4. How can you get messages between App1 and App2?

The way the problem is stated makes it pretty obvious that the solution *involves* using a Kubernetes cluster, but how exactly?

The naive solution might be to spin up a container which acts as a proxy; Nginx comes to mind.
This would definitely work, but I am exceedingly lazy and don't want to learn how to configure Nginx.
In fact, the solution I came to doesn't involve running any new pods!

Here's the code.
Below I'll explain what's happening here and why it works.

``` yaml
kind: Endpoints
metadata:
  name: myapp-proxy
subsets:
- addresses:
  - ip: 2.3.4.5k # App2's address
  ports:
  - port: 8080 # App2's service port
apiVersion: v1

---

kind: Service
metadata:
  name: myapp-proxy
spec:
  type: LoadBalancer
  loadBalancerSourceRanges:
  - "1.2.3.4/32" # App1's address
  ports:
  - protocol: TCP
    port: 80 # Redirect traffic hitting 80 to the app's service port
    targetPort: 8080
apiVersion: v1

---

kind: Ingress
metadata:
name: restaurant-proxy
spec:
rules:
  - host: myapp-proxy.somehost.net
  http:
    paths:
    - path: /
        backend:
          serviceName: myapp-proxy
          servicePort: 80
apiVersion: extensions/v1beta1
```

## Endpoints

Endpoints are the Kubernetes abstraction for IPs+Ports running the same application.
It's how you group together N instances of an app into one pool.

Under the hood Endpoints get created as a pre-requisite for every Service you deploy.
You don't usually need to deal with these directly as they are created implicitly whenever a Deployment gets applied.

By manually creating an endpoint we have imported our non-kubernetes app into Kubernetes.
That means we can do Kubernetes things with it like expose it via a Service or even put it behind an Ingress Pretty neat!

## Service

Services are how we expose an endpoint to the world.
Most cloud providers will give you a public IP address for a service and load balance across all of that service's endpoints.

This is as far as we need to proxy traffic between our two Apps.
App1 makes a request to whatever IP Kubernetes gets for the `myapp-proxy` Service and relays it to the `myapp-proxy` endpoint, which ultimately routes the traffic to App2.
What's really cool is that the endpoint that **is** App2 can be an self-hosted Virtual Machine, as long as the IP doesn't change this proxy will continue to work.

## Ingress

Ingresses are my favorite part of Kubernetes.
They're very convenient, incredibly powerful, and they work like... over half the time.

While not strictly necessary, this Ingress gives us some nice-to-haves.

- Gives us an easy to manage host name via something like [External DNS](https://github.com/kubernetes-incubator/external-dns).
- Could be extended to terminate SSL, again "for free", with something like [Cert Manager](https://github.com/jetstack/cert-manager).

So that's how we use Kubernetes to manage services (lower-case 's') which aren't running in Pods.
