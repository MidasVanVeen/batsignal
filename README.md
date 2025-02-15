Batsignal can be used in shell-scripts to wait for certain battery conditions, or to check current battery conditions. See examples below.

## Examples

```bash
batsignal --wait --state charging && echo "Charger connected!"
```

```bash
batsignal --wait --lt 10 && echo "Battery below 10%!"
```

```bash
batsignal --lt 25 && echo "Battery below 25%!" || echo "Battery okay"
```

## Install

A simple `cargo install batsignal` should do it.
