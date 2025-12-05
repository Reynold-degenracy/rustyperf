#!/bin/bash
cross build --release --target-dir /mnt/d/target/x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
cross build --release --target-dir /mnt/d/target/x86_64-pc-windows-gnu --target x86_64-pc-windows-gnu
cross build --release --target-dir /mnt/d/target/aarch64-unknown-linux-gnu --target aarch64-unknown-linux-gnu