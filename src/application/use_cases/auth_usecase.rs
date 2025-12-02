use std::sync::Arc;
use crate::application::{
    dtos::auth_dto::{LoginRequest, LoginResponse, RefreshResponse, RegisterRequest, RegisterResponse, UserInfo},
};
use anyhow::{Result, anyhow, Context};
use crate::{
    domain::repositories::{
        user_repository::UserRepository,
    },
    infrastructure::{
        argon2::PasswordService,
        jwt::JwtService,
    },
    domain::entities::user::UserEntity,
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
        if self.user_repo.find_by_email(&req.email).await
            .context("Database error while checking email")?
            .is_some() 
        {
            return Err(anyhow!("Email already exists"));
        }

        // hash password
        let hashed_password = self.password_repo.hash_password(&req.password).await
            .context("Failed to hash password")?;

        // สร้าง user entity
        let user = UserEntity::new(
            req.fname,
            req.lname,
            req.email,
            req.age,
            req.sex,
            req.phone,
            hashed_password,
        ).map_err(|e| anyhow!("{}", e))?;

        // Save user
        let user_id = self.user_repo.save(&user).await
            .context("Failed to save user")?;

        Ok(RegisterResponse {
            id: user_id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.as_str().to_string(), 
            lname: user.last_name.as_str().to_string(),
        })
    }

    /// เข้าสู่ระบบ
    pub async fn login(&self, req: LoginRequest) -> Result<(LoginResponse, String)> {
        // 1. ค้นหาผู้ใช้
        let user = self.user_repo.find_by_email(&req.email).await
            .context("Database error while fetching user")?
            .ok_or_else(|| anyhow!("Invalid credentials"))?;

        // 2. ตรวจรหัสผ่าน
        let valid = self.password_repo.verify_password(&req.password, user.password.as_str()).await
            .context("Failed to verify password")?;

        if !valid {
            return Err(anyhow!("Invalid credentials"));
        }

        // 3. ดึง Role ล่าสุด (สำคัญมาก ไม่ควร hardcode)
        let roles = self.user_repo.find_roles(user.id).await
            .context("Failed to fetch user roles")?;
        
        let role_names: Vec<String> = roles.iter()
            .map(|r| r.name.as_str().to_string()) 
            .collect();

        // 4. สร้าง Access Token (ใส่ Role เข้าไปใน Token เลย)
        let access_token = self.jwt_repo
            .generate_access_token(user.id, &role_names)
            .await
            .context("Failed to create access token")?;

        // 5. สร้าง Refresh Token (แบบ Stateless)
        let refresh_token = self.jwt_repo
            .generate_refresh_token(user.id)
            .await
            .context("Failed to create refresh token")?;

        let user_info = UserInfo {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.as_str().to_string(),
            lname: user.last_name.as_str().to_string(),
            roles: role_names,
        };

        Ok((LoginResponse {
            user: user_info,
            access_token,
        }, refresh_token))
    }

    /// Refresh token flow
    /// Flow: Validate RT -> Check DB (Active?) -> Get Roles -> Issue New AT
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<RefreshResponse> {
        

        // 1. ตรวจสอบ Refresh Token (Signature & Expiry)
        // หมายเหตุ: ใน jwt.rs ใหม่ validate_refresh_token คืนค่า i32 (user_id)
        let user_id = self.jwt_repo.validate_refresh_token(refresh_token).await
            .context("Invalid or expired refresh token")?;

        // 2. Security Check: ต้องเช็คกับ DB ว่า User ยังมีตัวตนอยู่ไหม (กันกรณี User โดนลบ/แบน แต่ RT ยังไม่หมดอายุ)
        let user = self.user_repo.find_by_id(user_id).await
            .context("Database error while fetching user")?
            .ok_or_else(|| anyhow!("User not found or account deactivated"))?;

        // 3. ดึง Role ล่าสุดจาก DB เสมอ (เผื่อมีการปรับเปลี่ยนสิทธิ์ระหว่างที่ RT ยังไม่หมดอายุ)
        let roles = self.user_repo.find_roles(user.id).await
            .context("Failed to fetch user roles")?;
        
        let role_names: Vec<String> = roles.iter()
            .map(|r| r.name.as_str().to_string())
            .collect();

        // 4. ออก Access Token ใบใหม่
        let new_access_token = self.jwt_repo
            .generate_access_token(user.id, &role_names)
            .await
            .context("Failed to create new access token")?;

        let user_info = UserInfo {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.as_str().to_string(),
            lname: user.last_name.as_str().to_string(),
            roles: role_names,
        };

        Ok(RefreshResponse {
            user: user_info,
            access_token: new_access_token,
        })
    }

    /// Validate access token และคืน user info
    /// ใช้สำหรับ Endpoint ที่ต้องการรายละเอียด User (`/me`)
    pub async fn validate_token(&self, token: &str) -> Result<UserInfo> {
        // 1. ตรวจสอบ AT (เปลี่ยนจาก validate_token -> validate_access_token)
        // คืนค่าเป็น Claims { sub, roles, ... }
        let claims = self.jwt_repo.validate_access_token(token).await
            .context("Invalid or expired access token")?;

        // 2. แปลง sub (String) กลับเป็น user_id (i32)
        let user_id = claims.sub.parse::<i32>()
            .map_err(|_| anyhow!("Invalid user ID format in token"))?;

        // 3. จำเป็นต้อง Query DB เพราะ UserInfo ต้องการ fname/lname 
        // (ซึ่งเราไม่ได้ใส่ไว้ใน Token เพื่อประหยัดขนาด Token)
        let user = self.user_repo.find_by_id(user_id).await
            .context("Database error while fetching user")?
            .ok_or_else(|| anyhow!("User not found"))?;

        // หมายเหตุ: จริงๆ เราใช้ Role จาก claims ก็ได้เพื่อลด DB Query 
        // แต่การ Query ใหม่ชัวร์กว่าเรื่อง Real-time consistency 
        // ในที่นี้ผมใช้ Query ตามโค้ดเดิมของคุณเพื่อความชัวร์
        let roles = self.user_repo.find_roles(user.id).await
            .context("Failed to fetch user roles")?;
        
        let role_names: Vec<String> = roles.iter()
            .map(|r| r.name.as_str().to_string())
            .collect();

        Ok(UserInfo {
            id: user.id,
            email: user.email.as_str().to_string(),
            fname: user.first_name.as_str().to_string(),
            lname: user.last_name.as_str().to_string(),
            roles: role_names,
        })
    }
}