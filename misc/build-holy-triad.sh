#! /usr/bin/env bash -x

# The script expects there're gcc and g++ 4.8 in the path.
# They can be installed by homebrew.
export CC=gcc-4.8
export CXX=g++-4.8
# Building gcc needs some header files from NetBSD
#NETBSD_SRC_DIR=$(pwd)/netbsd_src

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

#if [ ! -d ${NETBSD_SRC_DIR} ]; then
#  mkdir ${NETBSD_SRC_DIR}
#  cd ${NETBSD_SRC_DIR}
#  CVSROOT="anoncvs@anoncvs.NetBSD.org:/cvsroot" CVS_RSH="ssh" \
#      cvs checkout -r netbsd-6 -P src/sys || die checkout NetBSD src failed
#  ln -s src/sys/arch/amd64/include machine
#  cd ..
#fi

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
      make || die make gcc
      make install || die make gcc
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

