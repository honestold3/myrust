//#[macro_use]
#[cfg_attr(test, macro_use)]
extern crate adv_proj;

use adv_proj::af::unsafes;
use adv_proj::af::traits;
#[macro_use]
use adv_proj::af::types;
use adv_proj::af::closures;


#[macro_use]
macro_rules! say_hello{
   ()=>(
       println!("Hello");
   )
}

fn main() {
    //unsafes::kankan();
    //unsafes::kankan1();
    //unsafes::kankan2();
    //unsafes::kankan3();
    //unsafes::kankan4();
    //unsafes::kankan5();
    //unsafes::kankan6();

    //traits::kankan();
    //traits::kankan1();
    //traits::kankan1();
    //traits::kankan2();
    //traits::kankan3();
    //traits::kankan4();
    //traits::kankan5();

    //types::kankan();
    //closures::kankan();
    //closures::kankan1();
    //closures::kankan3();

//    closures::anonymous_fnonce();
//    closures::anonymous_fnonce_callback();
//    closures::anonymous_fnmut();
//    closures::anonymous_fnmut_callback();
//    closures::anonymous_fn();
//    closures::anonymous_fn_callback();

    types::kankan1();

    say_hello!();
    //multiply_add!(1,2,3);
}




