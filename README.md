# fs
This is my first attempt at rust programming.
I pretty quickly ran into a weird issue regarding UTF encoding on OSX.
The issue I have is that the file/dir code returns dirents which are not valid UTF-8,
for at least two cases I have run into so far.
1. Synology NAS SMB mounted volumes.
2. Dropbox.

This can be demonstrated using this programme as follows:

```console
mkdir /nas/tmp
fs --funny -d /nas/tmp
fs -d /nas/tmp
```
... where /nas/tmp is an SMB mounted volume.

```console
mkdir ~/Dropbox/funny
fs --funny -d ~/Dropbox/funny
fs -d ~/Dropbox/funny
```
You can contrast that to OSX local:
```console
mkdir /tmp/funny
fs --funny -d /tmp/funny
fs -d /tmp/funny
```

When the problem is apparent, a dirent obtained when traversing the filesystem is converted to legal UTF-8.
If the conversion differs from the original, the programme offers to attempt to fix it using the `rename` system call.
This appears to succeed without error, but has no effect.

The valid encoding and the invalid encoding differ.  However, both appear to name the file.

This problem exists for directorys as well, but I only handle files right now.
