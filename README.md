# Scuttle - A CLI tool to build C4 model diagrams from your living codebase

```sh
      *******                                                 ***                        * ***         ***** *               *****  *
    *       ***                               *         *      ***                     *  ****  *   ******  *             ******  *
   *         **                              **        **       **                    *  *  ****   **   *  *             **   *  *
   **        *                               **        **       **                   *  **   **   *    *  *             *    *  *
    ***                     **   ****      ********  ********   **                  *  ***            *  *                  *  *
   ** ***           ****     **    ***  * ********  ********    **       ***       **   **           ** **                 ** **
    *** ***        * ***  *  **     ****     **        **       **      * ***      **   **           ** **                 ** **
      *** ***     *   ****   **      **      **        **       **     *   ***     **   **           ** **               **** **
        *** ***  **          **      **      **        **       **    **    ***    **   **           ** **              * *** **
          ** *** **          **      **      **        **       **    ********     **   **           ** **                 ** **
           ** ** **          **      **      **        **       **    *******       **  **           *  **            **   ** **
            * *  **          **      **      **        **       **    **             ** *      *        *            ***   *  *
  ***        *   ***     *    ******* **     **        **       **    ****    *       ***     *     ****           *  ***    *
 *  *********     *******      *****   **     **        **      *** *  *******         *******     *  *************    ******
*     *****        *****                                         ***    *****            ***      *     *********        ***
*                                                                                                 *
 **                                                                                                **
```

## What is this CLI tool trying to solve?

The C4 model for system diagrams is really great. But adoption is a little tough
if the code base is being explained after the fact. For instance, Agile
practices are interpreted very literally and there's no up-front diagrams. So
now you have a codebase with very little system diagrams. This tool is meant to
consume active codebases and retrieve the system using the C4 model to organize
and spit out very simple Structurizr DSL files which can then be maintained
after creation to bootstrap the process of mass adoption of C4 model system
diagrams and the Structurizr services such as SaaS and CLI tooling.

- 🔗 [Structurizr DSL files](https://github.com/structurizr/dsl#reference)
- 🔗 [The C4 model for visualising software architecture](https://c4model.com/)
- 🔗 [Structurizr Cloud service](https://structurizr.com/pricing)

## Contributing

This project is still in high-development. Take a look at Issues to help track
the work. They should all be labeled with `help wanted` for anything that
outside folks can contribute to. Also, there's a need for documentation so if
there's anything that needs to be updated or is confusing or missing, please
contribute directly back to the documentation.

The branches are well documented but not intuitive. Please read through the
Issue below for more discussion on this.

- 🔗 [The branches in this repository are confusing](https://github.com/rogeruiz/scuttle/issues/9)

There are three branches because of the rapid development of Scuttle in recent
months. The old legacy Scuttle is on the `master` branch which is named that way
for legacy reasons. There is also the `scuttle-dos` branch which is the initial
working sketch that uses Structurizr and PlantUML to generate diagrams. Finally
there is the `main` default branch which contains the code for the Scuttle CLI
which is more ambitious in nature than `scuttle-dos` but will begin development
implementating the `scuttle-dos` functionality.

### Development

To work with Rust locally, install the `rustup` tool chain. You can find more
information on installing it at their homepage.

- 🔗 [https://rustup.rs/](https://rustup.rs/)

Once you've installed `rustup`, clone this repository and run `cargo build`.
This will give you a `scuttle` binary at `target/debug/scuttle` which you can
use to test the CLI tool. As you edit the Rust code found in the `src/`
directory, you can continue running `cargo build` to rebuild the `scuttle`
binary.

##### Building the binary

```sh
cargo build
```

##### Running the development build

```sh
target/debug/scuttle --help
```
