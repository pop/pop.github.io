+++
title = "Dynamic Attributes in Chef"

date = "2018-06-23"

description = "So you want to set a node attribute in a resource block? Good luck --oh wait you don't need luck because you have this blogpost!"

taxonomies.tags = [
    "europe 2015", "travel", "archive"
]
+++

## Problem

You are trying to set a node attribute in a resource block but it isn't working.

For example you have the following code:

```ruby
ruby_block 'foo' do
  block do
    # some ruby
    node.override['var'] = 'value'
  end
  action :run
end

if node['var'] == 'value'
  notifies :run, some_resource[name]
else
  notifies :run, other_resource[whatever]
end
```

Even though you're setting `node['var']`, `other_resource[whatever]` is getting notified to `:run`.
What gives?

## Solution

To jump ahead, because you're skimming this post for the answer, you're going to want to do something like this:

```diff
# go-go-gadget diffs
# Chef Version 13.5.8

  ruby_block 'foo' do
    block do
      # some ruby
      node.override['var'] = 'value'
    end
-   action :run
+   action :nothing
- end
+ end.run_action(:run)

if node['var'] == 'value'
  notifies :run, some_resource[name]
else
  notifies :run, other_resource[whatever]
end
```

Basically you want to execute the `ruby_block` that sets your attribute **now** so the rest of your code can consume the value either later in the compilation phase or during the execution phase of the Chef run.

## How/why did that work?

If you've read his far you're actually reading.
Welcome to the post!
Let's begin.

Let's dig into *exactly* what my problem was, because sometimes examples aren't good enough.
You know... for fun.

I was trying to trigger an application upgrade based on the current installed version.
I was *not* installing a package-manager-managed package[^1] so I couldn't just pin the version a `yum` resource, I had to do this kludge of a solution.

So for me `some_resource[name]` was downloading the latest version of the app (a `.tar.gz2` file from GitHub) and `other_resource[whatever]` was a no-op. The as part of the package download I unconditionally trigger a `:delayed` service restart.
Chef was 'upgrading' and restarting my app every thirty minutes which broke core functionality.[^2]

## Shit I tried before stumbling upon the solution

I tried a bunch of things, but they can all kinda be summed up in this:

```diff
# yay more diffs
  ruby_block 'foo' do
    block do
      # some ruby
      node.override['var'] = 'value'
    end
    notifies :create, 'tar[my-app]', :immediately # Triggers tar[my-app] to run now
  end

- if node['var'] == 'value'
-   notify :run, some_resource[name]
- else
-   notify :run, other_resource[whatever]
- end

  tar 'my-app' do
    action  :nothing # Unless triggered this does nothing
+   only_if { node['var'] == 'value' } # This should also do nothing if this isn't true
    # The rest of the resource block
  end
```

Which ultimately didn't work either.

I... I have no idea why this didn't work.

Something something Chef is complicated.

## Why the solution works (and the other stuff didn't)

The reason my ultimate solution *did* work is best summed up by this quote from the [Chef Docs](https://docs.chef.io/resource_common.html#run-action):

> Use `.run_action(:some_action)` at the end of a resource block to run
> the specified action during the compile phase.

My original code (read: broken code) was running the `if` block during the compilation phase (happens earlier) and running the `ruby_block` in the execution phase (happens later).
By telling it to run my `ruby_block` during the compilation phase we were ensuring it happened before the `if` block, and in a way running it like you would 'expect' a script to run.

It ain't idiomatic but it gets the job done.

## Errata

[^1]: Say that three times fast.

[^2]: The service was in question was Prometheus.
      When Chef ran every 30 minutes, it killed the Prometheus process and thus killed all of our pending alerts.
      TLDR we didn't get any alerts that took more than 30 minutes to trigger We also *kept* getting alerts that should taken a few hours to re-notify.
