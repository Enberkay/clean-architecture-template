// @generated automatically by Diesel CLI.

diesel::table! {
    book_categories (book_isbn, category_id) {
        #[max_length = 13]
        book_isbn -> Varchar,
        category_id -> Int4,
    }
}

diesel::table! {
    book_images (id) {
        id -> Int4,
        #[max_length = 13]
        book_isbn -> Varchar,
        image_url -> Text,
        #[max_length = 50]
        image_type -> Varchar,
        sort_order -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    books (isbn) {
        #[max_length = 13]
        isbn -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        author -> Nullable<Varchar>,
        synopsis -> Nullable<Text>,
        price -> Numeric,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    branches (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        address -> Nullable<Text>,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    inventories (branch_id, book_isbn) {
        branch_id -> Int4,
        #[max_length = 13]
        book_isbn -> Varchar,
        quantity -> Int4,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    order_items (id) {
        id -> Int4,
        order_id -> Int4,
        #[max_length = 13]
        book_isbn -> Varchar,
        #[max_length = 255]
        book_title -> Varchar,
        #[max_length = 255]
        book_author -> Nullable<Varchar>,
        quantity -> Int4,
        price_at_purchase -> Numeric,
        subtotal -> Numeric,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        order_date -> Timestamptz,
        #[max_length = 50]
        status -> Varchar,
        #[max_length = 50]
        source -> Varchar,
        total_amount -> Numeric,
        shipping_address -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    payments (id) {
        id -> Int4,
        order_id -> Nullable<Int4>,
        sale_id -> Nullable<Int4>,
        #[max_length = 50]
        payment_method -> Varchar,
        #[max_length = 255]
        transaction_ref -> Nullable<Varchar>,
        amount -> Numeric,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    receipts (id) {
        id -> Int4,
        #[max_length = 50]
        receipt_code -> Varchar,
        #[max_length = 20]
        type_name -> Varchar,
        reference_id -> Int4,
        #[max_length = 50]
        source -> Varchar,
        user_id -> Nullable<Int4>,
        branch_id -> Nullable<Int4>,
        total_amount -> Numeric,
        #[max_length = 50]
        payment_method -> Nullable<Varchar>,
        #[max_length = 255]
        payment_ref -> Nullable<Varchar>,
        issued_at -> Timestamptz,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    role_permissions (role_id, permission_id) {
        role_id -> Int4,
        permission_id -> Int4,
        assigned_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    sale_items (id) {
        id -> Int4,
        sale_id -> Int4,
        #[max_length = 13]
        book_isbn -> Varchar,
        #[max_length = 255]
        book_title -> Varchar,
        #[max_length = 255]
        book_author -> Nullable<Varchar>,
        quantity -> Int4,
        price_at_sale -> Numeric,
        subtotal -> Numeric,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    sales (id) {
        id -> Int4,
        employee_id -> Nullable<Int4>,
        branch_id -> Nullable<Int4>,
        sale_date -> Timestamptz,
        total_amount -> Numeric,
        #[max_length = 50]
        payment_method -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Int4,
        role_id -> Int4,
        assigned_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        fname -> Varchar,
        #[max_length = 255]
        lname -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        age -> Int4,
        #[max_length = 50]
        sex -> Varchar,
        #[max_length = 20]
        phone -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        branch_id -> Nullable<Int4>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(book_categories -> books (book_isbn));
diesel::joinable!(book_categories -> categories (category_id));
diesel::joinable!(book_images -> books (book_isbn));
diesel::joinable!(inventories -> books (book_isbn));
diesel::joinable!(inventories -> branches (branch_id));
diesel::joinable!(order_items -> books (book_isbn));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(payments -> orders (order_id));
diesel::joinable!(payments -> sales (sale_id));
diesel::joinable!(receipts -> branches (branch_id));
diesel::joinable!(receipts -> users (user_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(sale_items -> books (book_isbn));
diesel::joinable!(sale_items -> sales (sale_id));
diesel::joinable!(sales -> branches (branch_id));
diesel::joinable!(sales -> users (employee_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(users -> branches (branch_id));

diesel::allow_tables_to_appear_in_same_query!(
    book_categories,
    book_images,
    books,
    branches,
    categories,
    inventories,
    order_items,
    orders,
    payments,
    permissions,
    receipts,
    role_permissions,
    roles,
    sale_items,
    sales,
    user_roles,
    users,
);
