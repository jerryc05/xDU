# xDU
A naive and cross-platform implementation of Disk Usage.

## Example
```
NAME
	xdu -- Display Disk Usage - eXperimental version by @jerryc05

SYNOPSIS
	xdu [-d depth] directory [dir2 dir3...]

DESCRIPTION
	This xdu tool is a simplified version of *ix's `du` command. Fore more info, please refer to `man du`.

	The options are as follows:

	-d depth
			Print only `depth` level of directory. Maximum value for `depth` is 2^32 on 64/32bit OS, and 2^8 otherwise. Default value is 2.
```
