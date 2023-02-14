# [Brute Force](https://owasp.org/www-community/attacks/Brute_force_attack)

## Writeup
On the signin page, we are prompted for a username and a password. We can assume that the admin account has a username like `admin` or `administrator`. Using a program like the one in the `Resources` folder, we can enumerate through a list of common passwords untill we get a response that indicates we have logged in.

*List of common password curtosy of [Daniel Miessler's Security List](https://github.com/danielmiessler/SecLists.git)*

Brute forcing the password, we find that `shadow` is the administrator's password. Logging in gets us the flag.

## Possible Patches

Using stronger passwords not found in common password lists can prevent brute force attacks, limiting login attempts or introducing a recaptcha can also help alot in limiting automated attacks.