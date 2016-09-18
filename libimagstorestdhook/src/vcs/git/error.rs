use git2::Error as Git2Error;

use libimagstore::hook::error::HookError as HE;
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::result::HookResult;

generate_error_module!(
    generate_error_types!(GitHookError, GitHookErrorKind,
        ConfigError => "Configuration Error",
        NoConfigError => "No Configuration",
        ConfigTypeError => "Configuration value type wrong",

        RepositoryError                   => "Error while interacting with git repository",
        RepositoryInitError               => "Error while loading the git repository",
        RepositoryBackendError            => "Error in the git library",
        RepositoryBranchError             => "Error while interacting with git branch(es)",
        RepositoryBranchNameFetchingError => "Error while fetching branch name",
        RepositoryIndexFetchingError      => "Error while fetching Repository Index",
        RepositoryIndexWritingError       => "Error while writing Repository Index",
        RepositoryPathAddingError         => "Error while adding Path to Index",
        RepositoryCommittingError         => "Error while committing",
        RepositoryParentFetchingError     => "Error while fetching parent of commit",

        HeadFetchError                    => "Error while getting HEAD",
        NotOnBranch                       => "No Branch is checked out",
        MkRepo                            => "Repository creation error",
        MkSignature                       => "Error while building Signature object",

        RepositoryFileStatusError         => "Error while getting file status",

        GitConfigFetchError               => "Error fetching git config",
        GitConfigEditorFetchError         => "Error fetching 'editor' from git config",
        EditorError                       => "Error while calling editor"
    );
);

impl GitHookError {

    pub fn inside_of<T>(self, h: HEK) -> HookResult<T> {
        Err(HE::new(h, Some(Box::new(self))))
    }

}

impl From<GitHookError> for HE {

    fn from(he: GitHookError) -> HE {
        HE::new(HEK::HookExecutionError, Some(Box::new(he)))
    }

}

impl From<Git2Error> for GitHookError {

    fn from(ge: Git2Error) -> GitHookError {
        GitHookError::new(GitHookErrorKind::RepositoryBackendError, Some(Box::new(ge)))
    }

}

pub trait MapIntoHookError<T> {
    fn map_into_hook_error(self) -> Result<T, HE>;
}

impl<T> MapIntoHookError<T> for Result<T, GitHookError> {

    fn map_into_hook_error(self) -> Result<T, HE> {
        self.map_err(|e| HE::new(HEK::HookExecutionError, Some(Box::new(e))))
    }

}

pub use self::error::GitHookError;
pub use self::error::GitHookErrorKind;
pub use self::error::MapErrInto;

