pub mod lists;

// TODO in the futrure maybe make evrey model impl the traits here.
trait _DbModel {
    fn new();
    fn jsonify(self);
    fn clone_to_jsonify(&self);
}

trait _DbModelAsJson {}

trait _NewDbModel {}
