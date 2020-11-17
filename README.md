# clean-my-files

_A little tool to clean up after CleanMyMac X._

> For me, [CleanMyMac](https://macpaw.com/cleanmymac) seems to always run into
> various errors (usually permissions-related). So, I build a tool to clean up
> after CleanMyMac. 🤷‍♂️

## Usage

> Ensure you have
> [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
> (the dependency manager for [Rust](https://www.rust-lang.org)) installed.

```zsh
# Install clean-my-files:
cargo install --git https://github.com/stevenxie/clean-my-files

# Pass it the error report from CleanMyMac:
sudo clean-my-files < error-report.txt
```
