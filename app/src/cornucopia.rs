// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod fortunes
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(serde::Serialize, Debug, Clone, PartialEq, )] pub struct Fortunes
{ pub id : i32,pub message : String,}pub struct FortunesBorrowed < 'a >
{ pub id : i32,pub message : &'a str,} impl < 'a > From < FortunesBorrowed <
'a >> for Fortunes
{
    fn
    from(FortunesBorrowed { id,message,} : FortunesBorrowed < 'a >)
    -> Self { Self { id,message: message.into(),} }
}pub struct FortunesQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> FortunesBorrowed,
    mapper : fn(FortunesBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > FortunesQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(FortunesBorrowed) -> R) -> FortunesQuery
    < 'a, C, R, N >
    {
        FortunesQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn fortunes() -> FortunesStmt
{ FortunesStmt(cornucopia_async :: private :: Stmt :: new("/*
This will generate a function called fortunes which will run the SQL query. Note cornucopia checks the query at code generation time against Postgres.
*/
SELECT 
    id, message
FROM 
    Fortune")) } pub
struct FortunesStmt(cornucopia_async :: private :: Stmt) ; impl
FortunesStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> FortunesQuery < 'a, C,
Fortunes, 0 >
{
    FortunesQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { FortunesBorrowed { id : row.get(0),message : row.get(1),} }, mapper : | it | { <Fortunes>::from(it) },
    }
} }}}