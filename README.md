<h1 align="center">streamsum</h1>

streamsum reads data from stdin and prints out a checksum.

The input data is divided into chunks.  Each chunk corresponds to a single
character in the output.  This means:

* longer files produce longer checksums
* if two files are the same up to a certain point, their checksums will share a
  common prefix
* you can see the start of the checksum without waiting for the whole file to be
  processed (streamsum prints each character as it becomes known)

The chunk size increases exponentially as more data is processed.  This means
that it produces reasonably-sized checksums for a wide range of input sizes.


