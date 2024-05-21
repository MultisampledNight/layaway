# layaway

Hey, so, imagine you'd use [Sway] on a laptop and travel around a lot.
Often times when traveling, one can temporarily connect external outputs/screens
to the laptop!

But oh no, what dread is waiting upon ye?
Using only [Sway]'s mechanisms,
one would need to calculate the exact pixel position of each output oneself!

Worry not, as for this overengineered project aims to ease one's pain:
Instead of specifying *absolute* pixel positions,
one specifies *relative* logical positions,
like "the output on eDP-1 is centered below DP-3"
via a DSL.
In fact, this very situation would look like this in the DSL:

```
dp3 + edp/bottom
```

And boom, that's all what's needed for this project
to calculate and apply the output positions,
since it fetches all details it needs from [Sway].
As shell command, this suffices:

```sh
layaway 'dp3 + edp/bottom'
```

## Installation

### From the latest release

Check out https://github.com/MultisampledNight/layaway/releases/latest,
it lists the shell command to run!

### From source

1. [Install Rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)
2. Run `cargo install --git https://github.com/MultisampledNight/layaway`
3. Use `layaway` as you please!

> [!NOTE]
> This will install *the latest development version*.
> If you want a more stable one, consider using the latest release tag and
> pass it via `--tag` to the `cargo install` command, too.

> [!WARNING]
> Only Sway on Unixalikes is supported.
> Feel free to take a peek inside the [`comms`] module
> and open an issue or even PR
> if you want to change that!

## Usage details

See the [`parse::dsl`] module for a detailed description
(including an ABNF formalization!)
of how the DSL works.

You can specify the layout to use over 2 ways:
Imperatively over the command line (CLI),
  which takes precedence if both are specified,
or declaratively over the config file,
  which allows machine-specific configuration.

### Config file

Since it's rarely desirable to type the layout each time again on startup
one can define a layout to use on each machine
via a config file.
In that case, all what one needs to do for applying the layout is to run `layaway`, and
it'll look up the layout for this hostname and apply it!

The config file location is determined via [directories-next],
but essentially boils down to `~/.config/layaway/config.toml`
(XDG compliant, respecting appropriate env variables if they're set).

It's written in TOML with a single table named `[machines]`.
Each key-value pair is for one machine.
The key specifies the hostname for which to use it for,
and the value the actual layout in the aforementioned DSL to use in that case.

For example, this is the config file I'm using for two of my machines:

```toml
[machines]
destined = "dp + edp/bottom"
overloaded = "dp3 + hdmia/right,bottom"
```

This'd use `dp + edp/bottom` if the machine hostname is `destined`,
or `dp3 + hdmia/right,bottom` if the machine hostname is `overloaded`.

If the machine hostname is not in the config file,
one either needs to add it there
or specify the layout description via the CLI.

### No apply

In case you'd rather not have the layout directly applied,
but just take a look over it
or include it in your [Sway] config file,
you can also decide to pass the `--no-apply` flag to the CLI
to instead have the commands that would be ran
printed to stdout.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Feel free to open issues or PRs for whatever you'd like to see or want! However:
unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[Sway]: https://swaywm.org
[directories-next]: https://docs.rs/directories-next/2.0.0/directories_next/struct.ProjectDirs.html#method.config_dir
[`parse::dsl`]: ./src/parse/dsl.rs
[`comms`]: ./src/comms/mod.rs
