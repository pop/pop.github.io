Git Does a Lot of Things
========================

:date: 2016-07-27
:slug: git-does-a-lot-of-things
:status: published
:summary: The title is really an understatement...
:tags: Git, Fun Facts

I should make a tool that extends ``git`` to delete all of the files matched
by the ``.gitginore``.  I spend way too much time crafting ``find <...> |
xargs rm`` commands.  I know, I'll call it ``git clean``.

Well... before I get too far I'll just make sure it doesn't already exist.

.. code-block:: text

    $ man git clean

    GIT-CLEAN(1)                  Git Manual                       GIT-CLEAN(1)

    NAME
           git-clean - Remove untracked files from the working tree

    SYNOPSIS
        git clean [-d] [-f] [-i] [-n] [-q] [-e <pattern>] [-x | -X] [--]
        <path>...

    DESCRIPTION
        Cleans the working tree by recursively removing files that are not
        under version control, starting from the current directory.

        ...

Huh... well I... that's pretty much exactly how I would have done it...

``git`` *tab tab*

.. code-block:: text

    add                  filter-branch        relink 
    am                   format-patch         remote 
    annotate             fsck                 repack 
    apply                gc                   replace 
    archive              get-tar-commit-id    request-pull 
    bisect               grep                 reset 
    blame                help                 revert 
    branch               imap-send            review 
    bundle               init                 rm 
    checkout             instaweb             send-email 
    cherry               interpret-trailers   shortlog 
    cherry-pick          log                  show 
    clean                merge                show-branch 
    clone                mergetool            stage 
    column               mv                   stash 
    commit               name-rev             status 
    config               notes                submodule 
    credential           p4                   svn 
    describe             pull                 tag 
    diff                 push                 verify-commit 
    difftool             rebase               whatchanged 
    fetch                reflog               worktree

Ah, okay. I get it now. Git *does* do a lot of things.

.. note:: The above output is from `git 2.8.2`.
