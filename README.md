# archive-sum

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/9ec02e2f096f40d596cef5eb0b43a101)](https://www.codacy.com/app/wookietreiber/archive-sum?utm_source=github.com&utm_medium=referral&utm_content=idiv-biodiversity/archive-sum&utm_campaign=badger)
[![Build Status](https://travis-ci.org/idiv-biodiversity/ansible-role-repo-xcat.svg?branch=master)](https://travis-ci.org/idiv-biodiversity/archive-sum)

Generates checksums of files within an [archive file](https://en.wikipedia.org/wiki/Archive_file) without extracting its contents.

# Usage

The primary use of **archive-sum** is to **verify the integrity of archive files**, to be more specific, its contents.

In its simplest form, **archive-sum** prints the checksums of the files within an archive:

```console
$ archive-sum archive-sum-example.tar.gz
c157a79031e1c40f85931829bc5fc552  archive-sum-example/bar
258622b1688250cb619f3c9ccaefb7eb  archive-sum-example/baz
d3b07384d113edec49eaa6238ad5ff00  archive-sum-example/foo
```

If you want a different hash function (digest), specify it like this:

```console
$ archive-sum -d sha256 archive-sum-example.tar.gz
7d865e959b2466918c9863afca942d0fb89d7c9ac0c99bafc3749504ded97730  archive-sum-example/bar
bf07a7fbb825fc0aae7bf4a1177b2b31fcf8a3feeaf7092761e18c859ee52a9c  archive-sum-example/baz
b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c  archive-sum-example/foo
```

To get a more detailed help, take a look at the **archive-sum** help text:

```console
$ archive-sum --help
```

## Usage Example

Lets create an archive with some content:

```console
$ mkdir archive-sum-example
$ echo foo > archive-sum-example/foo
$ echo bar > archive-sum-example/bar
$ echo baz > archive-sum-example/baz
$ tar czf archive-sum-example.tar.gz archive-sum-example/
```

To compare the original content with the files within the archive, we first need to create the checksums of the original files:

```console
$ md5sum archive-sum-example/*
c157a79031e1c40f85931829bc5fc552  archive-sum-example/bar
258622b1688250cb619f3c9ccaefb7eb  archive-sum-example/baz
d3b07384d113edec49eaa6238ad5ff00  archive-sum-example/foo
```

Finally, we can compare this with the output of **archive-sum**:

```console
$ archive-sum archive-sum-example.tar.gz
c157a79031e1c40f85931829bc5fc552  archive-sum-example/bar
258622b1688250cb619f3c9ccaefb7eb  archive-sum-example/baz
d3b07384d113edec49eaa6238ad5ff00  archive-sum-example/foo
```

As you can see from the output, the content of the archive file is exactly the same as the original. We verified the integrity of the archive file.

## Supported Archive File Formats

[All archive formats that libarchive supports](https://github.com/libarchive/libarchive/#supported-formats). Your local libarchive installations needs to be configured to use these formats. Also, the following libarchive man page lists its supported formats:

```console
$ man 5 libarchive-formats
```

## Supported Hash Functions (Digests)

All hash functions that OpenSSL supports. You can get a list from your local OpenSSL installation:

```console
$ openssl list-message-digest-algorithms
```

# Installation

## Dependencies

- the [libarchive](http://www.libarchive.org/) multi-format archive and compression library
- the [OpenSSL](https://www.openssl.org/) cryptography library

## Installation

The installation requires the dependencies to be available and they are detected using their [pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config/) files.

```console
$ ./configure
$ make
$ make install
```
