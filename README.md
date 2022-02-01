# what is this

A simple server that sends an email on a post request.

It expects a json like this:

```json
{
	"subject": "request from website",
	"email": "contact_me@domain.com",
	"body": "mail body"
}
```

## usage

NOTE: only tls smtp connections allowed

These env vars must be set (or put into a `.env` file):

```text
SMTP_USERNAME
SMTP_PASSWORD
SMTP_SERVER
SMTP_PORT
EMAIL_FROM
EMAIL_TO
LISTEN_ADDR
```

Example settings for aws ses, put these in your `.env` file or export these environment variables:

```bash
SMTP_USERNAME=....................
SMTP_PASSWORD=...................................
SMTP_SERVER=email-smtp.eu-west-1.amazonaws.com
SMTP_PORT=465 # must be a tls port
EMAIL_TO=smtp_account_email@domain.com
EMAIL_FROM=my_personal_email@domain.com
LISTEN_ADDR=0.0.0.0:3000
```

Run it (or build the portable executable):

```bash
cargo run
```

Test it:

```bash
curl -i -X POST \
       -H "Content-Type: application/json" \
       -d '{"subject":"request from website", "email": "contact_me@domain.com", "body": "mail body"}' \
       localhost:3000/api/contact_request
```
