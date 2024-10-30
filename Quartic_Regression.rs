use calamine::{open_workbook, Reader, Xlsx};
use ndarray::{concatenate, prelude::*};
use nalgebra::{DMatrix, DVector};
use std::error::Error;

#[allow(non_snake_case)]
fn main() -> Result<(), Box<dyn Error>> {
    // เปิดไฟล์ Excel
    let mut workbook: Xlsx<_> = open_workbook("D:\\data.xlsx")?;
    
    // อ่านแผ่นงาน (sheet) ที่ต้องการ
    let range = workbook.worksheet_range("Sheet1")
        .ok_or("Cannot find 'Sheet1'")??;

    // สร้างเวกเตอร์สำหรับเก็บข้อมูล
    let mut x_data: Vec<f64> = Vec::new();
    let mut y_data: Vec<f64> = Vec::new();

    // อ่านข้อมูลจากแผ่นงาน
    for row in range.rows() {
        if let (Some(x1), Some(x2), Some(x3), Some(y)) = (row.get(0), row.get(1), row.get(2), row.get(3)) {
            let P = x1.get_float().unwrap_or(0.0);
            let T = x2.get_float().unwrap_or(0.0);
            let C = x3.get_float().unwrap_or(0.0);
            x_data.push(1.0);
            x_data.push(P);
            x_data.push(T);
            x_data.push(C);
            x_data.push(P.powi(2));
            x_data.push(P*T);
            x_data.push(P*C);
            x_data.push(T.powi(2));
            x_data.push(T*C);
            x_data.push(C.powi(2));
            x_data.push(P.powi(3));
            x_data.push(P.powi(2)*T);
            x_data.push(P.powi(2)*C);
            x_data.push(P*T.powi(2));
            x_data.push(P*T*C);
            x_data.push(P*C.powi(2));
            x_data.push(T.powi(3));
            x_data.push(T.powi(2)*C);
            x_data.push(T*C.powi(2));
            x_data.push(C.powi(3));
            x_data.push(P.powi(4));
            x_data.push(P.powi(3)*T);
            x_data.push(P.powi(3)*C);
            x_data.push(P.powi(2)*T.powi(2));
            x_data.push(P.powi(2)*T*C);
            x_data.push(P.powi(2)*C.powi(2));
            x_data.push(P*T.powi(3));
            x_data.push(P*T.powi(2)*C);
            x_data.push(P*T*C.powi(2));
            x_data.push(P*C.powi(3));
            x_data.push(T.powi(4));
            x_data.push(T.powi(3)*C);
            x_data.push(T.powi(2)*C.powi(2));
            x_data.push(T*C.powi(3));
            x_data.push(C.powi(4));
            y_data.push(y.get_float().unwrap_or(0.0));
        }
    }
    // แปลงข้อมูลเป็น ndarray
    let x: Array2<f64> = Array2::from_shape_vec((y_data.len(), 35), x_data)?;
    let y: Array1<f64> = Array1::from(y_data);

    // เพิ่มคอลัมน์ของ 1s สำหรับ bias term
    let ones: Array2<f64> = Array::ones((x.nrows(), 1));
    let x: Array2<f64> = concatenate![Axis(1), ones, x];

    // แปลง ndarray เป็น Vec โดยใช้ iter
    let x_vec: Vec<f64> = x.iter().cloned().collect();
    let y_vec: Vec<f64> = y.iter().cloned().collect();

    // แปลง Vec เป็น DMatrix ของ nalgebra
    let x_matrix: DMatrix<f64> = DMatrix::from_row_slice(x.nrows(), x.ncols(), &x_vec);
    let y_vector: DVector<f64> = DVector::from_row_slice(&y_vec);

    // เพิ่ม regularization term
    let lambda: f64 = 1.0; // ลองเพิ่มค่า lambda ให้ใหญ่ขึ้น
    let identity: DMatrix<f64> = DMatrix::identity(x_matrix.ncols(), x_matrix.ncols());
    let xtx: DMatrix<f64> = x_matrix.transpose() * &x_matrix + lambda * identity;

    // คำนวณค่าสัมประสิทธิ์ (coefficients) โดยใช้สมการ (X^T * X + λI)^-1 * X^T * y
    let coefficients: DVector<f64> = match xtx.try_inverse() {
        Some(xtx_inv) => {
            let xty: DVector<f64> = x_matrix.transpose() * &y_vector;
            xtx_inv * xty
        }
        None => {
            println!("Matrix is not invertible even with regularization");
            return Ok(());
        }
    };

    //ข้อมูลใหม่สำหรับการทำนาย
    let new_data_x: Array2<f64> = x;
    let new_data_y:Array1<f64> = y;


    //แปลงข้อมูลใหม่เป็น DMatrix
    let new_data_vec: Vec<f64> = new_data_x.iter().cloned().collect();
    let new_data_matrix: DMatrix<f64> = DMatrix::from_row_slice(new_data_x.nrows(), new_data_x.ncols(), &new_data_vec);

    //ทำนายค่าตัวแปรตามสำหรับข้อมูลใหม่
    let predictions: DVector<f64> = new_data_matrix * &coefficients;

    //แสดงผลการทำนาย
    for (i, prediction) in predictions.iter().enumerate() {
        println!("{} Predicted : {:.15} Actual : {:.15} => Residual : {:.15} Error : {:.5} %",
        i+1,
        prediction,
        new_data_y[i],
        (prediction-new_data_y[i]).abs(),
        (((prediction-new_data_y[i])/new_data_y[i]) * 100.0).abs());
    }

    // แสดงค่าสัมประสิทธิ์ทั้งหมด
    for (i, coeff) in coefficients.iter().enumerate() {
        println!("Coefficient {}: {}", i, coeff);
    }

    // ทำนายค่าตัวแปรตามสำหรับข้อมูลเดิม
    let y_pred: DVector<f64> = x_matrix * &coefficients;

    // คำนวณค่า MSE
    let mse: f64 = y_vector.iter().zip(y_pred.iter()).map(|(y, y_hat)| (y - y_hat).powi(2)).sum::<f64>() / y_vector.len() as f64;
    println!("MSE: {}", mse);

    // คำนวณค่า R-Square
    let y_mean: f64 = y_vector.mean();
    let ss_tot: f64 = y_vector.iter().map(|y| (y - y_mean).powi(2)).sum();
    let ss_res: f64 = y_vector.iter().zip(y_pred.iter()).map(|(y, y_hat)| (y - y_hat).powi(2)).sum();
    let r_square: f64 = 1.0 - (ss_res / ss_tot);
    println!("R-Square: {}", r_square);
    
    // สร้างสมการจากค่าสัมประสิทธิ์
    let equation = format!(
        "y = {:.15} +
            ({:.15}*1) +
            ({:.15}*P) +
            ({:.15}*T) +
            ({:.15}*C) +
            ({:.15}*P^2) +
            ({:.15}*P*T) +
            ({:.15}*P*C) +
            ({:.15}*T^2) +
            ({:.15}*T*C) +
            ({:.15}*C^2) +
            ({:.15}*P^3) +
            ({:.15}*P^2*T) +
            ({:.15}*P^2*C) +
            ({:.15}*P*T^2) +
            ({:.15}*P*T*C) +
            ({:.15}*P*C^2) +
            ({:.15}*T^3) +
            ({:.15}*T^2*C) +
            ({:.15}*T*C^2) +
            ({:.15}*C^3) +
            ({:.15}*P^4) +
            ({:.15}*P^3*T) +
            ({:.15}*P^3*C) +
            ({:.15}*P^2*(T^2)) +
            ({:.15}*(P^2)*T*C) +
            ({:.15}*(P^2)*(C^2)) +
            ({:.15}*P*(T^3)) +
            ({:.15}*P*(T^2)*C) +
            ({:.15}*P*T*(C^2)) +
            ({:.15}*P*(C^3)) +
            ({:.15}*(T^4) +
            ({:.15}*(T^3)*C +
            ({:.15}*(T^2)*(C^2)) +
            ({:.15}*T*(C^3)) +
            ({:.15}*C^4)",
        coefficients[0],
        coefficients[1],
        coefficients[2],
        coefficients[3], 
        coefficients[4], 
        coefficients[5], 
        coefficients[6], 
        coefficients[7], 
        coefficients[8], 
        coefficients[9], 
        coefficients[10], 
        coefficients[11], 
        coefficients[12],
        coefficients[13],
        coefficients[14],
        coefficients[15],
        coefficients[16],
        coefficients[17],
        coefficients[18],
        coefficients[19],
        coefficients[20],
        coefficients[21],
        coefficients[22],
        coefficients[23],
        coefficients[24],
        coefficients[25],
        coefficients[26],
        coefficients[27],
        coefficients[28],
        coefficients[29],
        coefficients[30],
        coefficients[31],
        coefficients[32],
        coefficients[33],
        coefficients[34],
        coefficients[35],
    );
    println!("Equation: {}", equation);

    Ok(())
}
