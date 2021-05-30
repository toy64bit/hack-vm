# docker run build -t docker-riscv-spike-toolchain .
# Uses `ubuntu` image (not `rust`) because SiFive's compiled tools work on Ubuntu 14.04.
FROM ubuntu:14.04
WORKDIR /workspace

# Configure apt and install packages
RUN apt-get update \
  && apt-get -y install --no-install-recommends apt-utils dialog 2>&1 \
  # Verify git, process tools installed
  && apt-get -y install git iproute2 procps \
  # Install host's gcc for `cargo install`
  && apt-get -y install gcc \
  # Install curl
  && apt-get -y install curl \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Install toolchain for RV32/RV64 from SiFive.
# Check for the latest package here: https://www.sifive.com/boards
RUN mkdir /opt/riscv64-toolchain \
  && curl -L https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14.tar.gz | tar xz -C /opt
ENV PATH $PATH:/opt/riscv64-unknown-elf-gcc-8.3.0-2019.08.0-x86_64-linux-ubuntu14/bin

# Install Spike
RUN apt-get update \
  && apt-get -y install device-tree-compiler build-essential
RUN git clone https://github.com/riscv/riscv-isa-sim.git \
  && cd riscv-isa-sim \
  && mkdir build \
  && cd build \
  && ../configure --prefix=/opt/riscv-isa-sim \
  && make \
  && make install
ENV PATH $PATH:/opt/riscv-isa-sim/bin

CMD ["bash"]