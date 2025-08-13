use service_resolver::ServiceResolver;

pub fn get_service_resolver() -> ServiceResolver {
    let environment = crate::repository::runtime_config_repo::get_current_env();

    ServiceResolver::new(environment)
}
