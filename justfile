commit_file := "commit"

build:
    cargo build

release:
    cargo build --release

commit:
    git diff --quiet && git diff --staged --quiet || git commit -F {{commit_file}}

amend:
    git diff --quiet && git diff --staged --quiet || git commit --amend -F {{commit_file}}

pull: commit
    git pull --rebase origin master

push: pull
    git push -F origin HEAD
