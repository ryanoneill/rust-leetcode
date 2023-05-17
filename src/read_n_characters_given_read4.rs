/// Given a `file` and assume that you can only read the file using a given
/// method `read4`, implement a method to read `n` characters.
///
/// The API `read4` reads four consecutive characters from `file`, then writes
/// those characters into the buffer array `buf4`.
///
/// The return value is the number of actual characters read.
///
/// Note that `read4()` has its own file pointer, much like `FILE *fp` in C.
///
/// Definition of read4:
/// * Parameter: char[] buf4
/// * Returns:   int
///
/// buf4[] is a destination, not a source. The results from read4 will be
/// copied to buf4[].
///
/// ...
struct Solution;

impl Solution {

    // Placeholder
    fn read4(&self, _buf4: &mut [char]) -> i32 {
        0
    }

    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let n = n as usize;

        let mut copied = 0;
        let mut buffer4 = vec![' '; 4];

        while copied < n {
            let read = self.read4(&mut buffer4) as usize;

            for i in 0..read {
                if copied == n {
                    break;
                }
                buf[copied] = buffer4[i];
                copied += 1;
            }
            if read < 4 {
                break;
            }
        }

        copied as i32
    }

}
