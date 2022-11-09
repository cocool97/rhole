pub fn reverse_domain_name(input: &str) -> String {
    // Addresses are added in reversed form
    // e.g fr.facebook.com would be added as com.facebook.fr
    // Like that we can iterate over each component of domain name and check if it'll be contained
    let mut rev_address = String::with_capacity(input.len());
    for component in input.split('.').rev() {
        rev_address.push_str(component);
        rev_address.push('.');
    }
    rev_address.pop();

    rev_address
}
