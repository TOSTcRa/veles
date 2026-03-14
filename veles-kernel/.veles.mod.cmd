savedcmd_veles.mod := printf '%s\n'   module.o helpers.o | awk '!x[$$0]++ { print("./"$$0) }' > veles.mod
