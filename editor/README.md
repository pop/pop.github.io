# editor.elijah.run

This is the code for an editor to my blog, [elijah.run](https://elijah.run).
You can check it out now by going to https://editor.elijah.run and viewing the content in read-only mode.

Uniquely my blog is a static site, meaning it's just a pile of `.md` files that get compiled into `.html` files.
Most editors of this sort are associated with dynamic sites that have databases; think Wordpress or Blogger.
So how does it work?

## How does it work?

Basically: the Github API and Rust+WASM.

The majority of the app is a Rust (compiled to WASM) HTML app using the [yew](https://yew.rs/) framework.

* That front-end queries the Github API directly to list files, get file contents, all that jazz.
* When authenticated it uses the API to create an edit branch, post updates to that branch, and even check Github Actions to make sure the branch builds so it can be safely merged.

There is a very small function running in Cloudflare Workers that handles Github OAuth.

## LLM Disclosure

> This project was developed in February 2026 using Claude Code with Opus 4.6 and Sonnet 4.6.

This is a project I've _wanted_ for many years, but never got around to learning the many tools, libraries, frameworks, protocols I would need to grok in order to build it.

I built this with Claude Code over about a week and continue to make updates as I find bugs and think of features I'd like to use.

This is an experiment with whatever we are calling this -- "vibe coding" or "on-demand software" -- I'm still figuring out how I feel about the whole thing.

This README though... was written by me.

## Contributing

Contributions are welcome from humans and LLM-assisted humans alike.
That said I am building this for my personal use, so features may be rejected for being out of scope.

You might find [PLANNING.md](PLANNING.md) and [CLAUDE.md](CLAUDE.md) useful resources even if you are not doing LLM-assisted development.

## LICENSE

This software is MIT licensed. See [LICENSE](LICENSE) for the full text.
