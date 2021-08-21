## threader

`threader` is a command-line application that lets you create twitter threads
using a plain old markdown file.

I usually write blog posts in Markdown, sometimes these posts are short enough
so that they merit being posted on Twitter as a thread rather than on my
website.

### Installation

(For now) You can clone the project and install the binary using by invoking the
following command in the project root:

```bash
$ cargo install --path .
```

### Usage

In order to use this, you must sign up for a Twitter developer account and grab
the following information:

- Consumer key and secret pair
- Access token key and secret pair

Once you have the information above set them in a `.env` file within the
project root:

```.
CONSUMER_KEY=
CONSUMER_SECRET=
ACCESS_TOKEN_KEY=
ACCESS_TOKEN_SECRET=
```

Here is a file called `thread.md` which we want to tweet. We can specify an
optional thread `title` which will get displayed at the top of the first tweet.
As you can see, individual tweets are delimited via a markdown `paragraph`.

```markdown
Threader is a command-line tool for crafting twitter threads

Write stuff down in a Markdown file and invoke a single command to publish your thread.

This could be useful if you write an article using Markdown and want to post it to your blog and Twitter at the same time.

Check out the code on GitHub: https://github.com/terror/threader
```

Assuming your environment is all set up, you can invoke the following command
to convert the following file into a Twitter thread and then have it tweeted on
your behalf:

```bash
threader --file thread.md
```

And then [view it on Twitter](https://twitter.com/436/status/1429214620510822409)!

### Prior Art

[`threader`](https://github.com/choldgraf/threader) - Easy Twitter threads
with Python
