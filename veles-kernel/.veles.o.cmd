savedcmd_veles.o := ld.lld -m elf_x86_64 --compress-debug-sections=zlib -z noexecstack   -r -o veles.o @veles.mod  ; /usr/src/linux-6.18.12-gentoo/tools/objtool/objtool --hacks=jump_label --hacks=noinstr --hacks=skylake --ibt --mcount --mnop --orc --retpoline --rethunk --sls --static-call --uaccess --prefix=16  --link  --module veles.o

veles.o: $(wildcard /usr/src/linux-6.18.12-gentoo/tools/objtool/objtool)
