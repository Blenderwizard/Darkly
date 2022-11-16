# [Cross Site Scripting (XSS)](https://owasp.org/www-community/attacks/xss/)

## Writeup

On the feedback page we have access to a form that allows a user to leave a comment. Inputing the word script in either the name field or the message field gets us the flag.

> **Warning**
>
> `<script>...</script>` does not work, simply leaving the form blank, in my opinion this should also give us the flag.
## Possible Patches
Sanitize input and parameters, and encode problematic characters such as `< > / &`.