# Rust Authentication API with Actix-Web

[![Thumbnail](https://img.youtube.com/vi/X74uyTr5kk8/maxresdefault.jpg)](https://youtu.be/X74uyTr5kk8)

This repository contains a comprehensive Rust-based API project using Actix-Web. The API includes robust JWT-based authentication, user management, and Swagger UI integration for interactive API documentation.

## Table of Contents

- [Rust Authentication API with Actix-Web](#rust-authentication-api-with-actix-web)
  - [Table of Contents](#table-of-contents)
  - [Project Overview](#project-overview)
    - [Key Features](#key-features)
  - [Project Structure](#project-structure)
  - [Setup Instructions](#setup-instructions)
  - [Prerequisites](#prerequisites)
  - [Environment Variables](#environment-variables)
  - [Database Migrations](#database-migrations)
  - [Running the Server](#running-the-server)
  - [API Endpoints](#api-endpoints)
    - [Authentication Endpoints](#authentication-endpoints)
    - [User Endpoints](#user-endpoints)
  - [Swagger UI](#swagger-ui)
  - [Middleware](#middleware)
    - [Authentication Middleware Guard](#authentication-middleware-guard)
    - [Role-Based Access Control](#role-based-access-control)
  - [OpenAPI Documentation](#openapi-documentation)
    - [Adding and Customizing OpenAPI Documentation](#adding-and-customizing-openapi-documentation)
  - [License](#license)
  - [Help full commands](#help-full-commands)

## Project Overview

This project showcases how to build a complete authentication system with Rust, using Actix-Web. The API is designed to be scalable and secure, integrating PostgreSQL for data storage and JWT for user authentication.

### Key Features

- **User Authentication:** Secure user registration, login, and logout using JWT.
- **User Management:** Retrieve user information and list all users.
- **Database Integration:** PostgreSQL with SQLx for efficient database handling.
- **Environment Configurations:** Manage configurations via environment variables.
- **Error Handling:** Comprehensive error handling with custom error types.
- **API Documentation:** Integrated Swagger UI for interactive API documentation.
- **Middleware:** Custom middleware for authentication and role-based access control.

## Project Structure

```plaintext
├── src
│   ├── handlers           # API route handlers
│   │   ├── mod.rs         # Utility functions module
│   │   ├── auth.rs        # Authentication-related handlers
│   │   ├── users.rs       # User-related handlers
│   ├── utils              # Utility functions
│   │   ├── mod.rs         # Utility functions module
│   │   ├── password.rs    # Password hashing and verification
│   │   ├── token.rs       # JWT signing and verification
│   ├── auth.rs            # Middleware implementations
│   ├── config.rs          # Configuration file for environment variables
│   ├── db.rs              # Database access layer
│   ├── dtos.rs            # Data Transfer Objects (DTOs)
│   ├── error.rs           # Error handling module
│   ├── main.rs            # Application entry point
│   ├── models.rs          # Database models
├── migrations             # Database migrations folder (created by SQLx)
├── .env                   # Environment variables file
├── Cargo.toml             # Rust dependencies and project metadata
├── README.md              # Project documentation
```

## Setup Instructions

## Prerequisites

Ensure you have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)
- [SQLx-CLI](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) (for database migrations)

## Environment Variables

Create a `.env` file in the root directory with the following content:

    DATABASE_URL=postgres://username:password@localhost/dbname
    JWT_SECRET=your_jwt_secret_key
    JWT_EXPIRATION=60  # JWT expiration time in minutes

Replace the placeholders with your actual database credentials and desired JWT configuration.

## Database Migrations

Run the following command to perform database migrations:

    sqlx migrate run

This will set up the necessary database schema for the application.

## Running the Server

Start the server using the command:

    cargo run
    
The API will be accessible at <http://localhost:8000>.

## API Endpoints

### Authentication Endpoints

- **Register User:** `POST /api/auth/register`
- **Login User:** `POST /api/auth/login`
- **Logout User:** `POST /api/auth/logout`

### User Endpoints

- **Get Authenticated User:** `GET /api/users/me`
- **List Users:** `GET /api/users`

Each endpoint is protected by JWT-based authentication, ensuring secure access.

## Swagger UI

Swagger UI is integrated for interactive API exploration and documentation. Access it by navigating to:

<http://localhost:8000/>

Here, you can view all the available endpoints, along with detailed information on request and response formats.

## Middleware

### Authentication Middleware Guard

The custom authentication middleware guards routes by verifying the presence and validity of JWT tokens. It ensures that only authenticated users can access certain endpoints.

### Role-Based Access Control

In addition to authentication, some routes enforce role-based access control (RBAC) using the `RequireAuth` middleware, which checks user roles like `Admin`, `Moderator`, or `User`.

## OpenAPI Documentation

The project supports OpenAPI 3.0, with schema generation and endpoint documentation provided through the `utoipa` crate.

### Adding and Customizing OpenAPI Documentation

- **Register the OpenAPI Schema:** The `openapi` object is configured in `main.rs`.
- **Register the API Handler as OpenAPI Path:** Each handler is registered as an OpenAPI path with detailed descriptions.
- **Serving the Swagger UI:** The OpenAPI object is served via a web server, accessible through Swagger UI.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Help full commands

```bash
    cargo install sqlx-cli --no-default-features --features postgres
```

```bash
    sqlx migrate add -r user_table
```

```bash
    sqlx database create
```

```bash
    sqlx migrate run
```
