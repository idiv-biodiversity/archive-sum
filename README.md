archive-sum
===========

[![Build Status](https://travis-ci.org/idiv-biodiversity/ansible-role-repo-xcat.svg?branch=master)](https://travis-ci.org/idiv-biodiversity/archive-sum)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/9ec02e2f096f40d596cef5eb0b43a101)](https://www.codacy.com/app/wookietreiber/archive-sum?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=idiv-biodiversity/archive-sum&amp;utm_campaign=Badge_Grade)

Generates checksums of files within an [archive file](https://en.wikipedia.org/wiki/Archive_file) without extracting its contents.

Table of Contents
-----------------

<!-- toc -->

- [Usage](#usage)
  * [Archive Verification](#archive-verification)
- [Installation](#installation)
  * [Arch Linux](#arch-linux)
  * [Manual](#manual)
- [Supported Archive File Formats](#supported-archive-file-formats)
- [Supported Hash Functions (Digests)](#supported-hash-functions-digests)

<!-- tocstop -->

Usage
-----

In its simplest form, **archive-sum** prints the checksums of the files within an archive:

```console
$ archive-sum example.tar.gz
c157a79031e1c40f85931829bc5fc552  example/bar
258622b1688250cb619f3c9ccaefb7eb  example/baz
d3b07384d113edec49eaa6238ad5ff00  example/foo
```

If you want a different hash function (digest), specify it like this:

```console
$ archive-sum -d sha256 example.tar.gz
7d865e959b2466918c9863afca942d0fb89d7c9ac0c99bafc3749504ded97730  example/bar
bf07a7fbb825fc0aae7bf4a1177b2b31fcf8a3feeaf7092761e18c859ee52a9c  example/baz
b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c  example/foo
```

To get a more detailed help, take a look at the **archive-sum** help text:

```console
$ archive-sum --help
```

### Archive Verification

The primary use of **archive-sum** is to **verify the integrity of archive files** by verifying its contents. First, so you can see and reproduce the entire workflow, lets create an archive:

```bash
mkdir example
echo foo > example/foo
echo bar > example/bar
echo baz > example/baz
tar czf example.tar.gz example/
```

We can now verify the integrity of the archive by comparing the contents of the archive with the original files:

```console
$ archive-sum -c example.tar.gz
example/foo: OK
example/baz: OK
example/bar: OK
```

As you can see from the output, the content of the archive file is exactly the same as the original. We verified the integrity of the archive file.

Installation
------------

### Arch Linux

Install the [**archive-sum** AUR package](https://aur.archlinux.org/packages/archive-sum/):

```
pacaur -S archive-sum
```

### Manual

**dependencies:**

- the [libarchive](http://www.libarchive.org/) multi-format archive and compression library
- the [OpenSSL](https://www.openssl.org/) cryptography library

**installation:**

The installation requires the dependencies to be available and they are detected using their [pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config/) files.

```console
$ ./configure
$ make
$ make install
```

Supported Archive File Formats
------------------------------

[All archive formats that your version of libarchive supports](https://github.com/libarchive/libarchive/#supported-formats). Your local libarchive installations needs to be configured to use these formats. Also, the following libarchive man page lists its supported formats:

```console
$ man 5 libarchive-formats
```

Supported Hash Functions (Digests)
----------------------------------

All hash functions that your version of OpenSSL supports. You can get a list from your local OpenSSL installation:

```console
$ openssl list-message-digest-algorithms
```
