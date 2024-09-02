use sheesh::{
    auth_token::AuthTokenGenerator,
    id::DefaultIdGenerator,
    session::SessionManager,
    token::DefaultHashGenerator,
    user::{Group, PrivateUserMeta, PublicUserMeta, Role, User, UserManager},
};

enum Roles {
    Admin,
    User,
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

    // this is gross...
    let token_generator = AuthTokenGenerator::<DefaultIdGenerator, DefaultHashGenerator>::default();

    let user_manager = UserManager::init(DefaultIdGenerator::init());
    let session_manager = SessionManager::init(id_generator, token_generator);

    let user: MyUser = user_manager.new_user(
        "test".to_string(),
        Roles::Admin,
        MyPublicUserMetadata {},
        MyPrivateUserMetadata {},
    );

    session_manager.new_session(user.id);
}
