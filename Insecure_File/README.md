# Insecure File

## Writeup

Using [Daniel Miessler's Security List](https://github.com/danielmiessler/SecLists.git) and [epi052's feroxbuster](https://github.com/epi052/feroxbuster), we find the `/admin` route and `robots.txt`:
```
User-agent: *
Disallow: /whatever
Disallow: /.hidden
```

In the `/whatever` folder, a single `htpasswd` file is present. Telling us that the server hosting route is an Apache Server and that the username and password to access `/admin` are found in this file.

`htpasswd`:
```
root:437394baff5aa33daa618be47b75cb49
```
It's format is simular to what is found in `/etc/passwd`, folowing the pattern of `username:hashed_password`. Using a md5 decryptor, we can find `root`'s password `qwerty123@`.

## Possible Patches

Apache warns to not expose these files, you can set this in the apache configuration file to make all ht files return forbidden error codes. 
``` apache
<FilesMatch “^/.ht”>
	Require all denied
</FilesMatch>
```