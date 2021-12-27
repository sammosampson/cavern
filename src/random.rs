use rand::Rng;
use rand::distributions::uniform::{
    SampleUniform,
    SampleRange
};


pub fn random_number<R, T>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>
{
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}