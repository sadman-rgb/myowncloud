use ic_cdk::export::Principal;

pub fn authenticate(user: Principal) -> bool {
    // Cek identitas pengguna berdasarkan Principal ID dari Internet Identity
    security::validate_principal(user)
}
