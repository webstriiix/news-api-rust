# Actix Web API

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Environment Configuration](#environment-configuration)
- [Running the Application](#running-the-application)
- [Generating Documentation](#generating-documentation)
- [API Documentation](#api-documentation)
- [Endpoints](#endpoints)
- [Technologies Used](#technologies-used)

## Introduction
This API is designed to help manage users, categories, and news articles. It includes features like secure login, user roles, and tools to create, read, update, and delete (CRUD) categories and news articles.

## Features
- 🔒 Secure login and authentication
- 📂 CRUD operations for categories and news articles
- 🛡️ Admin-only access to certain features
- 📝 Well-organized route structure

## Prerequisites
Make sure you have the following installed before you start:
- Rust and Cargo

## Installation
### Clone the Repository
```sh
git clone <repository_url>
cd actix-web-api
```
### Install Dependencies
```sh
cargo build
```

## Environment Configuration
Create a `.env` file in the project root and add the following settings:
```
# Database Configuration
DATABASE_URL=your_database_connection_string

# Authentication
JWT_SECRET=your_jwt_secret_key
```

## Running the Application
### Development Mode
```sh
cargo run
```
### Production Mode
```sh
cargo build --release
./target/release/actix-web-api
```

## Generating Documentation
Rust has a built-in way to create documentation from comments in the code.
### Generate and View Documentation
1. Run this command to generate the documentation:
   ```sh
   cargo doc --open
   ```
   This will create and open the documentation in your web browser.
2. Make sure to add proper Rust documentation comments (`///` for items, `//!` for module-level comments).

## API Documentation
### Endpoints

### Authentication
- `POST /auth/register` - Register a new user
- `POST /auth/login` - Log in a user

### Admin (Requires Authentication and Admin Privileges)
- `POST /admin/create-news` - Add a news article
- `PUT /admin/news-update/{id}` - Edit a news article
- `GET /admin/list-news` - Show all news articles
- `POST /admin/create-category` - Add a new category
- `PUT /admin/update-category` - Edit a category
- `GET /admin/news-detail/{id}` - Get details of a specific news article
- `DELETE /admin/delete-news/{id}` - Remove a news article
- `DELETE /admin/delete-category/{id}` - Remove a category

### User
- `GET /user/list-news` - Show all news articles

## Technologies Used
- 🚀 Web Framework: Actix Web
- 🛢️ Database: PostgreSQL
- 🔑 Authentication: JWT-based security
- 📦 Dependency Management: Cargo
- 📝 API Testing: Postman or `curl`

## Contributing
Feel free to submit issues and pull requests to improve this API.
