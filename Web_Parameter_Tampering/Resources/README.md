# [Web_Parameter_Tampering](https://owasp.org/www-community/attacks/Web_Parameter_Tampering)

## Writeup
On the recover password page we have a simple form with a single `"Submit"` button. However using the browsers web tools we can see that there is a hidden input.
```html
<!-- ... -->
<form action="#" method="POST">
	<input type="hidden" name="mail" value="webmaster@borntosec.com" maxlength="15">
	<input type="submit" name="Submit" value="Submit">
</form>
<!-- ... -->
```
A hidden form input with a name `"mail"` and a set value of `"webmaster@borntosec.com"`. We ca modifty value of mail to whatever we want, alowing us to change where the recovery email would be sent.

Submiting the form with a modified adress gets us the flag.

## Potential Patches
Verifying and parsing data on the backend is a good way to make sure that your forms haven't been tampered with on the frontend. You can verify for each route where forms post to checking if "hidden" data hasn't been changed, or simply ignoring what "hidden" values are given in the request and using known/expected values. 