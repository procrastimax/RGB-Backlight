# RGB Backlight
This tool reads all pixel of Monitor 0 and calculates the average pixel color over all pixel.
This average pixel color is then sent to the [ESP32-LED-Stripe-Server](https://github.com/procrastimax/ESP32-LED-Stripe-Server), to mimic the screen's average color resulting in an ambient room light.

## Usage
```
Usage: rgb-backlight [OPTIONS]

Options:
  -e, --endpoint <ENDPOINT>      [default: stripe.local]
  -w, --wait-delay <WAIT_DELAY>  wait time before updating RGB values in ms [default: 300]
  -u, --use-linear-rgb
  -p, --protocol <PROTOCOL>      [default: http]
  -m, --monitor <MONITOR>        [default: 0]
  -t, --threshold <THRESHOLD>    color change threshold as a sum of all channels to be exceeded in order to trigger an RGB value change [default: 25]
  -h, --help                     Print help
  -V, --version                  Print version
```

## TODO
- smooth out RGB changes which are "too intense" -> maybe use some kind of moving average?
