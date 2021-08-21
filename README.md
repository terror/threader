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
a Twitter API key and token pair and set them in a `.env` file within the
project root:

```.
API_KEY=
API_TOKEN=
```

Here is a file called `thread.md` which we want to tweet. We can specify an
optional thread `title` which will get displayed at the top of the first tweet.
As you can see, individual tweets are delimited via a markdown `paragraph`.

```markdown
## How to Get Rich (without getting lucky):

Seek wealth, not money or status. Wealth is having assets that earn while you sleep. Money is how we transfer time and wealth. Status is your place in the social hierarchy.

Understand that ethical wealth creation is possible. If you secretly despise wealth, it will elude you.

Ignore people playing status games. They gain status by attacking people playing wealth creation games.
```

Assuming your environment is all set up, you can invoke the following command
to convert the following file into a Twitter thread and then have it tweeted on
your behalf:
```bash
threader --file thread.md
```

### Prior Art

[`threader`](https://github.com/choldgraf/threader) - Easy Twitter threads
with Python
