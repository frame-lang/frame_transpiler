
# Frame Language Transpiler

This project contains the code for building the Frame Language Transpiler - the **Framepiler**.  The Framepiler is written in Rust and transpiles Frame specification documents into Python (with more languages to come) as well as UML Statechart diagrams.

## Explore Frame

To learn more about the Frame language, please find Frame's official documentation on [Read the Docs](https://docs.frame-lang.org). 

## Tools and Resources

The Frame project is still early days but there are some resources and communities to help. You can download the [VSCode](https://marketplace.visualstudio.com/items?itemName=frame-lang-org.frame-machine-maker) extension to develop 
Frame programs on your desktop or experiment with Frame online at the [Frame Playground](https://playground.frame-lang.org). 

You can also learn more about programming with automata at Reddit ![re](https://www.google.com/s2/favicons?domain_url=https://reddit.com) on the [r/statemachines](https://www.reddit.com/r/statemachines/) subreddit (I'm the mod).

## Frame Community

Connect with me and other Frame enthusists on the Frame **Discord channel** -  [The Art of the State](https://discord.com/invite/CfbU4QCbSD). You can also connect with me directly on [LinkedIn](https://www.linkedin.com/in/marktruluck/).

## Frame Examples

The [Frame Solution Depot](https://github.com/frame-lang/frame_solution_depot) is a Github repo and contains a growing body of examples and test specifications. 

## Reporting Bugs and Problems 

For now send issues to <bugs@frame-lang.org> while we get a better system in place. If you have a recommendation for a free bug tracker for open source communities please let me know!


## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Installing


#### MacOS

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Navigate to the framepiler/framec directory.
3. Type `cargo build`.
	3.a You will see a bunch of warnings. Apologies but this is pre-alpha code.
4. Type `./target/debug/framec ../examples/HelloWorld.frm python_3`.
	4.a You should see a base class for a Frame controller generated to stdout.
5. If you want to generate a release build:
	5.a Type `cargo build --release`
	5.b Type `./target/release/framec ../examples/HelloWorld.frm python_3`
6. You now have working debug and release Framepilers. Congratulations!
7. You can try 6 other languages + Plant UML. Replace the `python_3` above with any of these:
	7.a `python_3`
	7.b `plantuml` (try output at [PlantUml site](http://www.plantuml.com/))

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

**Mark Truluck** - *Creator of Frame* - [LinkedIn](https://www.linkedin.com/in/marktruluck/)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* [Alan Turing](https://en.wikipedia.org/wiki/Alan_Turing) - For inventing automata theory and helping end WWII. See [The Imitation Game](https://www.imdb.com/title/tt2084970/)
* [Dr. David Harel](http://www.wisdom.weizmann.ac.il/~harel/papers.html) - Who invented [Statecharts](https://www.sciencedirect.com/science/article/pii/0167642387900359) from which came Frame.

