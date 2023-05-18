# CCC


# About

When communicating with embedded systems we typically don't have higher-level APIs like HTTP or Websockets.  The two options that are often available are serial communication or perhaps TCP. However, when sending data as a stream of bytes we have two problems to solve.

* Framing -  Given a stream of bytes how do we know where one packet ends and another starts?
* Data Validation - How do we verify the data we got wasn't corrupted?

COBS solves the framing problem by:

* Takes the raw data and examines it for occurrences of the special framing character. Let's assume this character is a zero.
* It modifies the data to ensure that no zeros appear within the data itself.
* Then, it adds back a zero at the end of the data to mark the end of the packet.

The beauty of COBS is that the transformation is reversible and the original data can be reconstructed perfectly, without ambiguity.
We can use a CRC checksum to verify that the received packet isn't corrupted.
How we encode the data is arbitrary, if Serde supports the format in a no_std env then we're all good.
Currently, only JSON is implemented but I'll probably add support for CBOR.