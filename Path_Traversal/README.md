# [Path Traversal](https://owasp.org/www-community/attacks/Path_Traversal)

## Writeup

The website uses a parameter `page` to specify what page to render. Let's manually set path to `../../../../../../../../../etc/passwd`, attempting to climb the directory structure and read the passwd file at `/etc/passwd`.

This causes an alert to pop up with the flag.

## Possible Patches
Limit what files can be read and rendered to the user to those that are in a specific directory. You could also blacklist route attempts that have the `../`, be careful of encoded urls. 
