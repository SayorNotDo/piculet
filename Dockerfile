FROM purtontech/rust-on-nails-devcontainer:1.2.1 AS development


ARG USERNAME=root
RUN sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target