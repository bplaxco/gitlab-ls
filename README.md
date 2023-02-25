# gitlab-ls

List the repos under a GitLab namespace and its descendant groups if present

This was just thrown together to:

1. Try to solve a problem I have with GitLab
1. Play with rust a bit more

This needs a bit of love (See TO-DOs)

## Docs 

```
NAME

    gitlab-ls - List the repos under a GitLab namespace and its descendant groups if present

USAGE


    gitlab-ls <gitlab-server> <namespace-id>


OPTIONS

    gitlab-server - The domain of the server to check against

    namespace-id - The namespace to search 

AUTH

    Requires GITLAB_TOKEN env var to be set.

EXAMPLES

    gitlab-ls gitlab.com bplaxco
```

## TO-DOs

1. Get it to properly paginate projects in groups
1. Clean up the code and make it actually nice
1. Add unittests and better docs
