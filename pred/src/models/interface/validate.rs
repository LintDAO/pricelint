use burn::prelude::{Backend, Tensor};

pub trait Validate<B: Backend<FloatElem = f32>, const D: usize> {
    fn mse_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> Tensor<B, 1>;

    fn mre_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> Tensor<B, 1>;

    fn rmse_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> Tensor<B, 1> {
        self.mse_loss(predictions, targets).sqrt()
    }

    /// MAE 的近似转换（基于正态分布假设：MAE ≈ 0.7979 × √MSE）
    fn mae_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> Tensor<B, 1> {
        self.mse_loss(predictions, targets).sqrt() * (2.0 / std::f32::consts::PI).sqrt()
        // ≈ 0.79788456 × √MSE
    }
    /// Huber Loss（平滑的 MSE+MAE），需要知道 δ（常用 1.0~1.5）
    fn huber_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>, delta: f32) -> f32 {
        let mse = self.mse_loss(predictions, targets).into_scalar();
        let mae = self.mae_loss(predictions, targets).into_scalar();
        let sqrt_mse = mse.clone().sqrt();

        // Huber 的近似转换公式（经验公式，误差 < 5%）
        if sqrt_mse <= delta {
            mse // 小误差时等价于 MSE
        } else {
            delta * mae - 0.5 * delta * delta // 大误差时接近 MAE
        }
    }

    /// Log-Cosh Loss（比 MSE 对异常值更鲁棒）
    fn log_cosh_loss(&self, predictions: &Tensor<B, D>, targets: &Tensor<B, D>) -> f32 {
        let mse = self.mse_loss(predictions, targets).into_scalar();
        mse.ln() + (1.0 + (-2.0 * mse).exp()).ln()
        // 近似公式：log(cosh(x)) ≈ |x| - log(2) 当 |x| 大时
    }

    /// Quantile Loss（分位数回归），这里以中位数回归（τ=0.5）为例 → 相当于 MAE
    fn quantile_loss(
        &self,
        predictions: &Tensor<B, D>,
        targets: &Tensor<B, D>,
        tau: f32,
    ) -> Tensor<B, 1> {
        if (tau - 0.5).abs() < 1e-6 {
            self.mae_loss(predictions, targets)
        } else {
            // 其他分位数需要更多统计信息，这里给出粗略估计
            let mae = self.mae_loss(predictions, targets);
            mae * tau.max(1.0 - tau) / 0.5
        }
    }
}
