pub struct Network {
    pub layers: Vec<Matrix>,
    pub biases: Vec<Matrix>,
    pub weights: Vec<Matrix>,
    pub data: Vec<Matrix>,
}

impl Network {
    pub fn new(layers: Vec<usize>) -> Network {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::rand_matrix(layers[i + 1], layers[i]));
            biases.push(Matrix::rand_matrix(layers[i + 1], 1));
        }

        Network {
            layers,
            biases,
            weights,
            data: vec![],
        }
    }

    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len != self.layers[0] {
            panic!("Input length must match the number of neurons in the input layer");
        }
        let mut current: Matrix = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];

        for i in 0..self.layers.len() - 1 {
            current = self.weights[i].multiply(&current).add(&self.biases[i]);
        }
    }
}
