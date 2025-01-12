// https://mp.weixin.qq.com/s/wnmdbF9hYFq55veusS7Fdg 使用Rust从零构建神经网络：性能与精度的完美结合
// 构建一个简单的前馈神经网络，用来学习经典的XOR函数——一个测试网络是否能模拟非线性关系的经典问题。通过这个过程，
// 我们不仅会构建模型本身，还将探索机器学习中的关键概念，如反向传播、激活函数和损失计算。
#[cfg(test)]
mod tests {
    use rand::Rng;

    // 定义神经网络单层结构
    struct Layer {
        weights: Vec<Vec<f64>>, // weights（权重）
        biases: Vec<f64>,       // biases（偏置）
                                // 每个神经元的权重连接着上一层的所有神经元，而偏置则独立调整每个神经元的输出。
    }

    impl Layer {
        fn new(input_size: usize, output_size: usize) -> Layer {
            let mut rng = rand::thread_rng();

            // 用随机值初始化权重和偏置
            let weights = (0..output_size)
                .map(|_| (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
                .collect();

            let biases = (0..output_size).map(|_| rng.gen_range(-1.0..1.0)).collect();

            Layer { weights, biases }
        }
    }

    // Sigmoid 激活函数
    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    impl Layer {
        // forward方法通过计算加权和并加上偏置来得到每个神经元的输出，然后通过Sigmoid激活函数进行非线性变换。
        // 接下来，我们需要定义一个NeuralNetwork结构体来组合多个层。
        fn forward(&self, input: &[f64]) -> Vec<f64> {
            self.weights
                .iter()
                .enumerate()
                .map(|(i, neuron_weights)| {
                    let sum: f64 = neuron_weights
                        .iter()
                        .zip(input.iter())
                        .map(|(w, i)| w * i)
                        .sum();
                    sigmoid(sum + self.biases[i])
                })
                .collect()
        }
    }

    // 损失是衡量网络预测与目标值之间差距的标准。定义均方误差（MSE）计算函数：计算预测值和实际值之间差异的平方和，并求平均值。
    fn mean_squared_error(predicted: &[f64], actual: &[f64]) -> f64 {
        predicted
            .iter()
            .zip(actual.iter())
            .map(|(p, a)| (p - a).powi(2))
            .sum::<f64>()
            / predicted.len() as f64
    }

    // 定义神经网络结构
    struct NeuralNetwork {
        layers: Vec<Layer>,
    }

    impl NeuralNetwork {
        fn new(layer_sizes: &[usize]) -> NeuralNetwork {
            let layers = layer_sizes
                .windows(2)
                .map(|w| Layer::new(w[0], w[1]))
                .collect();
            NeuralNetwork { layers }
        }

        // NeuralNetwork的forward方法会将输入依次通过每一层处理，直到输出最终结果。
        fn forward(&self, input: &[f64]) -> Vec<f64> {
            self.layers
                .iter()
                .fold(input.to_vec(), |acc, layer| layer.forward(&acc))
        }
    }

    impl NeuralNetwork {
        fn backward(&mut self, inputs: &[f64], target: &[f64], learning_rate: f64) {
            let mut layer_inputs = vec![inputs.to_vec()];
            let mut current_input = inputs.to_vec();

            for layer in &self.layers {
                current_input = layer.forward(&current_input);
                layer_inputs.push(current_input.clone());
            }

            let error = layer_inputs
                .last()
                .unwrap()
                .iter()
                .zip(target.iter())
                .map(|(o, t)| o - t)
                .collect::<Vec<_>>();

            let mut current_error = error;

            for (layer, inputs) in self
                .layers
                .iter_mut()
                .rev()
                .zip(layer_inputs.iter().rev().skip(1))
            {
                current_error = layer.backward(inputs, &current_error, learning_rate);
            }
        }
    }
    // 反向传播是通过调整权重和偏置来最小化误差的过程。具体步骤包括：计算误差、计算梯度并更新权重和偏置。
    impl Layer {
        // backward方法会根据误差计算每一层的梯度，并更新权重和偏置。
        // 反向传播是通过调整权重和偏置来最小化误差的过程。具体步骤包括：计算误差、计算梯度并更新权重和偏置。backward方法会根据误差计算每一层的梯度，并更新权重和偏置。
        fn backward(&mut self, input: &[f64], error: &[f64], learning_rate: f64) -> Vec<f64> {
            let mut input_error = vec![0.0; input.len()];

            for (i, neuron_weights) in self.weights.iter_mut().enumerate() {
                for (j, weight) in neuron_weights.iter_mut().enumerate() {
                    input_error[j] += *weight * error[i];
                    *weight -= learning_rate * error[i] * input[j];
                }
                self.biases[i] -= learning_rate * error[i];
            }

            input_error
        }
    }

    // 创建一个训练循环，不断更新权重和偏置，直到网络学会XOR函数。在这个训练循环中，网络通过不断调整权重和偏置来最小化每个数据点的损失，直到学会XOR函数。
    #[test]
    fn neural_network_test() {
        let mut network = NeuralNetwork::new(&[2, 3, 1]);
        let data = vec![
            (vec![0.0, 0.0], vec![0.0]),
            (vec![0.0, 1.0], vec![1.0]),
            (vec![1.0, 0.0], vec![1.0]),
            (vec![1.0, 1.0], vec![0.0]),
        ];

        let learning_rate = 0.1;
        for epoch in 0..5000 {
            let mut loss = 0.0;

            for (input, target) in &data {
                let prediction = network.forward(input);
                loss += mean_squared_error(&prediction, target);
                network.backward(input, target, learning_rate);
            }

            if epoch % 1000 == 0 {
                println!("Epoch {}: Loss = {}", epoch, loss / data.len() as f64);
            }
        }
    }
}
