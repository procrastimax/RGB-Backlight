# RGB Backlight
This tool reads all pixel of Monitor 0 and calculates the average pixel color over all pixel.
This average pixel color is then sent to the [ESP32-LED-Stripe-Server](https://github.com/procrastimax/ESP32-LED-Stripe-Server), to mimic the screen's average color resulting in an ambient room light.

## Usage
```
Usage: rgb-backlight [OPTIONS]

Options:
  -e, --endpoint <ENDPOINT>      [default: stripe.local]
  -w, --wait-delay <WAIT_DELAY>  [default: 500]
  -u, --use-linear-rgb
  -p, --protocol <PROTOCOL>      [default: http]
  -m, --monitor <MONITOR>        [default: 0]
  -h, --help                     Print help
  -V, --version                  Print version
```

## TODO
- do not send setRGBA request, if screen barely changed
- smooth out RGB changes which are "too intense" -> maybe use some kind of moving average?
