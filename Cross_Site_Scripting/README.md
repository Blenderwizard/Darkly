# [Cross Site Scripting (XSS)](https://owasp.org/www-community/attacks/xss/)

## Writeup

On the main page, there is a link on the second NSA image that redirects to `page=media&src=nsa`. This page seemingly renders out whatever we specify in source. We can preform cross site scripting by changing what src renders.

We can do this by setting the src data to be text/html, then passing our payload in base64. Then by setting the src to this payload we can preform cross site scripting.

An example of such payload:
```bash
$> echo "<script>alert(1)</script>" | base64
PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pgo=
```

How to execute the payload:
```
http://<VM IP>/?page=media&src=data:text/html;base64,PHNjcmlwdD5hbGVydCgxKTwvc2NyaXB0Pgo=
```

Navigating to the site causes the flag to be given to us.

## Possible Patches
Sanitize input and parameters, encode problematic characters such as `< > / &`, and use a sink element instead of src, for example `element.classname`.
