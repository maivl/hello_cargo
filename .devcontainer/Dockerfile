FROM rust:1.70 AS rust

# Install Node.js 20
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs

# Set up work directory
WORKDIR /app

# Copy project files
COPY . .

# Install Node.js dependencies
RUN yarn install