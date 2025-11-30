use std::sync::Arc;
use crate::application::{
    dtos::auth_dto::{LoginRequest, LoginResponse, RefreshResponse, RegisterRequest, RegisterResponse, UserInfo},
};
use anyhow::{Result, anyhow};
use crate::{
    domain::repositories::{
        user_repository::UserRepository,
    },
    infrastructure::{
        argon2::PasswordService,
        jwt::JwtService,
    },
};

/// AuthUseCase จัดการ Authentication flow ทั้งหมด (AT/RT Stateless)
pub struct AuthUseCase {
    user_repo: Arc<dyn UserRepository>,
    password_repo: Arc<dyn PasswordService>,
    jwt_repo: Arc<dyn JwtService>,
}

impl AuthUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_repo: Arc<dyn PasswordService>,
        jwt_repo: Arc<dyn JwtService>,
    ) -> Self {
        Self {
            user_repo,
            password_repo,
            jwt_repo,
        }
    }

    /// สมัครสมาชิกใหม่
    pub async fn register(&self, req: RegisterRequest) -> Result<RegisterResponse> {
        // ตรวจสอบว่าอีเมลซ้ำไหม
        if let Some(_) = self.user_repo.find_by_email(&req.email).await.map_err(|e| {
            anyhow!("Database error while checking email: {}", e)
        })? {
            return Err(anyhow!("Email already exists"));
        }

        // hash password
        let hashed_password = self.password_repo.hash_password(&req.password).await.map_err(|e| {
            anyhow!("Failed to hash password: {}", e)
        })?;

        // สร้าง user entity
        let user = crate::domain::entities::user::UserEntity::new(
            req.fname,
            req.lname,
            req.email,
            req.age,
            req.sex,
            req.phone,
            hashed_password,
        )
        .map_err(|e| anyhow!("{}", e))?;

        //save user
        let user_id = self.user_repo.save(&user).await.map_err(|e| {
            anyhow!("Failed to save user: {}", e)
        })?;

        Ok(RegisterResponse {
            id: user_id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.clone(),
            lname: user.last_name.clone(),
        })
    }

    /// เข้าสู่ระบบ - สร้างทั้ง AT และ RT (RT ส่งกลับเพื่อเก็บใน cookie, AT อยู่ใน response)
    pub async fn login(&self, req: LoginRequest) -> Result<(LoginResponse, String)> {
        // ค้นหาผู้ใช้
        let user_opt = self.user_repo.find_by_email(&req.email).await.map_err(|e| {
            anyhow!("Database error while fetching user: {}", e)
        })?;

        let user = match user_opt {
            Some(u) => u,
            None => return Err(anyhow!("Invalid credentials")),
        };

        // ตรวจรหัสผ่าน
        let valid = self.password_repo.verify_password(&req.password, &user.password).await.map_err(|e| {
            anyhow!("Failed to verify password: {}", e)
        })?;

        if !valid {
            return Err(anyhow!("Invalid credentials"));
        }

        // Get user's real roles
        let roles = self.user_repo.find_roles(user.id).await
            .map_err(|e| anyhow!("Failed to fetch user roles: {}", e))?;
        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();

        // สร้าง Access Token
        let access_token = self
            .jwt_repo
            .generate_access_token(user.id, &role_names)
            .await
            .map_err(|e| anyhow!("Failed to create access token: {}", e))?;

        // สร้าง Refresh Token
        let refresh_token = self
            .jwt_repo
            .generate_refresh_token(user.id)
            .await
            .map_err(|e| anyhow!("Failed to create refresh token: {}", e))?;

        // สร้าง user info สำหรับ response
        let user_info = UserInfo {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.clone(),
            lname: user.last_name.clone(),
            roles: role_names,
        };

        Ok((LoginResponse {
            user: user_info,
            access_token: access_token.clone(),
        }, refresh_token))
    }

    /// Refresh token flow - ใช้ RT สร้าง AT ใหม่และคืน user info (AT อยู่ใน response แล้ว)
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<RefreshResponse> {
        // ตรวจสอบ RT โดยใช้ refresh secret
        let user_id = self.jwt_repo.validate_refresh_token(refresh_token).await.map_err(|e| {
            anyhow!("Invalid or expired refresh token: {}", e)
        })?;

        // ค้นหาข้อมูล user
        let user = self.user_repo.find_by_id(user_id).await.map_err(|e| {
            anyhow!("Database error while fetching user: {}", e)
        })?.ok_or_else(|| anyhow!("User not found"))?;

        // Get user's real roles
        let roles = self.user_repo.find_roles(user.id).await
            .map_err(|e| anyhow!("Failed to fetch user roles: {}", e))?;
        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();

        // สร้าง Access Token ใหม่
        let new_access_token = self
            .jwt_repo
            .generate_access_token(user.id, &role_names)
            .await
            .map_err(|e| anyhow!("Failed to create access token: {}", e))?;

        // สร้าง user info สำหรับ response
        let user_info = UserInfo {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.clone(),
            lname: user.last_name.clone(),
            roles: role_names,
        };

        Ok(RefreshResponse {
            user: user_info,
            access_token: new_access_token,
        })
    }

    /// Validate access token และคืน user info
    pub async fn validate_token(&self, token: &str) -> Result<UserInfo> {
        // ตรวจสอบ AT
        let user_id = self.jwt_repo.validate_token(token).await.map_err(|e| {
            anyhow!("Invalid or expired access token: {}", e)
        })?;

        // ค้นหาข้อมูล user
        let user = self.user_repo.find_by_id(user_id).await.map_err(|e| {
            anyhow!("Database error while fetching user: {}", e)
        })?.ok_or_else(|| anyhow!("User not found"))?;

        // Get user's real roles
        let roles = self.user_repo.find_roles(user.id).await
            .map_err(|e: anyhow::Error| anyhow!("Failed to fetch user roles: {}", e))?;
        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();

        // สร้าง user info สำหรับ response
        let user_info = UserInfo {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.clone(),
            lname: user.last_name.clone(),
            roles: role_names,
        };

        Ok(user_info)
    }
}