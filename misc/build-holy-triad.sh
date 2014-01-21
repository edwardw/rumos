#! /usr/bin/env bash -x

# The script expects there're gcc and g++ 4.8 in the path.
# They can be installed by homebrew.
export CC=gcc-4.8
export CXX=g++-4.8

# Gdb doesn't support more generic target x86_64-pc-elf,
# although other two do. More importantly, NetBSD buildrump.sh
# doesn't support generic platform elf. So x86_64-pc-netbsd
# it is.
TARGET=x86_64-pc-netbsd
PREFIX="/usr/local/stow/${TARGET}"
PATH="$PREFIX/bin:$PATH"

DIR_BUILD_BINUTILS=build-binutils
DIR_BUILD_GCC=build-gcc
DIR_BUILD_GDB=build-gdb

untar_then_cd ()
{
  tarball=$1; shift
  to=$1; shift
  pkg=${tarball%.tar.bz2}

  if [ ! -d ${pkg} ]; then
    tar xjf ${tarball}
  fi
  if [ ! -d ${to} ]; then
    mkdir ${to}
  fi
  cd ${to}
}

die ()
{
  echo '>> ERROR:' >&2
  echo ">> $*" >&2
  exit 1
}

for tarball in binutils*.bz2 gcc*.bz2 gdb*.bz2; do
  case ${tarball} in
    binutils-*.tar.bz2)
      untar_then_cd ${tarball} ${DIR_BUILD_BINUTILS}
      ../${tarball%.tar.bz2}/configure --target=${TARGET} --prefix=${PREFIX} \
          --disable-nls --disable-werror
      make || die make binutils
      make install || die make install binutils
      cd ..
      ;;

    gcc-*.tar.bz2)
      untar_then_cd ${tarball} ${DIR_BUILD_GCC}
      ../${tarball%.tar.bz2}/configure --target=${TARGET} --prefix=${PREFIX} \
          --disable-nls --enable-languages=c,c++ --without-headers
      make all-gcc || die make all-gcc
      # Get ftp://ftp.netbsd.org/pub/NetBSD/NetBSD-6.0/images/NetBSD-6.0-amd64.iso
      # mount it and:
      #   mkdir -p netbsd
      #   tar xzf NETBSD_60/AMD64/BINARY/SETS/COMP.TGZ -C netbsd
      #   cp -R netbsd/usr/include/amd64 gcc/include/machine
      #   cp -R netbsd/usr/include/sys gcc/include
      #   cp netbsd/usr/include/unistd.h gcc/include/
      #   cp netbsd/usr/include/pthread_types.h gcc/include/
      #   cp netbsd/usr/include/stdlib.h gcc/include/
      #   cp netbsd/usr/lib/crtbeginS.o gcc/
      #   cp netbsd/usr/lib/crtendS.o gcc/
      #   tar xzf NETBSD_60/AMD64/BINARY/SETS/BASE.TGZ -C netbsd
      #   cp netbsd/lib/libc.so.12.181 gcc/libc.so
      # This is the ugliest hack ever.
      make all-target-libgcc || die make all-target-libgcc
      make install-gcc || die make install-gcc
      make install-target-libgcc || die make install-target-libgcc
      # GRUB always needs i386 version of the header files, so:
      #   cp -R netbsd/usr/include/i386 ${PREFIX}/lib/gcc/${TARGET}/4.8.2/include
      cd ..
      ;;

    gdb-*.tar.bz2)
      untar_then_cd ${tarball} ${DIR_BUILD_GDB}
      ../${tarball%.tar.bz2}/configure --target=${TARGET} --prefix=${PREFIX} \
          --program-prefix=${TARGET}- --disable-werror
      make || die make gdb
      make install || die make install gdb
      cd ..
      ;;
  esac
done

