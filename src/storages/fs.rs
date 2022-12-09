//! fs implementation

use std::env;
use std::io;
use std::path::{Path, PathBuf};

use crate::async_trait;
use crate::dto::{
    CompleteMultipartUploadError, CompleteMultipartUploadOutput, CompleteMultipartUploadRequest,
    CopyObjectError, CopyObjectOutput, CopyObjectRequest,
    CreateBucketError, CreateBucketOutput, CreateBucketRequest,
    CreateMultipartUploadError, CreateMultipartUploadOutput, CreateMultipartUploadRequest,
    DeleteBucketError, DeleteBucketOutput, DeleteBucketRequest,
    DeleteObjectError, DeleteObjectOutput, DeleteObjectRequest,
    DeleteObjectsError, DeleteObjectsOutput, DeleteObjectsRequest,
    GetBucketLocationError, GetBucketLocationOutput, GetBucketLocationRequest,
    GetObjectError, GetObjectOutput, GetObjectRequest,
    HeadBucketError, HeadBucketOutput, HeadBucketRequest,
    HeadObjectError, HeadObjectOutput, HeadObjectRequest,
    ListBucketsError, ListBucketsOutput, ListBucketsRequest,
    ListObjectsError, ListObjectsOutput, ListObjectsRequest,
    ListObjectsV2Error, ListObjectsV2Output, ListObjectsV2Request,
    PutObjectError, PutObjectOutput, PutObjectRequest,
    UploadPartError, UploadPartOutput, UploadPartRequest,
};
use crate::errors::S3StorageResult;
use crate::storage::S3Storage;

/// An S3 storage implementation backed by a file system
#[derive(Debug)]
pub struct FileSystem {
    /// root path
    root: PathBuf,
}

impl FileSystem {
    /// Constructs a file system storage located at `root`
    /// # Errors
    /// Returns an `Err` if current working directory is invalid or `root` doesn't exist
    pub fn new(root: impl AsRef<Path>) -> io::Result<Self> {
        let root = env::current_dir()?.join(root).canonicalize()?;
        Ok(Self { root })
    }
}

#[async_trait]
impl S3Storage for FileSystem {
    #[tracing::instrument]
    async fn create_bucket(
        &self,
        input: CreateBucketRequest,
    ) -> S3StorageResult<CreateBucketOutput, CreateBucketError> {
        Ok(CreateBucketOutput::default())
    }

    #[tracing::instrument]
    async fn copy_object(
        &self,
        input: CopyObjectRequest,
    ) -> S3StorageResult<CopyObjectOutput, CopyObjectError> {
        Ok(CopyObjectOutput::default())
    }

    #[tracing::instrument]
    async fn delete_bucket(
        &self,
        input: DeleteBucketRequest,
    ) -> S3StorageResult<DeleteBucketOutput, DeleteBucketError> {
        Ok(DeleteBucketOutput)
    }

    #[tracing::instrument]
    async fn delete_object(
        &self,
        input: DeleteObjectRequest) -> S3StorageResult<DeleteObjectOutput, DeleteObjectError> {
        Ok(DeleteObjectOutput::default())
    }

    #[tracing::instrument]
    async fn delete_objects(
        &self,
        input: DeleteObjectsRequest) -> S3StorageResult<DeleteObjectsOutput, DeleteObjectsError> {
        Ok(DeleteObjectsOutput::default())
    }

    #[tracing::instrument]
    async fn get_bucket_location(
        &self,
        input: GetBucketLocationRequest) -> S3StorageResult<GetBucketLocationOutput, GetBucketLocationError> {
        Ok(GetBucketLocationOutput::default())
    }

    #[tracing::instrument]
    async fn get_object(
        &self,
        input: GetObjectRequest) -> S3StorageResult<GetObjectOutput, GetObjectError> {
        Ok(GetObjectOutput::default())
    }

    #[tracing::instrument]
    async fn head_bucket(
        &self,
        input: HeadBucketRequest) -> S3StorageResult<HeadBucketOutput, HeadBucketError> {
        Ok(HeadBucketOutput)
    }

    #[tracing::instrument]
    async fn head_object(
        &self,
        input: HeadObjectRequest) -> S3StorageResult<HeadObjectOutput, HeadObjectError> {
        Ok(HeadObjectOutput::default())
    }

    #[tracing::instrument]
    async fn list_buckets(
        &self,
        input: ListBucketsRequest) -> S3StorageResult<ListBucketsOutput, ListBucketsError> {
        Ok(ListBucketsOutput::default())
    }

    #[tracing::instrument]
    async fn list_objects(
        &self,
        input: ListObjectsRequest) -> S3StorageResult<ListObjectsOutput, ListObjectsError> {
        Ok(ListObjectsOutput::default())
    }

    #[tracing::instrument]
    async fn list_objects_v2(
        &self,
        input: ListObjectsV2Request) -> S3StorageResult<ListObjectsV2Output, ListObjectsV2Error> {
        Ok(ListObjectsV2Output::default())
    }

    #[tracing::instrument]
    async fn put_object(
        &self,
        input: PutObjectRequest) -> S3StorageResult<PutObjectOutput, PutObjectError> {
        Ok(PutObjectOutput::default())
    }

    #[tracing::instrument]
    async fn create_multipart_upload(
        &self,
        input: CreateMultipartUploadRequest) -> S3StorageResult<CreateMultipartUploadOutput, CreateMultipartUploadError> {
        Ok(CreateMultipartUploadOutput::default())
    }

    #[tracing::instrument]
    async fn upload_part(
        &self,
        input: UploadPartRequest) -> S3StorageResult<UploadPartOutput, UploadPartError> {
        Ok(UploadPartOutput::default())
    }

    #[tracing::instrument]
    async fn complete_multipart_upload(
        &self,
        input: CompleteMultipartUploadRequest) -> S3StorageResult<CompleteMultipartUploadOutput, CompleteMultipartUploadError> {
        Ok(CompleteMultipartUploadOutput::default())
    }
}
