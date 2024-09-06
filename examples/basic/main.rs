use sheesh::{
    auth_token::AuthTokenGenerator,
    id::DefaultIdGenerator,
    session::SessionManager,
    token::DefaultHashGenerator,
    user::{Group, PrivateUserMeta, PublicUserMeta, Role, User, UserManager},
};

enum Roles {
    Admin,
}

impl Role for Roles {}

struct MyPublicUserMetadata {}
impl PublicUserMeta for MyPublicUserMetadata {}

struct MyPrivateUserMetadata {}
impl PrivateUserMeta for MyPrivateUserMetadata {}

struct SomeGroup {}
impl Group for SomeGroup {}

type MyUser = User<Roles, SomeGroup, MyPublicUserMetadata, MyPrivateUserMetadata>;

fn main() {
    let id_generator = DefaultIdGenerator::init();
    let hash_generator = DefaultHashGenerator::init();

    let user_manager = UserManager::init(id_generator);
    let token_generator = AuthTokenGenerator::init(id_generator, hash_generator, 30);
    let session_manager = SessionManager::init(id_generator, token_generator);

    let mut user: MyUser = user_manager.new_user(
        "test".to_string(),
        Roles::Admin,
        MyPublicUserMetadata {},
        MyPrivateUserMetadata {},
    );

    let public = MyPublicUserMetadata {};
    let private = MyPrivateUserMetadata {};

    user.set_public_data(public);
    user.set_private_data(private);

    let (mut session, mut token) = session_manager.new_session(user.public().id()).unwrap();

    token.invalidate();

    let mut new_token = session_manager.refresh_session_token(&mut session).unwrap();

    let is_valid_token = new_token.is_valid();
}
