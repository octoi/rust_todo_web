use crate::model::{self, init_db};

use super::{UserMac, UserPatch};

#[tokio::test]
async fn model_user_register() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let data_fx = UserPatch {
        username: String::from("john"),
        password: String::from("123"),
    };

    // -- ACTION
    let user = UserMac::register(&db, data_fx.clone()).await?;

    // -- CHECK
    assert_eq!(data_fx.username, user.username);
    assert_eq!(data_fx.password, user.password);

    Ok(())
}

#[tokio::test]
async fn model_user_login() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let data_fx = UserPatch {
        username: String::from("john"),
        password: String::from("123"),
    };

    UserMac::register(&db, data_fx.clone()).await?;

    // -- ACTION
    let user = UserMac::login(&db, data_fx.clone()).await?;

    // -- CHECK
    assert_eq!(data_fx.username, user.username);
    assert_eq!(data_fx.password, user.password);

    Ok(())
}

#[tokio::test]
async fn model_user_register_fail() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let data_fx = UserPatch {
        username: String::from("john"),
        password: String::from("123"),
    };

    UserMac::register(&db, data_fx.clone()).await?;

    // -- ACTION
    let user = UserMac::register(&db, data_fx.clone()).await;

    match user {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(model::Error::UserNameIsAlreadyTaken(username)) => {
            assert_eq!("john", username);
        }
        other_error => assert!(false, "Wrong Error {:?}", other_error),
    }

    Ok(())
}

#[tokio::test]
async fn model_user_login_password_fail() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let data_register = UserPatch {
        username: String::from("john"),
        password: String::from("123"),
    };

    let data_login = UserPatch {
        username: String::from("john"),
        password: String::from("321"),
    };

    UserMac::register(&db, data_register.clone()).await?;

    // -- ACTION
    let result = UserMac::login(&db, data_login.clone()).await;

    // -- CHECK
    match result {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(model::Error::InvalidPassword(username)) => {
            assert_eq!("john", username);
        }
        other_error => assert!(false, "Wrong Error {:?}", other_error),
    }

    Ok(())
}

#[tokio::test]
async fn model_user_login_username_fail() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;

    let data_register = UserPatch {
        username: String::from("john"),
        password: String::from("123"),
    };

    let data_login = UserPatch {
        username: String::from("james"),
        password: String::from("123"),
    };

    UserMac::register(&db, data_register.clone()).await?;

    // -- ACTION
    let result = UserMac::login(&db, data_login.clone()).await;

    // -- CHECK
    match result {
        Ok(_) => assert!(false, "Should not succeed"),
        Err(model::Error::EntityNotFound(typ, data)) => {
            assert_eq!("person", typ);
            assert_eq!("james", data);
        }
        other_error => assert!(false, "Wrong Error {:?}", other_error),
    }

    Ok(())
}
