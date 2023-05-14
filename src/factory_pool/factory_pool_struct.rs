/// this struct stores all factories used to produce images
/// it contaisn a pool out of factories and keeps track of thier id
/// there is open channel so the factory send a nummber so the
/// factory pools asks for the value
/// if the factory cant give the value it just get
/// it just get thrown over and ignort

struct FactoryPool {}
impl FactoryPool {
    pub async fn new() -> Self {
        Self {}
    }
}
