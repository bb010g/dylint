use std::{
    env::{var, VarError},
    fs::File,
    io::{Error, ErrorKind, Read},
    result::Result,
};

fn main() {}

fn deref_assign_before_ok_return(flag: &mut bool) -> Result<(), VarError> {
    *flag = true;
    Ok(())
}

fn call_with_mut_ref_before_ok_return(xs: &mut Vec<u32>) -> Result<(), VarError> {
    xs.push(0);
    Ok(())
}

fn deref_assign_before_err_return(flag: &mut bool) -> Result<(), VarError> {
    *flag = true;
    Err(VarError::NotPresent)
}

fn call_with_mut_ref_before_err_return(xs: &mut Vec<u32>) -> Result<(), VarError> {
    xs.push(0);
    Err(VarError::NotPresent)
}

fn deref_assign_before_error_switch(flag: &mut bool) -> Result<(), VarError> {
    *flag = true;
    let _ = var("X")?;
    Ok(())
}

fn call_with_mut_ref_before_error_switch(xs: &mut Vec<u32>) -> Result<(), VarError> {
    xs.push(0);
    let _ = var("X")?;
    Ok(())
}

fn deref_assign_after_ok_assign(flag: &mut bool) -> Result<(), VarError> {
    let result = Ok(());
    *flag = true;
    result
}

fn call_with_mut_ref_after_ok_assign(xs: &mut Vec<u32>) -> Result<(), VarError> {
    let result = Ok(());
    xs.push(0);
    result
}

fn deref_assign_after_err_assign(flag: &mut bool) -> Result<(), VarError> {
    let result = Err(VarError::NotPresent);
    *flag = true;
    result
}

fn call_with_mut_ref_after_err_assign(xs: &mut Vec<u32>) -> Result<(), VarError> {
    let result = Err(VarError::NotPresent);
    xs.push(0);
    result
}

fn deref_assign_in_ok_arm(flag: &mut bool) -> Result<(), VarError> {
    let result = var("X").map(|_| {});
    match result {
        Ok(_) => {
            *flag = true;
        }
        Err(_) => {}
    }
    result
}

fn call_with_mut_ref_in_ok_arm(xs: &mut Vec<u32>) -> Result<(), VarError> {
    let result = var("X").map(|_| {});
    match result {
        Ok(_) => {
            xs.push(0);
        }
        Err(_) => {}
    }
    result
}

fn deref_assign_in_err_arm(flag: &mut bool) -> Result<(), VarError> {
    let result = var("X").map(|_| {});
    match result {
        Ok(_) => {}
        Err(_) => {
            *flag = true;
        }
    }
    result
}

fn call_with_mut_ref_in_err_arm(xs: &mut Vec<u32>) -> Result<(), VarError> {
    let result = var("X").map(|_| {});
    match result {
        Ok(_) => {}
        Err(_) => {
            xs.push(0);
        }
    }
    result
}

fn contributing_call(file: &mut File) -> Result<bool, Error> {
    let mut buf = [0];
    file.read(&mut buf).and_then(|size| {
        if size == 0 {
            Err(Error::from(ErrorKind::UnexpectedEof))
        } else {
            Ok(buf[0] != 0)
        }
    })
}

mod bank {
    struct Account {
        balance: i64,
    }

    struct InsufficientBalance;

    impl Account {
        fn withdraw(&mut self, amount: i64) -> Result<i64, InsufficientBalance> {
            self.balance -= amount;
            if self.balance < 0 {
                return Err(InsufficientBalance);
            }
            Ok(self.balance)
        }

        fn safe_withdraw(&mut self, amount: i64) -> Result<i64, InsufficientBalance> {
            let new_balance = self.balance - amount;
            if new_balance < 0 {
                return Err(InsufficientBalance);
            }
            self.balance = new_balance;
            Ok(self.balance)
        }
    }
}

mod more_than_two_variants {
    enum Error {
        Zero,
        One,
        Two,
    }

    fn deref_assign_before_err_return(flag: &mut bool) -> Result<(), Error> {
        *flag = true;
        Err(Error::Two)
    }
}
