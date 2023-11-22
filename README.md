# 🛠️ Rustmogul

> You are a hardware store owner managing a warehouse of arcane contraptions to
> upgrade and trade to achieve market monopoly by beating local competition.
>
> Literally beating the competition.

or, more simply:

> a rustic auto battler inspired by Dota Underlords

# Development

If building complains about failing to build dependencies, try installing the following:

```bash
sudo apt install \
  build-essential portaudio19-dev libasound2-dev libpulse-dev libdbus-1-dev libudev-dev
# and you _might_ need a reboot after that

# read more: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md
```

```bash
cargo run
```

You should run `clippy` linter from time to time:

```bash
cargo clippy --all-targets --all-features --fix
```
