export interface db {
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