use std::ops::Deref;

use super::*;

use serial_test::serial;
use sha256::digest;
use tide_testing::TideTestingExt;

#[serial]
#[async_std::test]
async fn make() {
    reset_all().await;

    let mut app = tide::new();
    app.at("/api/account/manage/create")
        .post(crate::account::handle::manage::make_account);

    let account_id = 123456;
    let password = "password123456";

    let token;

    crate::account::INSTANCE
        .push(crate::account::Account::Verified {
            id: account_id,
            attributes: crate::account::UserAttributes {
                email: lettre::Address::new("yujiening2025", "i.pkuschool.edu.cn").unwrap(),
                name: "Jiening Yu".to_string(),
                school_id: 2522320,
                house: Some(sms3rs_shared::account::House::ZhiZhi),
                phone: 16601550826,
                organization: None,
                permissions: vec![sms3rs_shared::account::Permission::ManageAccounts],
                registration_time: chrono::Utc::now(),
                registration_ip: Some("127.0.0.1".to_string()),
                password_sha: digest(password.to_string()),
                token_expiration_time: 0,
            },
            tokens: {
                let mut t = crate::account::verify::Tokens::new();
                token = t.new_token(account_id, 0);
                t
            },
            verify: crate::account::UserVerifyVariant::None,
        })
        .await;

    use sms3rs_shared::account::handle::manage::MakeAccountDescriptor;

    let descriptor = MakeAccountDescriptor {
        email: lettre::Address::new("myg", "i.pkuschool.edu.cn").unwrap(),
        name: "Yuguo Ma".to_string(),
        school_id: 114514,
        phone: 1919810,
        house: None,
        organization: Some("PKU".to_string()),
        password: "password".to_string(),
        permissions: vec![
            sms3rs_shared::account::Permission::ManageAccounts,
            sms3rs_shared::account::Permission::OP,
        ],
    };

    let response_json: serde_json::Value = app
        .post("/api/account/manage/create")
        .header("Token", token.to_string())
        .header("AccountId", account_id.to_string())
        .body_json(&descriptor)
        .unwrap()
        .recv_json()
        .await
        .unwrap();

    assert_eq!(
        response_json
            .as_object()
            .unwrap()
            .get("status")
            .unwrap()
            .as_str()
            .unwrap(),
        "success"
    );

    let instance = crate::account::INSTANCE.inner().read().await;
    let a = instance.get(
        *crate::account::INSTANCE
            .index()
            .read()
            .await
            .get(
                &response_json
                    .as_object()
                    .unwrap()
                    .get("account_id")
                    .unwrap()
                    .as_u64()
                    .unwrap(),
            )
            .unwrap(),
    );

    assert!(a.is_some());

    assert!(a
        .unwrap()
        .read()
        .await
        .has_permission(sms3rs_shared::account::Permission::ManageAccounts));

    // Test for permission overflowing
    assert!(!a
        .unwrap()
        .read()
        .await
        .has_permission(sms3rs_shared::account::Permission::OP));
}

#[serial]
#[async_std::test]
async fn view() {
    reset_all().await;

    let mut app = tide::new();
    app.at("/api/account/manage/view")
        .post(crate::account::handle::manage::view_account);

    let account_id = 123456;
    let password = "password123456";

    let token;

    crate::account::INSTANCE
        .push(crate::account::Account::Verified {
            id: account_id,
            attributes: crate::account::UserAttributes {
                email: lettre::Address::new("yujiening2025", "i.pkuschool.edu.cn").unwrap(),
                name: "Jiening Yu".to_string(),
                school_id: 2522320,
                house: Some(sms3rs_shared::account::House::ZhiZhi),
                phone: 16601550826,
                organization: None,
                permissions: vec![sms3rs_shared::account::Permission::ViewAccounts],
                registration_time: chrono::Utc::now(),
                registration_ip: Some("127.0.0.1".to_string()),
                password_sha: digest(password.to_string()),
                token_expiration_time: 0,
            },
            tokens: {
                let mut t = crate::account::verify::Tokens::new();
                token = t.new_token(account_id, 0);
                t
            },
            verify: crate::account::UserVerifyVariant::None,
        })
        .await;

    let test_account_id_0 = 114514;
    let test_account_id_1 = 114513;
    let test_password = "password123456";

    crate::account::INSTANCE
        .push(crate::account::Account::Verified {
            id: test_account_id_0,
            attributes: crate::account::UserAttributes {
                email: lettre::Address::new("myg", "i.pkuschool.edu.cn").unwrap(),
                name: "Yuguo Ma".to_string(),
                school_id: 114514,
                house: None,
                phone: 1919810,
                organization: None,
                permissions: vec![sms3rs_shared::account::Permission::OP],
                registration_time: chrono::Utc::now(),
                registration_ip: Some("127.0.0.1".to_string()),
                password_sha: digest(test_password.to_string()),
                token_expiration_time: 0,
            },
            tokens: crate::account::verify::Tokens::new(),
            verify: crate::account::UserVerifyVariant::None,
        })
        .await;

    crate::account::INSTANCE
        .push(crate::account::Account::Verified {
            id: test_account_id_1,
            attributes: crate::account::UserAttributes {
                email: lettre::Address::new("myg", "i.pkuschool.edu.cn").unwrap(),
                name: "Yuguo Ma".to_string(),
                school_id: 114514,
                house: None,
                phone: 1919810,
                organization: None,
                permissions: vec![],
                registration_time: chrono::Utc::now(),
                registration_ip: Some("127.0.0.1".to_string()),
                password_sha: digest(test_password.to_string()),
                token_expiration_time: 0,
            },
            tokens: crate::account::verify::Tokens::new(),
            verify: crate::account::UserVerifyVariant::None,
        })
        .await;

    let descriptor = sms3rs_shared::account::handle::manage::ViewAccountDescriptor {
        accounts: vec![test_account_id_0, test_account_id_1],
    };

    let response_json: serde_json::Value = app
        .post("/api/account/manage/view")
        .header("Token", token.to_string())
        .header("AccountId", account_id.to_string())
        .body_json(&descriptor)
        .unwrap()
        .recv_json()
        .await
        .unwrap();

    assert_eq!(
        response_json
            .as_object()
            .unwrap()
            .get("status")
            .unwrap()
            .as_str()
            .unwrap(),
        "success"
    );

    let result: Vec<sms3rs_shared::account::handle::manage::ViewAccountResult> = response_json
        .as_object()
        .unwrap()
        .get("results")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|e| serde_json::from_value(e.clone()).unwrap())
        .collect();

    assert!(matches!(
        result[0],
        sms3rs_shared::account::handle::manage::ViewAccountResult::Err { .. }
    ));

    assert!(matches!(
        result[1],
        sms3rs_shared::account::handle::manage::ViewAccountResult::Ok(_)
    ));

    match &result[0] {
        sms3rs_shared::account::handle::manage::ViewAccountResult::Err { id, .. } => {
            assert_eq!(id, &test_account_id_0)
        }
        _ => unreachable!(),
    }

    match &result[1] {
        sms3rs_shared::account::handle::manage::ViewAccountResult::Ok(e) => {
            assert_eq!(e.id, test_account_id_1)
        }
        _ => unreachable!(),
    }
}

#[serial]
#[async_std::test]
async fn modify() {
    reset_all().await;

    let mut app = tide::new();
    app.at("/api/account/manage/modify")
        .post(crate::account::handle::manage::modify_account);

    let account_id = 123456;
    let password = "password123456";

    let token;

    crate::account::INSTANCE
        .push(crate::account::Account::Verified {
            id: account_id,
            attributes: crate::account::UserAttributes {
                email: lettre::Address::new("yujiening2025", "i.pkuschool.edu.cn").unwrap(),
                name: "Jiening Yu".to_string(),
                school_id: 2522320,
                house: Some(sms3rs_shared::account::House::ZhiZhi),
                phone: 16601550826,
                organization: None,
                permissions: vec![sms3rs_shared::account::Permission::ManageAccounts],
                registration_time: chrono::Utc::now(),
                registration_ip: Some("127.0.0.1".to_string()),
                password_sha: digest(password.to_string()),
                token_expiration_time: 0,
            },
            tokens: {
                let mut t = crate::account::verify::Tokens::new();
                token = t.new_token(account_id, 0);
                t
            },
            verify: crate::account::UserVerifyVariant::None,
        })
        .await;

    let test_account_id_0 = 114514;
    let test_account_id_1 = 114513;
    let test_password = "password123456";

    crate::account::INSTANCE
        .push(crate::account::Account::Verified {
            id: test_account_id_0,
            attributes: crate::account::UserAttributes {
                email: lettre::Address::new("myg", "i.pkuschool.edu.cn").unwrap(),
                name: "Yuguo Ma".to_string(),
                school_id: 114514,
                house: None,
                phone: 1919810,
                organization: None,
                permissions: vec![sms3rs_shared::account::Permission::OP],
                registration_time: chrono::Utc::now(),
                registration_ip: Some("127.0.0.1".to_string()),
                password_sha: digest(test_password.to_string()),
                token_expiration_time: 0,
            },
            tokens: crate::account::verify::Tokens::new(),
            verify: crate::account::UserVerifyVariant::None,
        })
        .await;

    crate::account::INSTANCE
        .push(crate::account::Account::Verified {
            id: test_account_id_1,
            attributes: crate::account::UserAttributes {
                email: lettre::Address::new("myg", "i.pkuschool.edu.cn").unwrap(),
                name: "Yuguo Ma".to_string(),
                school_id: 114514,
                house: None,
                phone: 1919810,
                organization: None,
                permissions: vec![],
                registration_time: chrono::Utc::now(),
                registration_ip: Some("127.0.0.1".to_string()),
                password_sha: digest(test_password.to_string()),
                token_expiration_time: 0,
            },
            tokens: crate::account::verify::Tokens::new(),
            verify: crate::account::UserVerifyVariant::None,
        })
        .await;

    use sms3rs_shared::account::{
        handle::manage::{AccountModifyDescriptor, AccountModifyVariant},
        Permission,
    };

    {
        let descriptor = AccountModifyDescriptor {
            account_id: test_account_id_0,
            variants: vec![AccountModifyVariant::Name("Tianyang He".to_string())],
        };

        let response_json: serde_json::Value = app
            .post("/api/account/manage/modify")
            .header("Token", token.to_string())
            .header("AccountId", account_id.to_string())
            .body_json(&descriptor)
            .unwrap()
            .recv_json()
            .await
            .unwrap();

        assert_ne!(
            response_json
                .as_object()
                .unwrap()
                .get("status")
                .unwrap()
                .as_str()
                .unwrap(),
            "success"
        );
    }

    {
        let descriptor = AccountModifyDescriptor {
            account_id: test_account_id_1,
            variants: vec![
                AccountModifyVariant::Email(
                    lettre::Address::new("hetianyang2021", "i.pkuschool.edu.cn").unwrap(),
                ),
                AccountModifyVariant::Name("Tianyang He".to_string()),
                AccountModifyVariant::SchoolId(2100000),
                AccountModifyVariant::Phone(1),
                AccountModifyVariant::House(Some(sms3rs_shared::account::House::ZhengXin)),
                AccountModifyVariant::Organization(Some("SubIT".to_string())),
                AccountModifyVariant::Permission(vec![Permission::ManageAccounts, Permission::OP]),
            ],
        };

        let response_json: serde_json::Value = app
            .post("/api/account/manage/modify")
            .header("Token", token.to_string())
            .header("AccountId", account_id.to_string())
            .body_json(&descriptor)
            .unwrap()
            .recv_json()
            .await
            .unwrap();

        assert_eq!(
            response_json
                .as_object()
                .unwrap()
                .get("status")
                .unwrap()
                .as_str()
                .unwrap(),
            "success"
        );

        let am = crate::account::INSTANCE.inner().read().await;
        let a = am
            .get(
                *crate::account::INSTANCE
                    .index()
                    .read()
                    .await
                    .get(&test_account_id_1)
                    .unwrap(),
            )
            .unwrap()
            .read()
            .await;

        assert!(matches!(
            a.deref(),
            crate::account::Account::Verified { .. }
        ));

        match a.deref() {
            crate::account::Account::Verified { id, attributes, .. } => {
                assert_eq!(*id, test_account_id_1);

                assert_eq!(
                    attributes.email,
                    lettre::Address::new("hetianyang2021", "i.pkuschool.edu.cn").unwrap()
                );
                assert_eq!(attributes.name, "Tianyang He");
                assert_eq!(attributes.school_id, 2100000);
                assert_eq!(attributes.phone, 1);
                assert_eq!(
                    attributes.house,
                    Some(sms3rs_shared::account::House::ZhengXin)
                );
                assert_eq!(attributes.organization, Some("SubIT".to_string()));
                assert_eq!(attributes.permissions, vec![Permission::ManageAccounts]);
            }
            _ => unreachable!(),
        }
    }
}