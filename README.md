# search-rg

For this week's project we'll be fast protoyping and building another rust implementation for grep based off of ripgrep. For those unfamiliar, grep (or Global Regular Expression Search and Print) is a GNU command line tool for searching for text across entire file systems and the basis for most matching searches everywhere.

Our motivation to understand how fast multi-file text-searches—like [Github's phenomonal code search](https://web.archive.org/web/20230606164719/https://github.blog/2023-02-06-the-technology-behind-githubs-new-code-search/) and the eponoymous [grep](https://en.wikipedia.org/wiki/Grep)—work is an exercise in building large, scalable, and performant systems. In our attempt to recreate the most command line search and regex tool on Unix machines today—ripgrep—we hope to learn a bit more about these systems and rust benchmarks.

Some precursor reading*:
- [Grep in 60 seconds](https://youtube.com/shorts/5_t_I_4OuwQ?feature=share)
- [Anatomy of a Grep](https://blog.burntsushi.net/ripgrep/#anatomy-of-a-grep) by the ripgrep developer
- [ripgrep core source code](https://github.com/BurntSushi/ripgrep/tree/master/crates/core)
- Read all the documentation for [Silver Searcher](https://geoff.greer.fm/2011/12/27/the-silver-searcher-better-than-ack/), an  ack clone which is itself grep for code search, selectively ignoring files and optimizing for searched input. It is very good
- Finally a [ripgrep issue on indexing](https://github.com/BurntSushi/ripgrep/issues/95) which links very nicely to an article explaining how trigrams worked in the original Google search engine, and shed light on the how and tradeoffs for making ripgrep even faster.

*In priming our understanding of an implementation, we will also learn about the heritage and history of good CL search.

# getting-started

# 