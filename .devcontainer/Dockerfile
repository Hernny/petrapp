FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

# Actualiza paquetes base (incluye glibc 2.39 vía repos de Noble)
RUN apt-get update && apt-get upgrade -y && \
    rm -rf /var/lib/apt/lists/*

# Configuración de locales para evitar warnings de Perl
RUN apt-get update && apt-get install -y locales \
    && sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen \
    && locale-gen
    
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

# Instala dependencias del sistema
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libudev-dev \
    llvm \
    libclang-dev \
    protobuf-compiler \
    libssl-dev \
    curl \
    git \
    ca-certificates \
    postgresql-client \
    redis-tools \
    && rm -rf /var/lib/apt/lists/*

# Instala Node.js (versión LTS recomendada para desarrollo con Solana)
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get install -y nodejs

# Habilita Corepack para Yarn (Node >=16 suele traerlo)
RUN corepack enable && corepack prepare yarn@stable --activate

# Instalación oficial de Solana (incluye Rust, Solana CLI, Anchor)
RUN curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

# Configura PATH explícitamente
ENV PATH="/root/.local/share/solana/install/active_release/bin:/root/.cargo/bin:${PATH}"

# Instala Nx CLI globalmente
RUN npm install -g nx@latest

WORKDIR /workspace

CMD ["/bin/bash"]
