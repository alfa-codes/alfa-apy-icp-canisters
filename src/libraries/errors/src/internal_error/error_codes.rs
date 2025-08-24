pub mod module {
    pub mod areas {
        pub mod external_services {
            pub const AREA_CODE: &str = "01";
            pub mod domains {
                pub mod kong_swap {
                    pub const DOMAIN_CODE: &str = "01";
                    pub mod components {
                        pub const CORE: &str = "01";
                    }
                }
                pub mod icp_swap {
                    pub const DOMAIN_CODE: &str = "02";
                    pub mod components {
                        pub const CORE: &str = "01";
                    }
                }
                pub mod icrc_ledger {
                    pub const DOMAIN_CODE: &str = "03";
                    pub mod components {
                        pub const CORE: &str = "01";
                        pub const MOCK_CORE: &str = "51";
                    }
                }
                pub mod canister {
                    pub const DOMAIN_CODE: &str = "04";
                    pub mod components {
                        pub const CORE: &str = "01";
                    }
                }
            }
        }

        pub mod libraries {
            pub const AREA_CODE: &str = "02";
            pub mod domains {
                pub mod swap {
                    pub const DOMAIN_CODE: &str = "01";
                    pub mod components {
                        pub const SWAP_SERVICE: &str = "01";
                        pub const KONG_SWAP: &str = "02";
                        pub const ICP_SWAP: &str = "03";
                    }
                }
                pub mod liquidity {
                    pub const DOMAIN_CODE: &str = "02";
                    pub mod components {
                        pub const CORE: &str = "01";
                        pub const KONG_SWAP_CLIENT: &str = "02";
                        pub const ICP_SWAP_CLIENT: &str = "03";
                    }
                }
                pub mod validation {
                    pub const DOMAIN_CODE: &str = "03";
                    pub mod components {
                        pub const CORE: &str = "01";
                    }
                }
                pub mod provider {
                    pub const DOMAIN_CODE: &str = "04";
                    pub mod components {
                        pub const MOCK_KONG_SWAP: &str = "51";
                        pub const MOCK_ICP_SWAP: &str = "52";
                    }
                }
            }
        }
        pub mod canisters {
            pub const AREA_CODE: &str = "03";
            pub mod domains {
                pub mod vault {
                    pub const DOMAIN_CODE: &str = "01";
                    pub mod components {
                        pub const CORE: &str = "01";
                        pub const STRATEGIES: &str = "02";
                    }
                }
                pub mod pool_stats {
                    pub const DOMAIN_CODE: &str = "02";
                    pub mod components {
                        pub const CORE: &str = "01";
                        pub const POOL_METRICS: &str = "02";
                    }
                }
                pub mod strategy_history {
                    pub const DOMAIN_CODE: &str = "03";
                    pub mod components {
                        pub const CORE: &str = "01";
                        pub const TEST_SNAPSHOTS_SERVICE: &str = "02";
                    }
                }
            }
        }
    }
}

pub mod error_kinds {
    pub const NOT_FOUND: &str = "01";
    pub const VALIDATION: &str = "02";
    pub const BUSINESS_LOGIC: &str = "03";
    pub const EXTERNAL_SERVICE: &str = "04";
    pub const ACCESS_DENIED: &str = "05";
    pub const INFRASTRUCTURE: &str = "06";
    pub const TIMEOUT: &str = "07";
    pub const UNKNOWN: &str = "08";
}
