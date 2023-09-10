# CONTRIBUTING

The entire code base is written for linux/MAC hosts. it is possible to use windows however I will not provide any support for setting up a windows host to develop on.

If all you have is windows and you really want to contribute you should look into running a virtualbox or using the Windows Subsystem for Linux (WSL2)

When there is a need for more structured development guidelines then this project will use an Agile methodology. For more information about agile I recommend reading
[The Wiki about Agile](https://en.wikipedia.org/wiki/Agile_software_development)
or checking out [atlassian's guide to Agile](https://www.atlassian.com/agile)

for the time being we are going to pretty relaxed with the development that should look something like the following.

- email me your creds (see below)
- wait to be added to the project
- assign yourself tasks from issues on the kaban board
- get directory locally (git clone)
- choose if you want to use author or mainline branching structure
- write code
- open PR
- wait for review
- get your changes in the codebase

### Mainline branch structure (preferred)

you can start your own branch with the following nomenclature

`<category_branch>/<author>-<issue_number>_<short_desc>`

for example:

`docs/jwilson1172-1_adding_github_templates`

if there is more details that you need to convey with a branch name...you are incorrect use the above.

*but wait what if...*

you have two branches on the same area subbranch that are also addressing the same issue ticket that both have the exact same short description? `simple don't do that, go back and merge the changes from both branches together or name the second branch with a different short description`


### Author branch structure

Similar to mainline but all branches are based from the <author> subbranch.
e.g
origin:
    jwilson1172/
        1_adding_github_templates
        2_test_issue_for_project_board
        10_changing_something

Still maintain the:
`<issue_number>_<short_desc>` format for the branch names it makes it alot easier to track

### Ok where do I sign up?

I'd like to have some registration portal for developers but I don't have that live yet so were are doing this more informal

email me at `jw59615+devcorr@gmail.com` [mailto link](mailto:jw59615+devcorr@gmail.com) with the following information:

- github username
- preferred name to be called (or None for github username)

**either**
- gpg fingerprint
- keyserver url
- signed ssh pubkey

if your gpg identity lives on a keyserver

**or**

- gpg pubkey
- signed ssh pubkey

if you are using a gpg identity that does not live on a keyserver

### how do I do that?

if you don't have gpg/ssh keys follow:\
[how to make a GPG key](https://docs.github.com/en/authentication/managing-commit-signature-verification/generating-a-new-gpg-key)\
[how to make a SSH key](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent)

to upload a key to a keyserver:\
[how to distribute GPG keys](https://www.gnupg.org/gph/en/manual/x457.html)

for exporting a gpg key use the following command:

```sh
gpg --armor --export <KEYID> > <github_username>_GPG.pub
```

where `<KEYID>` and `<github_username>` are their respective values.

**IF YOU SEND ME YOUR PRIVATE KEY I WILL GENERATE AND PUBLISH A REVOCATION CERT**

### To set the project up locally

```sh
git clone git@github.com:Jwilson1172/python_rust_for_ds.git

cd python_rust_for_ds
docker build -t my_docker_name_rust rust/Dockerfile
docker build -t my_docker_name_python python/Dockerfile
```

this will clone the repo and build the docker images for each side of this project

the python image will be about 2Gb
the rust image is negligible in size (7Mb I think)

At the moment these containers are just environments for the code to run. later I'd like to get these containers to test and build the packages.
