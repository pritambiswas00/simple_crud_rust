use sqlx::{ PgPool, Error };

use crate::user::{ model::User, views::NewUserRequest };

pub async fn register_user(pool: PgPool, new_user: NewUserRequest) -> Result<User, Error> {
    let user = sqlx
        ::query_as::<_, User>(
            "
            INSERT INTO
                users
                (
                    email,
                    password,
                    user_name
                )
            VALUES (
                $1,
                $2,
                $3
            )
            RETURNING
                *
            "
        )
        .bind(new_user.email)
        .bind(new_user.password)
        .bind(new_user.user_name)
        .fetch_one(&pool).await?;

    Ok(user)
}

pub async fn find_user_by_email_and_password(
    pool: PgPool,
    email: &str,
    password: &str
) -> Result<User, Error> {
    let user = sqlx
        ::query_as::<_, User>(
            "
          SELECT
              *
          FROM
              users
          WHERE
              email = $1 AND
              password = $2
          "
        )
        .bind(email)
        .bind(password)
        .fetch_one(&pool).await?;

    Ok(user)
}

pub async fn find_user_by_email(pool: PgPool, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx
        ::query_as::<_, User>(
            "
            SELECT
                *
            FROM
                users
            WHERE
                email = $1
            "
        )
        .bind(email)
        .fetch_one(&pool).await?;

    Ok(user)
}

pub async fn find_user_by_id(pool: PgPool, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx
        ::query_as::<_, User>(
            "
            SELECT
                *
            FROM
                users
            WHERE
                id = $1
            "
        )
        .bind(id)
        .fetch_one(&pool).await?;

    Ok(user)
}


