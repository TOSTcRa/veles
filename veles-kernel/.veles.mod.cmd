savedcmd_veles.mod := printf '%s\n'   veles.o | awk '!x[$$0]++ { print("./"$$0) }' > veles.mod
