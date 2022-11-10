# [Form Action Hijacking](https://owasp.org/www-community/attacks/Form_action_hijacking)
###### *Could also be considered Web Parameter Tampering, I guess?*

## Writeup

Navigating to route `/index.php?page=feedback`, we see a forum where users can post a comment, and we can see the comments bellow the form. Attempting to validate the form with one or both of the fields missing causes a pop-up to be displayed and the form to be discarded.


This validation happens in the front end of the site in a code block:
```html
<script>
function validate_required(field,alerttxt) {
	with (field) {
		if (value==null||value=="") {
			alert(alerttxt);return false;
		}
		else {
			return true;
		}
	}
}
function validate_form(thisform) {
	with (thisform) {
		if (validate_required(txtName,"Name can not be empty.") == false)
		{txtName.focus();return false;}
		if (validate_required(mtxMessage,"Message can not be empty.") == false)
		{mtxMessage.focus();return false;}
	}
}
</script>
```

And the form calls this function when the submit button is clicked:
``` html
<form method="post" name="guestform" onsubmit="return validate_form(this)">
<!-- ... -->
</form>
```

If we simpily remove the contents of onsubmit attribute in the form, we can bypass all check for if a form field is empty. Doing so gets us the flag.

## Possible Patches

Verify your contents of a submitted form in the backend by checking the data that was submited. Disregard the form if critical data is missing or incorrect.