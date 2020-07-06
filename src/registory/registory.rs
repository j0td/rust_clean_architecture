use diesel::mysql::MysqlConnection;

struct Registory {
  conn: MysqlConnection,
  controller: Controller,
  service: Service,
  presenter: Presenter,
}

trait Registory {
  fn new_user_controller(&self, conn: MysqlConnection, service: Service, presenter: Presenter) -> UserController;
}

struct UserRegistory {
}

impl UserRegistory for UserRegistory {
  fn new_user_controller(&self, service: UserService, presenter: UserPresenter) -> Controller {
    
  }
  
  fn new_service(&self) -> Service {

  }

  fn new_repository(&self) -> Repository {

  }

  fn new_presenter(&self) -> Presenter {

  }
  
  fn new(&self) -> Registory {
    Registory
  }
}

// まちがった、、
// イメージこんな感じです rustの書き方わからん
pub fn new_registory(conn) {
  Registory{ conn: conn }
}

fn new_user_controler() -> controller:: user_controller {
  controller::new_user_controller(conn, i::new_user_service(), i::new_user_presenter())
}

// コピペですがw ここで依存関係全部組み立てるイメージです
fn new_user_service() -> service.user_service {
	return service.NewChatService(i.NewChatRepository(), i.NewQueueService())
}

func (i *interactor) NewChatRepository() repository.ChatRepository {
	return repository.NewChatRepository()
}

func (i *interactor) NewChatPresenter() presenter.ChatPresenter {
	return presenter.NewChatPresenter()
}

// interface registory {
//    NewRegistory returnはResityroyのstruct 
// }
// あ、newRegistoryの引数にconnが必要かも
// 実装は Registoryのconnに引数のconnを渡して返すような。
// 
// 
