# [Web_Parameter_Tampering](https://owasp.org/www-community/attacks/Web_Parameter_Tampering)

## Writeup
On the route `?page=survey` we have a multiple forms where you can select a value between one and ten, causing the form to submit itself and update the votes. With the browsers dev tools we can see the form.
```html
<!-- ... -->
<form action="#" method="post">
	<input type="hidden" name="sujet" value="2">
	<select name="valeur" onchange="javascript:this.form.submit();">
		<option value="1">1</option>
		<option value="2">2</option>
		<option value="3">3</option>
		<option value="4">4</option>
		<option value="5">5</option>
		<option value="6">6</option>
		<option value="7">7</option>
		<option value="8">8</option>
		<option value="9">9</option>
		<option value="10">10</option>
	</select>
</form>
<!-- ... -->
```
We can modifty value of one of the option values to be something else than the defaults, allowing us to vote with more "effect" than we should have.

Submiting the form with a modified value gets us the flag.

## Potential Patches
Verifying and parsing data on the backend is a good way to make sure that your forms haven't been tampered with on the frontend. You can verify for each route where forms post to checking if "hidden" data hasn't been changed, or simply ignoring what "hidden" values are given in the request and using known/expected values. 