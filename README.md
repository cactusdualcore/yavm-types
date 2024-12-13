# YAVM Types

Dynamic typing information for YAVM.

## Goals

1. **Cheap to compare**
   Operations should be typesafe at runtime (i.e. integer addition requires
   integer operands), but these checks come at a cost. Keep them to a minimum.
2. **Cheap to clone**
   Programming is about functions. The virtual machine is built around
   pass-by-value, so cloning values must be cheap.
3. **Fast**
   Data should be fast to read, write and manipulate.
4. **Flexible**
   The runtime should have good support for rich data type
