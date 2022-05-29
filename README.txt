This is ambrose, a program for playing music using stepper motors.

In order to run ambrose, you'll need to install Rust. To do this, follow the
instructions at

    https://www.rust-lang.org/tools/install

ambrose is designed to run on a Raspberry Pi. The `run_raspi.sh` script runs
ambrose on CPU 3, so for best results, it's probably a good idea to configure
the Pi so that no other processes will automatically run on CPU 3. You can do
this by editing the `/boot/cmdline.txt` file by adding the `isolcpus=3` option
to it. For example, if the contents of your `/boot/cmdline.txt` file are

    console=serial0,115200 console=tty1 root=PARTUUID=ffffffff-ff rootfstype=ext4 fsck.repair=yes rootwait quiet splash plymouth.ignore-serial-consoles

then you should add `isolcpus=3` to the end, so that it will look something like

    console=serial0,115200 console=tty1 root=PARTUUID=ffffffff-ff rootfstype=ext4 fsck.repair=yes rootwait quiet splash plymouth.ignore-serial-consoles isolcpus=3

You can also run ambrose on a PC using `run_rodio.sh`. (Under Windows, you'll
need to use Cygwin or something.) This will generate a square wave and play it
through the system speakers.

As of this writing, there is no way to tell ambrose which song to play; you'll
have to edit main.rs manually in order to get it to play a different song.
