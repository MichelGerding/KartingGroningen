macro_rules! get_vec {
    ($func:expr, $( $x:expr ),+) => {
        $func.bind($($x,)*)
            .all()
            .await
            .unwrap()
            .into_iter()
            .map(|e| e.into())
            .collect()
    }
}


macro_rules! get {
    ($func:expr, $( $x:expr ),+) => {
        $func.bind($($x,)*)
            .one()
            .await
            .unwrap()
            .into()
    }
}

pub(crate) use get_vec;
pub(crate) use get;
