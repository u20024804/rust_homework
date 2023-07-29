#[macro_export]
macro_rules! longer_string {
    ($s1:expr, $s2:expr) => {
        {
            if($s1.len() >= $s2.len()) {
                $s1
            } else {
                $s2
            }
        }
    };

    ($s1:expr, $s2:expr, $s3:expr) => {
        {
            if($s1.len() >= $s2.len()) {
                    if($s1.len() >= $s3.len()) {
                        $s1
                    } else {
                        $s3
                    }
                } else {
                    if($s2.len() >= $s3.len()) {
                        $s2
                    } else {
                        $s3
                    }
                }
        }
    };
}
