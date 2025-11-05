use std::sync::Arc;
use crate::application::{
    application_errors::{ApplicationError, ApplicationResult},
    dtos::auth_dto::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse},
};
use crate::domain::{
    entities::refresh_token::NewRefreshToken,
    repositories::{
        user_repository::UserRepository,
        password_repository::PasswordRepository,
        token_repository::{JwtRepository, TokenRepository},
    },
};

/// AuthService จัดการ Authentication flow ทั้งหมด
pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    password_repo: Arc<dyn PasswordRepository>,
    jwt_repo: Arc<dyn JwtRepository>,
    token_repo: Arc<dyn TokenRepository>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_repo: Arc<dyn PasswordRepository>,
        jwt_repo: Arc<dyn JwtRepository>,
        token_repo: Arc<dyn TokenRepository>,
    ) -> Self {
        Self {
            user_repo,
            password_repo,
            jwt_repo,
            token_repo,
        }
    }

    /// สมัครสมาชิกใหม่
    pub async fn register(&self, req: RegisterRequest) -> ApplicationResult<RegisterResponse> {
        //ตรวจสอบว่าอีเมลซ้ำไหม
        if let Some(_) = self.user_repo.find_by_email(&req.email).await.map_err(|e| {
            ApplicationError::internal(format!("Database error while checking email: {}", e))
        })? {
            return Err(ApplicationError::conflict("Email already exists"));
        }

        //hash password
        let hashed_password = self.password_repo.hash(&req.password).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to hash password: {}", e))
        })?;

        //สร้าง user entity
        let user = crate::domain::entities::user::UserEntity::new(
            req.fname,
            req.lname,
            req.email,
            req.age,
            req.sex,
            req.phone,
            hashed_password,
            None,
        )
        .map_err(|e| ApplicationError::bad_request(e.to_string()))?;

        //save user
        let user_id = self.user_repo.save(&user).await.map_err(|e| {
                    ApplicationError::internal(format!("Failed to save user: {}", e))
                })?;

        Ok(RegisterResponse {
            id: user_id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.clone(),
            lname: user.last_name.clone(),
        })
    }

    /// เข้าสู่ระบบ
    pub async fn login(&self, req: LoginRequest) -> ApplicationResult<(LoginResponse, String)> {
        //ค้นหาผู้ใช้
        let user_opt = self.user_repo.find_by_email(&req.email).await.map_err(|e| {
            ApplicationError::internal(format!("Database error while fetching user: {}", e))
        })?;

        let user = match user_opt {
            Some(u) => u,
            None => return Err(ApplicationError::unauthorized("Invalid credentials")),
        };

        //ตรวจรหัสผ่าน
        let valid = self.password_repo.verify(&req.password, &user.password).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to verify password: {}", e))
        })?;

        if !valid {
            return Err(ApplicationError::unauthorized("Invalid credentials"));
        }

        //generate tokens
        let access_token = self
            .jwt_repo
            .create_access_token(user.id, &[], &[])
            .await
            .map_err(|e| ApplicationError::internal(format!("Failed to create access token: {}", e)))?;

        let refresh_token = self
            .jwt_repo
            .create_refresh_token(user.id, 7)
            .await
            .map_err(|e| ApplicationError::internal(format!("Failed to create refresh token: {}", e)))?;

        let refresh_token_hash = self
            .jwt_repo
            .hash_refresh_token(&refresh_token)
            .await
            .map_err(|e| ApplicationError::internal(format!("Failed to hash refresh token: {}", e)))?;

        //store refresh token
        let token_data = NewRefreshToken {
            user_id: user.id,
            token_hash: refresh_token_hash,
            expires_at: chrono::Utc::now() + chrono::Duration::days(7),
        };

        self.token_repo.store_refresh_token(token_data).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to store refresh token: {}", e))
        })?;

        Ok((
            LoginResponse {
                access_token,
            },
            refresh_token,
        ))
    }

    ///Validate token and return user id
    pub async fn validate_token(&self, token: &str) -> ApplicationResult<i32> {
        let user_id = self.jwt_repo.validate_access_token(token).await.map_err(|e| {
            ApplicationError::unauthorized(format!("Invalid or expired token: {}", e))
        })?;
        Ok(user_id)
    }

    ///Refresh access token using refresh token
    pub async fn refresh_token(&self, refresh_token: &str) -> ApplicationResult<(LoginResponse, String)> {
        // Hash the refresh token to compare with stored hash
        let refresh_token_hash = self
            .jwt_repo
            .hash_refresh_token(refresh_token)
            .await
            .map_err(|e| ApplicationError::internal(format!("Failed to hash refresh token: {}", e)))?;

        // Get stored refresh token from repository
        let stored_token = self.token_repo.get_refresh_token(&refresh_token_hash).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to get refresh token: {}", e))
        })?;

        let stored_token = match stored_token {
            Some(token) => token,
            None => return Err(ApplicationError::unauthorized("Invalid or expired refresh token")),
        };

        // Check if token is expired
        if stored_token.expires_at < chrono::Utc::now() {
            // Clean up expired token
            let _ = self.token_repo.revoke_token(&refresh_token_hash).await;
            return Err(ApplicationError::unauthorized("Refresh token expired"));
        }

        // Generate new access token
        let new_access_token = self
            .jwt_repo
            .create_access_token(stored_token.user_id, &[], &[])
            .await
            .map_err(|e| ApplicationError::internal(format!("Failed to create access token: {}", e)))?;

        // Generate new refresh token (rotation)
        let new_refresh_token = self
            .jwt_repo
            .create_refresh_token(stored_token.user_id, 7)
            .await
            .map_err(|e| ApplicationError::internal(format!("Failed to create refresh token: {}", e)))?;

        let new_refresh_token_hash = self
            .jwt_repo
            .hash_refresh_token(&new_refresh_token)
            .await
            .map_err(|e| ApplicationError::internal(format!("Failed to hash refresh token: {}", e)))?;

        // Store new refresh token
        let token_data = NewRefreshToken {
            user_id: stored_token.user_id,
            token_hash: new_refresh_token_hash.clone(),
            expires_at: chrono::Utc::now() + chrono::Duration::days(7),
        };

        self.token_repo.store_refresh_token(token_data).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to store refresh token: {}", e))
        })?;

        // Revoke old refresh token
        self.token_repo.revoke_token(&refresh_token_hash).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to revoke old refresh token: {}", e))
        })?;

        Ok((
            LoginResponse {
                access_token: new_access_token,
            },
            new_refresh_token,
        ))
    }

    ///Logout (revoke refresh token)
    pub async fn logout(&self, refresh_token_hash: &str) -> ApplicationResult<()> {
        self.token_repo.revoke_token(refresh_token_hash).await.map_err(|e| {
            ApplicationError::internal(format!("Failed to revoke token: {}", e))
        })
    }
}
