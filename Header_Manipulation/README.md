# Header Manipulation

## Writeup

The copyright page at the very bottom links to a ***bizarre*** page. Inspecting the source code we can see comments with the folowing instructions.

1. You must come from : "https://www.nsa.gov/"
2. Let's use this browser : "ft_bornToSec". It will help you a lot.

So lets do just thatm using curl we can set the `User-Agent` header, the header specify what browser a client is using and `Refferer Page` header specifying what site we arrived from.

``` sh
curl -A "ft_bornToSec" -e "https://www.nsa.gov/" -s "http://<REPLACE WITH YOUR IP>/\?page\=b7e44c7a40c5f80139f0a50f3650fb2bd8d00b0d24667c4c2ca32c88e13b758f" | grep "The flag is "
```

Executing this we get the flag.
