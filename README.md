
# Frame Language Transpiler v0.7.3

[![Join the chat at https://gitter.im/frame-language/python-discussion](https://badges.gitter.im/frame-language/python-discussion.svg)](https://gitter.im/frame-language/python-discussion?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)


Hi! So very glad you are interested in Frame. Frame system design markdown language for software architects and engineers. It is an easy to learn textual language for defining system specifications that can generate both UML documentation as well as code in 7 langauges. 


## Purpose

This project contains the code for building the Frame Language Transpiler - the Framepiler.  The Framepiler is written in Rust and transpiles Frame specification documents into 6 programming languages (more to come) and UML documentation.

## New in Frame v0.7.X

Frame now has a full-featured runtime interface in Rust!

For details, see the [Release Notes](https://github.com/frame-lang/frame_transpiler/releases).


## Resources

The Frame project is just getting started but there are some resources and communities to help. You can now download [VSCode](https://marketplace.visualstudio.com/items?itemName=frame-lang-org.frame-machine-maker) and [Atom](https://atom.io/packages/frame-machine-maker) extensions to work with Frame in these popular free development applications.

An [online version of the Framepiler](https://framepiler.frame-lang.org) is also available and provides examples and links to other resources. You can learn more about the Frame language at [frame-lang.org](https://frame-lang.org) as well as find general resources about programming with automata at Reddit ![re](https://www.google.com/s2/favicons?domain_url=https://reddit.com) on the [r/statemachines](https://www.reddit.com/r/statemachines/) subreddit.

Communities exist at [Gitter](https://gitter.im/frame-language/community) and [Discord](https://discord.com/invite/CfbU4QCbSD).

### Frame Examples
The [Framepiler](https://framepiler.frame-lang.org/example/aHR0cHM6Ly9naXN0LmdpdGh1Yi5jb20vZnJhbWUtbGFuZy8wZGFmMDMzOGU0YTkyYjc1NWViMTQ2NGM3YzVjMTM3Zg==) itself has a number of examples baked into it but I also have started a [Gitter Frame Examples](https://gitter.im/frame-language/frame-examples) channel for contributions. The Framepiler supports links to Gists so please create and share!

The [Frame Solution Depot](https://github.com/frame-lang/frame_solution_depot) is a Github repo and growing body of examples and test specifications. This is useful in conjunction with the [VSCode](https://marketplace.visualstudio.com/items?itemName=frame-lang-org.frame-machine-maker) and [Atom](https://atom.io/packages/frame-machine-maker) extensions.

## Bugs and Problems

For now please report issues to the [Gitter Bug Channel](https://gitter.im/frame-language/bug-reports) while we get a better system in place. If you have a recommendation please let me know there!

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.


### Installing


#### MacOS
1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Navigate to the framepiler/framec directory.
3. Type `cargo build`.
	3.a You will see a bunch of warnings. Apologies but this is pre-alpha code.
4. Type `./target/debug/framec ../examples/HelloWorld.frm c_sharp`.
	4.a You should see a base class for a Frame controller generated to stdout.
5. If you want to generate a release build:
	5.a Type `cargo build --release`
	5.b Type `./target/release/framec ../examples/HelloWorld.frm c_sharp`
6. You now have working debug and release Framepilers. Congratulations!
7. You can try 6 other languages + Plant UML. Replace the `c_sharp` above with any of these:
	7.a `javascript`
	7.b `cpp`
	7.c `gdscript`
	7.d `java_8`
	7.e `plantuml` (try output at [PlantUml site](http://www.plantuml.com/))
	7.f `python_3`
	7.g `rust` (experimental - only partially implemented)

#### Linux

1. Install  [Rust](https://www.rust-lang.org/tools/install).
2. Probably the same as MacOS but guessing you can figure it out if you know Linux and Rust. Still - please send me instructions on [Gitter Bug Channel](https://gitter.im/frame-language/bug-reports)  and I will add to next release notes. Thanks!

#### Windows
1. Install  [Rust](https://www.rust-lang.org/tools/install).
2. Help needed. Please send me instructions on [Gitter Bug Channel](https://gitter.im/frame-language/bug-reports)  and I will add to next release notes. Thanks!


## Built With

* [Rust](https://www.rust-lang.org/) - Rust language

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/frame-lang/064097505d77b7ecb7f49a30f75622c4) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/frame-lang/frame_transpiler/tags).

## Author

* **Mark Truluck** - *Creator of Frame* - [LinkedIn](https://www.linkedin.com/in/marktruluck/)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* [Alan Turning](https://en.wikipedia.org/wiki/Alan_Turing) - For inventing automata theory and helping end WWII. See [The Imitation Game](https://www.imdb.com/title/tt2084970/)
* [Dr. David Harel](http://www.wisdom.weizmann.ac.il/~harel/papers.html) - Who invented [Statecharts](https://www.sciencedirect.com/science/article/pii/0167642387900359) from which came Frame.
