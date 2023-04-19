# Set of cmd line utilities written in rust

## Find

`find <folder> [--name <pattern>] [--case-insensitive]`

* find all files with doc on name (case sensitive)
```bash
$ find / --name doc 
```

* find all files with doc on name (case insensitive)
```bash
$ find / --name doc -i
```

## Sha256 Hash Sum

`sha256sum FILEs [-c]`

* calculate the hash of a give file(s)
```bash
$ sha256sum /usr/bin/true /usr/bin/false > true-false.sha256
```

* check if the given hash file matches files
```bash
$ sha256sum -c true-false.sha256
```

