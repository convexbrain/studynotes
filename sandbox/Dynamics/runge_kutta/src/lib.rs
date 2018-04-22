pub type FP = f64;

#[derive(Debug)]
pub struct RungeKutta {
    q: Vec<FP>,
    k: Vec<FP>,
    dq: Vec<FP>,
    tq: Vec<FP>
}

impl RungeKutta {
    pub fn new(n: usize) -> RungeKutta {
        RungeKutta {
            q: vec![0.0; n],
            k: vec![0.0; n],
            dq: vec![0.0; n],
            tq: vec![0.0; n]
        }
    }
    
    pub fn init_val<F>(&mut self, init_eq: F)
        where F: Fn(&mut [FP]) {
    
        init_eq(&mut self.q);
    }

    pub fn step<F>(&mut self, dt: FP, diff_eq: F)
        where F: Fn(&mut [FP], &[FP]) {
        
        let q: &mut [FP] = &mut self.q;
        let k: &mut [FP] = &mut self.k;
        let dq: &mut [FP] = &mut self.dq;
        let tq: &mut [FP] = &mut self.tq;

        diff_eq(k, q);

        for (i, k1) in k.iter().enumerate() {
            let k1dt = *k1 * dt;
            tq[i] = q[i] + 0.5 * k1dt;
            dq[i] = k1dt;
        }

        diff_eq(k, tq);

        for (i, k2) in k.iter().enumerate() {
            let k2dt = *k2 * dt;
            tq[i] = q[i] + 0.5 * k2dt;
            dq[i] += 2.0 * k2dt;
        }

        diff_eq(k, tq);

        for (i, k3) in k.iter().enumerate() {
            let k3dt = *k3 * dt;
            tq[i] = q[i] + k3dt;
            dq[i] += 2.0 * k3dt;
        }

        diff_eq(k, tq);

        for (i, k4) in k.iter().enumerate() {
            let k4dt = *k4 * dt;
            q[i] += (1.0 / 6.0) * (dq[i] + k4dt);
        }
    }

    pub fn get_val(&self) -> &[FP] {
        &self.q
    }
}

#[test]
fn test() {
    const TPI: FP = 2.0 * std::f64::consts::PI;

    let mut sim = RungeKutta::new(3);
    sim.init_val(|q| {q[1] = 1.0});

    for _ in 0..32 {
        for _ in 0..32 {
            sim.step(1.0 / 1024.0, |dqdt, q| {
                // 0:t, 1:x, 2:dxdt
                dqdt[0] = 1.0;
                dqdt[1] = q[2];
                dqdt[2] = - TPI * TPI * q[1];
            });
            let c = (TPI * sim.get_val()[0]).cos();
            let diff = sim.get_val()[1] - c;
            let eps = 1.0e-10;
            println!("{} <? {}", diff, eps);
            assert!(diff.abs() < eps);
        }
    }
    assert_eq!(1.0, sim.get_val()[0]);
}
