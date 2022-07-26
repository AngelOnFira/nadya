# Nadya

This project was created for [langjam 3](https://github.com/langjam/langjam),
which had the theme "beautiful assembly".

![](https://cdn.discordapp.com/attachments/444005079410802699/1000858219721281607/ezgif.com-gif-maker1.gif)

## Running Examples

All of the examples can be found in their own folder in the `examples/`
directory.

There are several examples you can run. Nadya is build in Rust, and *should*
work by just compiling (might require stable 1.61 or above). To run an example:

`cargo run -- --example rain`

There available examples are:

- `addition`: Add 1 and 2 together
- `file`: Multiply each number in a file by 9
- `rain` Lots of falling numbers!
- `maze` Changing directions a little

![](https://cdn.discordapp.com/attachments/444005079410802699/1000862363651674172/ezgif.com-gif-maker2.gif)

> An example of basic addition. Numbers will spawn variables that move along a
> path, and wait until they can combine with something else.

### Execution Notes

- To exit the program, press `q`.
- If the code crashes, you'll likely need to close your terminal.
- Make sure your terminal is tall enough to see the full program running.

## Language Details

Nadya is a simple language that can currently add and multiply numbers together.
Variables move around the program with the `O` symbol, and will wait at
intersections until another variable gets there to "merge" with it.

This ~~game~~ language is meant to simulate items moving around some assembly
line. The inspiration for this language is factory games like Satisfactory or
Factorio. The name comes from my friend
[Aidan](https://github.com/aidancrowther) (backwards Nadia) who I taught to play
Factorio and has since greatly outskilled me.

## More Language Details

- A file `input.txt` will be loaded if there is any `F` character around the
  program. This file must contain integers on each line, and these will be the
  numbers that spawn from this place.

## Credits

Thanks to my brother for making bigger examples 30 minutes before the jam ended
~~and I had not yet started on my submission~~

Thanks to [@bigforestnotrees](https://github.com/bigforestnotrees) and
[@zesterer](https://github.com/zesterer) for ideas and sanity checks!

## Documentation

The documentation for Nadya can be found [in the wiki](https://github.com/AngelOnFira/nadya/wiki/Nadya-reference).

## License

The Summer of Rust Labs is duel-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
