use std::path::PathBuf;
#[derive(Debug, Clone, PartialEq)]
pub enum ExitCode {
    Success(Option<Vec<PathBuf>>),
    GeneralError,
    KilledBySigint,
}

impl Into<(i32, Option<Vec<PathBuf>>)> for ExitCode {
    fn into(self) -> (i32, Option<Vec<PathBuf>>) {
        match self {
            ExitCode::Success(output) => (0, output),
            ExitCode::GeneralError => (1, None),
            ExitCode::KilledBySigint => (130, None),
        }
    }
}

impl ExitCode {
    fn is_error(&self) -> bool {
        match self {
            ExitCode::Success(_) => false,
            _ => true
        }
    }
}

pub fn merge_exitcodes(results: &[ExitCode]) -> ExitCode {
    if results.iter().any(ExitCode::is_error) {
        return ExitCode::GeneralError;
    }
    let mut buf: Vec<PathBuf> = Vec::new();
    for result in results {
        match result {
            ExitCode::Success(output) => {
                if let Some(lines) = output {
                    for line in lines.iter() {
                        buf.push( line.into() );
                    }
                }
            },
            _ => panic!("infallible: exit codes already purged.")
        }
    }
    if buf.len() > 0 {
        ExitCode::Success(Some(buf))
    } else {
        ExitCode::Success(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_when_no_results() {
        assert_eq!(merge_exitcodes(&[]), ExitCode::Success(None));
    }

    #[test]
    fn general_error_if_at_least_one_error() {
        assert_eq!(
            merge_exitcodes(&[ExitCode::GeneralError]),
            ExitCode::GeneralError
        );
        assert_eq!(
            merge_exitcodes(&[ExitCode::KilledBySigint]),
            ExitCode::GeneralError
        );
        assert_eq!(
            merge_exitcodes(&[ExitCode::KilledBySigint, ExitCode::Success(None)]),
            ExitCode::GeneralError
        );
        assert_eq!(
            merge_exitcodes(&[ExitCode::Success(None), ExitCode::GeneralError]),
            ExitCode::GeneralError
        );
        assert_eq!(
            merge_exitcodes(&[ExitCode::GeneralError, ExitCode::KilledBySigint]),
            ExitCode::GeneralError
        );
    }

    #[test]
    fn success_if_no_error() {
        assert_eq!(merge_exitcodes(&[ExitCode::Success(None)]), ExitCode::Success(None));
        assert_eq!(
            merge_exitcodes(&[ExitCode::Success(None), ExitCode::Success(None)]),
            ExitCode::Success(None)
        );
    }
}
