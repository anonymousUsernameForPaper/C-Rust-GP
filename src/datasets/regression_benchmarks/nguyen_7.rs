use rand::distributions::{Distribution, Uniform};



fn make_label(inputs: &Vec<Vec<f32>>) -> Vec<Vec<f32>>{
    let mut labels: Vec<f32> = vec![];
    for d in inputs {
        labels.push(d[0].ln_1p()  + d[0].powf(2.0).ln_1p());
    }

    vec![labels]
}


pub fn get_dataset() -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let mut data = vec![];

    let between = Uniform::new(0.0, 2.0);
    let mut rng = rand::thread_rng();

    for _ in 0..20 {
        let mut elem: Vec<f32> = vec![];
        elem.push(between.sample(&mut rng));

        data.push(elem);
    }


    let labels = make_label(&data);

    (data, labels)
}

pub fn get_eval_dataset() -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    get_dataset()
}
