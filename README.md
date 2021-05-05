# comment-service

A service to count comments on social media platforms. I use it on my blog to have a dynamic comment count on my blog posts.

# Usage

The service can be accessed via REST endpoint. The url you pass should be in base64 and url encoded in the form `btoa(encodeURIComponent(url))`.

```
https://comment-service.herokuapp.com/platform/{platform}/url/{url}
```

Upon a valid request the response will contain the number of comments on a post. The response will be cached for 30 minutes.

# Local Development Setup

This service was developed using [Subo](https://github.com/suborbital/subo). Start by making sure you have this locally installed.

```bash
# Sudo permissions are occasionally required. This is a nice shorthand
alias mysudo='sudo -E env "PATH=$PATH"'

# Build the application
mysudo subo build --native

# Locally serve the application
subo dev | jq
```

# Deployment

Write and test your changes locally then deploy them with either of the following methods.

## Git Push

```bash
git push heroku master
```

## Heroku CLI

```bash
heroku container:login

heroku container:push web

heroku container:release web
```
