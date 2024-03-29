# [Session Prediction](https://owasp.org/www-community/attacks/Session_Prediction)

## Writeup

Using developer tools we can see that the site has a single cookie, `admin` which is set to what suspiciosly looks like an MD5 hash (`68934a3e9455fa72420237eb05902327`).

> **Note**
>
> A MD5 hash follow the Regex expresion `/^[a-fA-F0-9]{32}$/i`, the value of our cookie matches this expression.

Decrypting the hash with a tool like [dcode](https://dcode.fr/md5-hash) we can see that `false` was the plaintext. Using [dcode](https://dcode.fr/md5-hash) again, we can encode the plaintext `true` to the md5 hash (`b326b5062b2f0e69046810717534cb09`). Using the dev-tools we can set our admin cookie to this value and simply navigate to any page, stealing the admin session and the flag.

## Potential Patch

Using a more secure token strategy for sessions like Json Web Tokens would secure the site much more than simply using MD5. Part of the JWT's is signed by the server, making very hard for users to change the token without invalidating it. Not to mention but the admin permisions should be stored in a database, not in the form of a token or cookie.
