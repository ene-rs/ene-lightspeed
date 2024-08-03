pub trait SplitAtMultipleByteIndices {
    fn split_at_multiple_byte_indices(&self, indices: &[usize]) -> Vec<&str>;
}

impl SplitAtMultipleByteIndices for &str {
    fn split_at_multiple_byte_indices(&self, indices: &[usize]) -> Vec<&str> {
        let mut result = Vec::new();
        let mut start = 0;
        for &end in indices {
            result.push(&self[start..end]);
            start = end;
        }
        result.push(&self[start..]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_at_multiple_byte_indices() {
        let s = "this is a test to split at multiple byte indices";
        let indices = [4, 8, 12];
        let result = s.split_at_multiple_byte_indices(&indices);
        insta::assert_debug_snapshot!(result);
    }
}
