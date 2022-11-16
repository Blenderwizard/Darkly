# [Open Redirect](https://cheatsheetseries.owasp.org/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.html)

## Writeup

At the bottom of the page there are 3 links that redirect the user to either Facebook, Twitter and Instagram:
``` html
<ul class="icons">
	<li><a href="index.php?page=redirect&amp;site=facebook" class="icon fa-facebook"></a></li>
	<li><a href="index.php?page=redirect&amp;site=twitter" class="icon fa-twitter"></a></li>
	<li><a href="index.php?page=redirect&amp;site=instagram" class="icon fa-instagram"></a></li>
</ul>
```

Replace the `site` parameter with another site grants us the flag.

## Possible Patches
Using client specified redirects can be dangerous, since bad actors could abuse this system to redrict to dangerous sites to leak information from connected users such as tokens, or preform phishing attacks on users.

Don't use paramters that can be modified by users, instead use static routes that take zero user input to do redirects.