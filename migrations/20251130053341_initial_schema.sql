-- =====================================================
-- =============== RBAC CORE TABLES ====================
-- =====================================================

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    fname VARCHAR(255) NOT NULL,
    lname VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    age INTEGER NOT NULL CHECK (age >= 0 AND age <= 150),
    sex VARCHAR(50) NOT NULL CHECK (sex IN ('MALE', 'FEMALE', 'OTHER')),
    phone VARCHAR(20) NOT NULL,
    password VARCHAR(255) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_active ON users(is_active);

CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_roles (
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id INTEGER NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- =====================================================
-- ================== BOOK CATALOG =====================
-- =====================================================

CREATE TABLE books (
    isbn VARCHAR(13) PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255),
    synopsis TEXT,
    price DECIMAL(10,2) NOT NULL CHECK (price >= 0),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_books_title ON books(title);
CREATE INDEX idx_books_active ON books(is_active);
CREATE INDEX idx_books_author ON books(author);

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE book_categories (
    book_isbn VARCHAR(13) NOT NULL REFERENCES books(isbn) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (book_isbn, category_id)
);

CREATE INDEX idx_book_categories_book ON book_categories(book_isbn);
CREATE INDEX idx_book_categories_category ON book_categories(category_id);

-- =====================================================
-- =============== BOOK IMAGES (MULTIPLE) ==============
-- =====================================================

CREATE TABLE book_images (
    id SERIAL PRIMARY KEY,
    book_isbn VARCHAR(13) NOT NULL REFERENCES books(isbn) ON DELETE CASCADE,
    image_url TEXT NOT NULL,
    image_type VARCHAR(50) NOT NULL DEFAULT 'GALLERY' CHECK (image_type IN ('COVER', 'PREVIEW', 'GALLERY')),
    sort_order INTEGER NOT NULL DEFAULT 0 CHECK (sort_order >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_book_images_book_isbn ON book_images(book_isbn);
CREATE INDEX idx_book_images_type ON book_images(image_type);

-- =====================================================
-- ================== BRANCHES =========================
-- =====================================================

CREATE TABLE branches (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address TEXT,
    phone VARCHAR(20),
    email VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_branches_active ON branches(is_active);

-- =====================================================
-- ================== INVENTORY ========================
-- =====================================================

CREATE TABLE inventories (
    branch_id INTEGER NOT NULL REFERENCES branches(id) ON DELETE CASCADE,
    book_isbn VARCHAR(13) NOT NULL REFERENCES books(isbn) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 0 CHECK (quantity >= 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (branch_id, book_isbn)
);

CREATE INDEX idx_inventories_book ON inventories(book_isbn);
CREATE INDEX idx_inventories_branch ON inventories(branch_id);
CREATE INDEX idx_inventories_quantity ON inventories(quantity) WHERE quantity > 0;

-- =====================================================
-- ================== ORDERS SYSTEM ====================
-- =====================================================

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    order_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'CONFIRMED', 'SHIPPED', 'DELIVERED', 'CANCELLED')),
    source VARCHAR(50) NOT NULL DEFAULT 'ONLINE' CHECK (source IN ('ONLINE', 'POS', 'PHONE')),
    total_amount DECIMAL(10,2) NOT NULL CHECK (total_amount >= 0),
    shipping_address TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_date ON orders(order_date);
CREATE INDEX idx_orders_source ON orders(source);

CREATE TABLE order_items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    book_isbn VARCHAR(13) NOT NULL REFERENCES books(isbn) ON DELETE RESTRICT,
    book_title VARCHAR(255) NOT NULL,
    book_author VARCHAR(255),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    price_at_purchase DECIMAL(10,2) NOT NULL CHECK (price_at_purchase >= 0),
    subtotal DECIMAL(10,2) NOT NULL CHECK (subtotal >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_order_items_book_isbn ON order_items(book_isbn);

-- =====================================================
-- ================== SALES SYSTEM =====================
-- =====================================================

CREATE TABLE sales (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    branch_id INTEGER REFERENCES branches(id) ON DELETE SET NULL,
    sale_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    total_amount DECIMAL(10,2) NOT NULL CHECK (total_amount >= 0),
    payment_method VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sales_employee_id ON sales(employee_id);
CREATE INDEX idx_sales_branch_id ON sales(branch_id);
CREATE INDEX idx_sales_date ON sales(sale_date);
CREATE INDEX idx_sales_payment_method ON sales(payment_method);

CREATE TABLE sale_items (
    id SERIAL PRIMARY KEY,
    sale_id INTEGER NOT NULL REFERENCES sales(id) ON DELETE CASCADE,
    book_isbn VARCHAR(13) NOT NULL REFERENCES books(isbn),
    book_title VARCHAR(255) NOT NULL,
    book_author VARCHAR(255),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    price_at_sale DECIMAL(10,2) NOT NULL CHECK (price_at_sale >= 0),
    subtotal DECIMAL(10,2) NOT NULL CHECK (subtotal >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_sale_items_sale_id ON sale_items(sale_id);
CREATE INDEX idx_sale_items_book_isbn ON sale_items(book_isbn);

-- =====================================================
-- ================== PAYMENTS =========================
-- =====================================================

CREATE TABLE payments (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id) ON DELETE CASCADE,
    sale_id INTEGER REFERENCES sales(id) ON DELETE CASCADE,
    payment_method VARCHAR(50) NOT NULL,
    transaction_ref VARCHAR(255),
    amount DECIMAL(10,2) NOT NULL CHECK (amount >= 0),
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'COMPLETED', 'FAILED', 'REFUNDED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_order_id ON payments(order_id);
CREATE INDEX idx_payments_sale_id ON payments(sale_id);
CREATE INDEX idx_payments_status ON payments(status);
CREATE INDEX idx_payments_transaction_ref ON payments(transaction_ref) WHERE transaction_ref IS NOT NULL;

-- =====================================================
-- ================ RECEIPTS (Centralized) ===========
-- =====================================================

CREATE TABLE receipts (
    id SERIAL PRIMARY KEY,
    receipt_code VARCHAR(50) NOT NULL UNIQUE,
    type_name VARCHAR(20) NOT NULL CHECK (type_name IN ('SALE', 'ORDER')),
    reference_id INTEGER NOT NULL,
    source VARCHAR(50) NOT NULL CHECK (source IN ('POS', 'ONLINE')),
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    branch_id INTEGER REFERENCES branches(id) ON DELETE SET NULL,
    total_amount DECIMAL(10,2) NOT NULL CHECK (total_amount >= 0),
    payment_method VARCHAR(50),
    payment_ref VARCHAR(255),
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status VARCHAR(50) NOT NULL DEFAULT 'PAID' CHECK (status IN ('PAID', 'CANCELLED', 'REFUNDED')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_receipts_type ON receipts(type_name);
CREATE INDEX idx_receipts_source ON receipts(source);
CREATE INDEX idx_receipts_user_id ON receipts(user_id);
CREATE INDEX idx_receipts_branch_id ON receipts(branch_id);
CREATE INDEX idx_receipts_issued_at ON receipts(issued_at);
CREATE INDEX idx_receipts_receipt_code ON receipts(receipt_code);
CREATE INDEX idx_receipts_reference_id ON receipts(reference_id);

-- =====================================================
-- ================ SAMPLE DATA SEEDING ==============
-- =====================================================

-- Insert default roles (ไม่มี permissions แล้ว)
INSERT INTO roles (name, description) VALUES
('ADMIN', 'System administrator with full access'),
('MANAGER', 'Store manager with operational access'),
('EMPLOYEE', 'Store employee with basic operational access'),
('CUSTOMER', 'Customer with self-service access');
