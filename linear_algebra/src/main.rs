//use std::io;

fn main() {
    let expr: String = String::from("2+3*4");
    let rpm: String = convert_to_rpm(expr.clone());
    println!("{} -> {}", expr, rpm);
}

fn convert_to_rpm(mut expr: String) -> String {
    // 余分な括弧を削除する
    expr = delete_extra_brackets(expr);

    // これ以上計算できるものがない場合は終了
    if expr.len() <= 1 {
        return expr;
    }
    // 文字列全体が数字である場合は終了
    match expr.parse::<i32>() {
        Ok(_) => { 
            return expr;
        }
        Err(_) => {
            // 演算子を探す関数の呼び出し
            let mut result: String = find_add_or_sub(expr.clone());
            if result != String::from("Not Found Add or Sub") {
                return result;
            }

            result = find_mul(expr.clone());
            if result != String::from("") {
                return result;
            }

            return String::from("Error");
        }
    }
}

fn delete_extra_brackets(expr: String) -> String {
    if expr.starts_with("(") || expr.ends_with(")") {
        let mut deep: i32 = 0;
        while let Some(c) = expr[1..expr.len() - 2].chars().next() {
            if c == '(' {
                deep += 1;
            } else if c == ')' {
                deep -= 1;
            }
            if deep == 0 {
                return expr;
            }
        }
        return expr[1..expr.len() - 2].to_string();
    }
    return expr.clone();
}

fn find_add_or_sub(expr: String) -> String {
    let mut deep: i32 = 0;
    for (index, c) in expr.chars().enumerate() {
        if c == '(' {
            deep += 1;
        } else if c == ')' {
            deep -= 1;
        }
        if deep > 0 {
            continue;
        }

        if ( index == 0 ) && (c == '+' || c == '-') {
            let inial_expr: String = expr.clone()[0..index].to_string();
            let rest_expr: String = expr.clone()[index+1..].to_string();
            return  format!("{},{},{},",inial_expr,rest_expr,c);
        }
    }
    return String::from("Not Found Add or Sub");
}

fn find_mul(expr: String) -> String {
    let mut deep: i32 = 0;
    let mut is_decimal = false;
    for (index, c) in expr.chars().enumerate() {
        if c == '(' {
            deep += 1;
        } else if c == ')' {
            deep -= 1;
            if deep == 0 {
                return make_mul_expr(expr, index);
            }
        }
        if deep > 0 {
            continue;
        }

        if c == '*' {
            let inial_expr: String = expr.clone()[0..index].to_string();
            let rest_expr: String = expr.clone()[index+1..].to_string();
            return  format!("{},{},*,",inial_expr,rest_expr);
        }
        
        // 数字が連続している場合は、最後の数字までを探索する
        match expr[..index].parse::<i32>() {
            Ok(_) => {
                // 数値に終わりを検出するためのフラグ
                is_decimal = true;
                continue;
            },
            Err(_) => {
                if is_decimal {
                    // expr[0..ct]ではexpr[ct]は含まれない
                    return make_mul_expr(expr, index);
                }
                else {
                    return make_mul_expr(expr, index);
                }
            }
        }
        // 文字式や括弧が連続している場所を検出する
    }
    return String::from("Error");
}

// 掛け算の式を組み立てる関数
fn make_mul_expr(expr: String, rest_init_ct: usize) -> String {
    let inial_expr: String = expr.clone()[0..rest_init_ct+1].to_string();
    let rest_expr: String = expr.clone()[rest_init_ct..].to_string();
    return  format!("{},{},*,",inial_expr,rest_expr);
}