pub mod stash_receiver;
pub mod build_calculator;
pub mod builds_repository;
pub mod item_repository;

#[macro_export]
macro_rules! define_repo_method {
    ($actor:ty, $msg_t:ty, $res:ty, $call:tt, $($args:tt),*) => {
        impl Handler<$msg_t> for $actor {
            type Result = $res;

            fn handle(&mut self, _msg: $msg_t, _: &mut Self::Context) -> Self::Result {
                self.repo.$call(
                    $( &_msg.$args, )*
                )
            }
        }
    }
}