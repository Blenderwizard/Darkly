# [Brute Force Attack](https://owasp.org/www-community/attacks/Brute_force_attack)

## Writeup

Using [Daniel Miessler's Security List](https://github.com/danielmiessler/SecLists.git) and [epi052's feroxbuster](https://github.com/epi052/feroxbuster), we can find `robots.txt`:
```
User-agent: *
Disallow: /whatever
Disallow: /.hidden
```

In the `/.hidden` folder, we find a autoindex of about 20 directories and a README file. The README file is just a taunt, and the directories each lead around 20 more directories and another README file with a taunt. This continues 2 more times.

One of the READMEs probably has the flag within it, we can deploy a Brute Force Attack to help us find the flag, an example in rust is provided.

## Possible Patches

Turning off autoindex would increase the search time and render the use of a creative Brute Force Attack (since we would also have to brute force directory names), but the best way to make sure that sensitive data is not reached by never exposing it on routes or locking it behind sessions.