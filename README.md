## Adding a new dependency
With git it is possible to check out only certain branches and directories from another git repository.
1. git clone --depth=1 --no-checkout https://github.com/org/repo.git ./repo
1. cd repo && git checkout commit-ish && cd ..
1. git submodule add https://github.com/org/repo.git ./repo
1. git submodule absorbgitdirs
1. git -C repo config core.sparseCheckout true
1. `echo 'repofolder/*' >> .git/modules/repo/info/sparse-checkout`
1. git submodule update --force --checkout repo

