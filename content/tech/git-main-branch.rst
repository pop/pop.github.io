Nuggit: Manually change your default branch name
================================================

:date: 2021-07-07
:slug: git-main-branch
:status: published
:summary: Really the story of not upgrading for far too long...
:tags: Git, Problematic Language

So I was on my laptop running an ancient release of Linux called Fedora 30 -- so old COVID wasn't even a twinkle in a bat's eye when it was released.

On this ancient OS I'm have ``git 2.21.3`` which is like 13 versions behind.  I ask yum -- err *dnf* -- if there is an upgrade and it's like "nope". Nothing makes me feel older than software.

I've gotten into the habit of naming my default branches ``main`` instead of ``master`` because every time I say "master" I need to take a shower. I'm pretty sure the latest git names the default branch ``main`` out of the box, but git from April 2019 was not aware of problematic language choices.

So as one does I'm starting a new repo and I decide before updating ``git``, which is gonna require like 3 Fedora upgrades, I'm just going to rename the branch. I know I'll get woke-git if I upgrade Fedora, but I want to write this post first so we're gonna find a workaround.

.. code-block:: text

	[branch:master]$ git branch -m main
	error: refname refs/heads/master not found
	fatal: Branch rename failed

Hmm. So on a bare repo you can't rename the starting branch because there's no objects in the repo... or something like that *waves hands*.

Umm... I guess we'll go spelunking into the `.git` directory to see if we can manually force our `main` branch naming.

.. code-block:: text

	[branch:master]$ cd .git
	[.git]$ tree -F
	.
	├── branches/
	├── config
	├── description
	├── HEAD
	├── hooks/
	│   ├── A bunch of sample scripts
	├── info/
	│   └── exclude
	├── objects/
	│   ├── info/
	│   └── pack/
	└── refs/
		├── heads/
		└── tags/

Well unsurprisingly we have a bunch of empty directories and some sample scripts.

The only files that _might_ be worth looking at are `config`, `description`, and `HEAD`.

.. code-block:: text

	// config
	[core]
		repositoryformatversion = 0
		filemode = true
		bare = false
		logallrefupdates = true

This seems to be config options for this repo, none of which mention branch naming so it's a skip for me.

.. code-block:: text

	// description
	Unnamed repository; edit this file 'description' to name the repository.

I had no idea this feature existed. I have never seen it be used -- but fun facts!

.. code-block:: text

	// HEAD
	ref: refs/heads/master

And the money shot. Let's change that bad boy to `refs/heads/main` and see if my magical git prompt picks it up:

.. code-block:: text

	[.git]$ cd ../
	[branch:main]$ 

Heyo! There you have it. How to manually change the name of your main branch in a fresh git repo.

Sanity check, this won't fuck up if we commit right?

.. code-block:: text

	$ cargo init .
	$ git add . && git commit -m "initial commit"
	[main (root-commit) ebecbee] initial commit
	 3 files changed, 12 insertions(+)
	 create mode 100644 .gitignore
	 create mode 100644 Cargo.toml
	 create mode 100644 src/main.rs

Note...
-------

After anxiously upgrading to Fedora 34 I confirmed that git fixed a lot of this:

.. code-block:: text

	[pop@lappy foo]$ git init
	hint: Using 'master' as the name for the initial branch. This default branch name
	hint: is subject to change. To configure the initial branch name to use in all
	hint: of your new repositories, which will suppress this warning, call:
	hint:
	hint: 	git config --global init.defaultBranch <name>
	hint:
	hint: Names commonly chosen instead of 'master' are 'main', 'trunk' and
	hint: 'development'. The just-created branch can be renamed via this command:
	hint:
	hint: 	git branch -m <name>

And yes, ``git branch -m <name>`` does work on an empty repo.
