// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod driver
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct SearchDriverWithStatsPaginatedParams < T1 : cornucopia_async::StringSql,> { pub name : T1,pub limit : i64,pub offset : i64,}#[derive( Debug)] pub struct SearchDriverByNameParams < T1 : cornucopia_async::StringSql,> { pub name : T1,pub limit : i64,pub offset : i64,}#[derive(Clone,Copy, Debug)] pub struct UpdateDriverRatingParams < > { pub rating : f64,pub uncertainty : f64,pub id : i32,}#[derive( Debug)] pub struct CreateDriverParams < T1 : cornucopia_async::StringSql,> { pub name : T1,pub rating : f64,pub uncertainty : f64,}#[derive( Debug, Clone, PartialEq, )] pub struct Driver
{ pub id : i32,pub name : String,pub rating : f64,pub uncertainty : f64,}pub struct DriverBorrowed < 'a >
{ pub id : i32,pub name : &'a str,pub rating : f64,pub uncertainty : f64,} impl < 'a > From < DriverBorrowed <
'a >> for Driver
{
    fn
    from(DriverBorrowed { id,name,rating,uncertainty,} : DriverBorrowed < 'a >)
    -> Self { Self { id,name: name.into(),rating,uncertainty,} }
}pub struct DriverQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> DriverBorrowed,
    mapper : fn(DriverBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > DriverQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(DriverBorrowed) -> R) -> DriverQuery
    < 'a, C, R, N >
    {
        DriverQuery
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
}#[derive( Debug, Clone, PartialEq, )] pub struct DriverWithStats
{ pub id : i32,pub name : String,pub rating : f64,pub uncertainty : f64,pub fastest_lap_time : f64,pub avg_lap_time : f64,pub median_lap_time : f64,pub total_laps : i32,pub total_heats : i32,}pub struct DriverWithStatsBorrowed < 'a >
{ pub id : i32,pub name : &'a str,pub rating : f64,pub uncertainty : f64,pub fastest_lap_time : f64,pub avg_lap_time : f64,pub median_lap_time : f64,pub total_laps : i32,pub total_heats : i32,} impl < 'a > From < DriverWithStatsBorrowed <
'a >> for DriverWithStats
{
    fn
    from(DriverWithStatsBorrowed { id,name,rating,uncertainty,fastest_lap_time,avg_lap_time,median_lap_time,total_laps,total_heats,} : DriverWithStatsBorrowed < 'a >)
    -> Self { Self { id,name: name.into(),rating,uncertainty,fastest_lap_time,avg_lap_time,median_lap_time,total_laps,total_heats,} }
}pub struct DriverWithStatsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> DriverWithStatsBorrowed,
    mapper : fn(DriverWithStatsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > DriverWithStatsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(DriverWithStatsBorrowed) -> R) -> DriverWithStatsQuery
    < 'a, C, R, N >
    {
        DriverWithStatsQuery
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
}#[derive( Debug, Clone, PartialEq, )] pub struct GetDriversFromHeatInOrderFastestLap
{ pub id : i32,pub name : String,pub rating : f64,pub uncertainty : f64,}pub struct GetDriversFromHeatInOrderFastestLapBorrowed < 'a >
{ pub id : i32,pub name : &'a str,pub rating : f64,pub uncertainty : f64,} impl < 'a > From < GetDriversFromHeatInOrderFastestLapBorrowed <
'a >> for GetDriversFromHeatInOrderFastestLap
{
    fn
    from(GetDriversFromHeatInOrderFastestLapBorrowed { id,name,rating,uncertainty,} : GetDriversFromHeatInOrderFastestLapBorrowed < 'a >)
    -> Self { Self { id,name: name.into(),rating,uncertainty,} }
}pub struct GetDriversFromHeatInOrderFastestLapQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetDriversFromHeatInOrderFastestLapBorrowed,
    mapper : fn(GetDriversFromHeatInOrderFastestLapBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetDriversFromHeatInOrderFastestLapQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetDriversFromHeatInOrderFastestLapBorrowed) -> R) -> GetDriversFromHeatInOrderFastestLapQuery
    < 'a, C, R, N >
    {
        GetDriversFromHeatInOrderFastestLapQuery
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
}pub struct I64Query < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> i64,
    mapper : fn(i64) -> T,
} impl < 'a, C, T : 'a, const N : usize > I64Query < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(i64) -> R) -> I64Query
    < 'a, C, R, N >
    {
        I64Query
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
}pub fn get_all_drivers() -> GetAllDriversStmt
{ GetAllDriversStmt(cornucopia_async :: private :: Stmt :: new("select * from public.drivers")) } pub
struct GetAllDriversStmt(cornucopia_async :: private :: Stmt) ; impl
GetAllDriversStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> DriverQuery < 'a, C,
Driver, 0 >
{
    DriverQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { DriverBorrowed { id : row.get(0),name : row.get(1),rating : row.get(2),uncertainty : row.get(3),} }, mapper : | it | { <Driver>::from(it) },
    }
} }pub fn get_driver_by_id() -> GetDriverByIdStmt
{ GetDriverByIdStmt(cornucopia_async :: private :: Stmt :: new("select * from public.drivers WHERE id = $1")) } pub
struct GetDriverByIdStmt(cornucopia_async :: private :: Stmt) ; impl
GetDriverByIdStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
id : & 'a i32,) -> DriverQuery < 'a, C,
Driver, 1 >
{
    DriverQuery
    {
        client, params : [id,], stmt : & mut self.0, extractor :
        | row | { DriverBorrowed { id : row.get(0),name : row.get(1),rating : row.get(2),uncertainty : row.get(3),} }, mapper : | it | { <Driver>::from(it) },
    }
} }pub fn get_drivers_by_ids() -> GetDriversByIdsStmt
{ GetDriversByIdsStmt(cornucopia_async :: private :: Stmt :: new("select * from public.drivers WHERE id = any($1::int[])")) } pub
struct GetDriversByIdsStmt(cornucopia_async :: private :: Stmt) ; impl
GetDriversByIdsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::ArraySql<Item = i32>,>
(& 'a mut self, client : & 'a  C,
ids : & 'a T1,) -> DriverQuery < 'a, C,
Driver, 1 >
{
    DriverQuery
    {
        client, params : [ids,], stmt : & mut self.0, extractor :
        | row | { DriverBorrowed { id : row.get(0),name : row.get(1),rating : row.get(2),uncertainty : row.get(3),} }, mapper : | it | { <Driver>::from(it) },
    }
} }pub fn search_driver_with_stats_paginated() -> SearchDriverWithStatsPaginatedStmt
{ SearchDriverWithStatsPaginatedStmt(cornucopia_async :: private :: Stmt :: new("select
    d.*,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as avg_lap_time,
    percentile_cont(0.5) WITHIN GROUP ( ORDER BY l.lap_time) as median_lap_time,
    CAST(count(l.lap_time) AS INT) as total_laps,
    CAST(count(DISTINCT l.heat) AS INT) as total_heats
from public.drivers d
         inner join public.laps l on d.id = l.driver
where d.name like concat('%', $1::text ,'%')
GROUP BY d.id
limit $2 offset $3")) } pub
struct SearchDriverWithStatsPaginatedStmt(cornucopia_async :: private :: Stmt) ; impl
SearchDriverWithStatsPaginatedStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
name : & 'a T1,limit : & 'a i64,offset : & 'a i64,) -> DriverWithStatsQuery < 'a, C,
DriverWithStats, 3 >
{
    DriverWithStatsQuery
    {
        client, params : [name,limit,offset,], stmt : & mut self.0, extractor :
        | row | { DriverWithStatsBorrowed { id : row.get(0),name : row.get(1),rating : row.get(2),uncertainty : row.get(3),fastest_lap_time : row.get(4),avg_lap_time : row.get(5),median_lap_time : row.get(6),total_laps : row.get(7),total_heats : row.get(8),} }, mapper : | it | { <DriverWithStats>::from(it) },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, SearchDriverWithStatsPaginatedParams < T1,>, DriverWithStatsQuery < 'a,
C, DriverWithStats, 3 >, C > for SearchDriverWithStatsPaginatedStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    SearchDriverWithStatsPaginatedParams < T1,>) -> DriverWithStatsQuery < 'a, C,
    DriverWithStats, 3 >
    { self.bind(client, & params.name,& params.limit,& params.offset,) }
}pub fn get_driver_with_stats() -> GetDriverWithStatsStmt
{ GetDriverWithStatsStmt(cornucopia_async :: private :: Stmt :: new("select
    d.*,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as avg_lap_time,
    percentile_cont(0.5) WITHIN GROUP ( ORDER BY l.lap_time) as median_lap_time,
    CAST(count(l.lap_time) AS INT) as total_laps,
    CAST(count(DISTINCT l.heat) AS INT) as total_heats
from public.drivers d
         inner join public.laps l on d.id = l.driver
where d.id = $1::int
GROUP BY d.id")) } pub
struct GetDriverWithStatsStmt(cornucopia_async :: private :: Stmt) ; impl
GetDriverWithStatsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
id : & 'a i32,) -> DriverWithStatsQuery < 'a, C,
DriverWithStats, 1 >
{
    DriverWithStatsQuery
    {
        client, params : [id,], stmt : & mut self.0, extractor :
        | row | { DriverWithStatsBorrowed { id : row.get(0),name : row.get(1),rating : row.get(2),uncertainty : row.get(3),fastest_lap_time : row.get(4),avg_lap_time : row.get(5),median_lap_time : row.get(6),total_laps : row.get(7),total_heats : row.get(8),} }, mapper : | it | { <DriverWithStats>::from(it) },
    }
} }pub fn get_driver_by_name() -> GetDriverByNameStmt
{ GetDriverByNameStmt(cornucopia_async :: private :: Stmt :: new("select
    *
from public.drivers
WHERE name = $1::text")) } pub
struct GetDriverByNameStmt(cornucopia_async :: private :: Stmt) ; impl
GetDriverByNameStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
name : & 'a T1,) -> DriverQuery < 'a, C,
Driver, 1 >
{
    DriverQuery
    {
        client, params : [name,], stmt : & mut self.0, extractor :
        | row | { DriverBorrowed { id : row.get(0),name : row.get(1),rating : row.get(2),uncertainty : row.get(3),} }, mapper : | it | { <Driver>::from(it) },
    }
} }pub fn search_driver_by_name() -> SearchDriverByNameStmt
{ SearchDriverByNameStmt(cornucopia_async :: private :: Stmt :: new("select
    *
from public.drivers
WHERE name LIKE concat('%', $1::text, '%')
LIMIT $2 OFFSET $3")) } pub
struct SearchDriverByNameStmt(cornucopia_async :: private :: Stmt) ; impl
SearchDriverByNameStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
name : & 'a T1,limit : & 'a i64,offset : & 'a i64,) -> DriverQuery < 'a, C,
Driver, 3 >
{
    DriverQuery
    {
        client, params : [name,limit,offset,], stmt : & mut self.0, extractor :
        | row | { DriverBorrowed { id : row.get(0),name : row.get(1),rating : row.get(2),uncertainty : row.get(3),} }, mapper : | it | { <Driver>::from(it) },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, SearchDriverByNameParams < T1,>, DriverQuery < 'a,
C, Driver, 3 >, C > for SearchDriverByNameStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    SearchDriverByNameParams < T1,>) -> DriverQuery < 'a, C,
    Driver, 3 >
    { self.bind(client, & params.name,& params.limit,& params.offset,) }
}pub fn get_drivers_from_heat_in_order_fastest_lap() -> GetDriversFromHeatInOrderFastestLapStmt
{ GetDriversFromHeatInOrderFastestLapStmt(cornucopia_async :: private :: Stmt :: new("select
    d.*
from public.drivers d
inner join public.laps l on d.id = l.driver
inner join public.session h on h.id = l.heat
where h.id = $1
group by d.id
order by min(l.lap_time) asc")) } pub
struct GetDriversFromHeatInOrderFastestLapStmt(cornucopia_async :: private :: Stmt) ; impl
GetDriversFromHeatInOrderFastestLapStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
heat : & 'a i32,) -> GetDriversFromHeatInOrderFastestLapQuery < 'a, C,
GetDriversFromHeatInOrderFastestLap, 1 >
{
    GetDriversFromHeatInOrderFastestLapQuery
    {
        client, params : [heat,], stmt : & mut self.0, extractor :
        | row | { GetDriversFromHeatInOrderFastestLapBorrowed { id : row.get(0),name : row.get(1),rating : row.get(2),uncertainty : row.get(3),} }, mapper : | it | { <GetDriversFromHeatInOrderFastestLap>::from(it) },
    }
} }pub fn update_driver_rating() -> UpdateDriverRatingStmt
{ UpdateDriverRatingStmt(cornucopia_async :: private :: Stmt :: new("with update_rows AS (
    UPDATE
        public.drivers
    set
        rating=$1,
        uncertainty=$2
    where id = $3
    RETURNING 1
) SELECT count(*) FROM update_rows")) } pub
struct UpdateDriverRatingStmt(cornucopia_async :: private :: Stmt) ; impl
UpdateDriverRatingStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
rating : & 'a f64,uncertainty : & 'a f64,id : & 'a i32,) -> I64Query < 'a, C,
i64, 3 >
{
    I64Query
    {
        client, params : [rating,uncertainty,id,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }impl < 'a, C : GenericClient, > cornucopia_async ::
Params < 'a, UpdateDriverRatingParams < >, I64Query < 'a,
C, i64, 3 >, C > for UpdateDriverRatingStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    UpdateDriverRatingParams < >) -> I64Query < 'a, C,
    i64, 3 >
    { self.bind(client, & params.rating,& params.uncertainty,& params.id,) }
}pub fn create_driver() -> CreateDriverStmt
{ CreateDriverStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO public.drivers (name, rating, uncertainty) values ($1, $2, $3) RETURNING *")) } pub
struct CreateDriverStmt(cornucopia_async :: private :: Stmt) ; impl
CreateDriverStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
name : & 'a T1,rating : & 'a f64,uncertainty : & 'a f64,) -> DriverQuery < 'a, C,
Driver, 3 >
{
    DriverQuery
    {
        client, params : [name,rating,uncertainty,], stmt : & mut self.0, extractor :
        | row | { DriverBorrowed { id : row.get(0),name : row.get(1),rating : row.get(2),uncertainty : row.get(3),} }, mapper : | it | { <Driver>::from(it) },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, CreateDriverParams < T1,>, DriverQuery < 'a,
C, Driver, 3 >, C > for CreateDriverStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    CreateDriverParams < T1,>) -> DriverQuery < 'a, C,
    Driver, 3 >
    { self.bind(client, & params.name,& params.rating,& params.uncertainty,) }
}}pub mod heats
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(Clone,Copy, Debug)] pub struct GetHeatWithStatsPaginatedParams < > { pub limit : i64,pub offset : i64,}#[derive( Debug)] pub struct CreateNewHeatParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> { pub heat_id : T1,pub heat_type : T2,pub start_date : time::PrimitiveDateTime,}#[derive( Debug, Clone, PartialEq, )] pub struct Heat
{ pub id : i32,pub heat_id : String,pub heat_type : String,pub start_date : time::PrimitiveDateTime,}pub struct HeatBorrowed < 'a >
{ pub id : i32,pub heat_id : &'a str,pub heat_type : &'a str,pub start_date : time::PrimitiveDateTime,} impl < 'a > From < HeatBorrowed <
'a >> for Heat
{
    fn
    from(HeatBorrowed { id,heat_id,heat_type,start_date,} : HeatBorrowed < 'a >)
    -> Self { Self { id,heat_id: heat_id.into(),heat_type: heat_type.into(),start_date,} }
}pub struct HeatQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> HeatBorrowed,
    mapper : fn(HeatBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > HeatQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(HeatBorrowed) -> R) -> HeatQuery
    < 'a, C, R, N >
    {
        HeatQuery
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
}#[derive( Debug, Clone, PartialEq, )] pub struct GetHeatWithStats
{ pub heat_id : String,pub heat_type : String,pub start_time : time::PrimitiveDateTime,pub amount_of_laps : i32,pub amount_of_drivers : i32,pub fastest_lap_time : f64,pub average_lap_time : f64,}pub struct GetHeatWithStatsBorrowed < 'a >
{ pub heat_id : &'a str,pub heat_type : &'a str,pub start_time : time::PrimitiveDateTime,pub amount_of_laps : i32,pub amount_of_drivers : i32,pub fastest_lap_time : f64,pub average_lap_time : f64,} impl < 'a > From < GetHeatWithStatsBorrowed <
'a >> for GetHeatWithStats
{
    fn
    from(GetHeatWithStatsBorrowed { heat_id,heat_type,start_time,amount_of_laps,amount_of_drivers,fastest_lap_time,average_lap_time,} : GetHeatWithStatsBorrowed < 'a >)
    -> Self { Self { heat_id: heat_id.into(),heat_type: heat_type.into(),start_time,amount_of_laps,amount_of_drivers,fastest_lap_time,average_lap_time,} }
}pub struct GetHeatWithStatsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetHeatWithStatsBorrowed,
    mapper : fn(GetHeatWithStatsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetHeatWithStatsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetHeatWithStatsBorrowed) -> R) -> GetHeatWithStatsQuery
    < 'a, C, R, N >
    {
        GetHeatWithStatsQuery
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
}#[derive( Debug, Clone, PartialEq, )] pub struct DeleteHeat
{ pub id : i32,pub heat_id : String,pub heat_type : String,pub start_date : time::PrimitiveDateTime,}pub struct DeleteHeatBorrowed < 'a >
{ pub id : i32,pub heat_id : &'a str,pub heat_type : &'a str,pub start_date : time::PrimitiveDateTime,} impl < 'a > From < DeleteHeatBorrowed <
'a >> for DeleteHeat
{
    fn
    from(DeleteHeatBorrowed { id,heat_id,heat_type,start_date,} : DeleteHeatBorrowed < 'a >)
    -> Self { Self { id,heat_id: heat_id.into(),heat_type: heat_type.into(),start_date,} }
}pub struct DeleteHeatQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> DeleteHeatBorrowed,
    mapper : fn(DeleteHeatBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > DeleteHeatQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(DeleteHeatBorrowed) -> R) -> DeleteHeatQuery
    < 'a, C, R, N >
    {
        DeleteHeatQuery
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
}pub fn get_all_heats() -> GetAllHeatsStmt
{ GetAllHeatsStmt(cornucopia_async :: private :: Stmt :: new("select * from public.session")) } pub
struct GetAllHeatsStmt(cornucopia_async :: private :: Stmt) ; impl
GetAllHeatsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> HeatQuery < 'a, C,
Heat, 0 >
{
    HeatQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { HeatBorrowed { id : row.get(0),heat_id : row.get(1),heat_type : row.get(2),start_date : row.get(3),} }, mapper : | it | { <Heat>::from(it) },
    }
} }pub fn get_heat_from_id() -> GetHeatFromIdStmt
{ GetHeatFromIdStmt(cornucopia_async :: private :: Stmt :: new("select * from public.session where id = $1")) } pub
struct GetHeatFromIdStmt(cornucopia_async :: private :: Stmt) ; impl
GetHeatFromIdStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
id : & 'a i32,) -> HeatQuery < 'a, C,
Heat, 1 >
{
    HeatQuery
    {
        client, params : [id,], stmt : & mut self.0, extractor :
        | row | { HeatBorrowed { id : row.get(0),heat_id : row.get(1),heat_type : row.get(2),start_date : row.get(3),} }, mapper : | it | { <Heat>::from(it) },
    }
} }pub fn get_heats_from_ids() -> GetHeatsFromIdsStmt
{ GetHeatsFromIdsStmt(cornucopia_async :: private :: Stmt :: new("select * from public.session WHERE id = any($1::int[])")) } pub
struct GetHeatsFromIdsStmt(cornucopia_async :: private :: Stmt) ; impl
GetHeatsFromIdsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::ArraySql<Item = i32>,>
(& 'a mut self, client : & 'a  C,
ids : & 'a T1,) -> HeatQuery < 'a, C,
Heat, 1 >
{
    HeatQuery
    {
        client, params : [ids,], stmt : & mut self.0, extractor :
        | row | { HeatBorrowed { id : row.get(0),heat_id : row.get(1),heat_type : row.get(2),start_date : row.get(3),} }, mapper : | it | { <Heat>::from(it) },
    }
} }pub fn get_heat_from_name() -> GetHeatFromNameStmt
{ GetHeatFromNameStmt(cornucopia_async :: private :: Stmt :: new("select * from public.session where heat_id = $1::text")) } pub
struct GetHeatFromNameStmt(cornucopia_async :: private :: Stmt) ; impl
GetHeatFromNameStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
name : & 'a T1,) -> HeatQuery < 'a, C,
Heat, 1 >
{
    HeatQuery
    {
        client, params : [name,], stmt : & mut self.0, extractor :
        | row | { HeatBorrowed { id : row.get(0),heat_id : row.get(1),heat_type : row.get(2),start_date : row.get(3),} }, mapper : | it | { <Heat>::from(it) },
    }
} }pub fn get_all_heats_with_stats() -> GetAllHeatsWithStatsStmt
{ GetAllHeatsWithStatsStmt(cornucopia_async :: private :: Stmt :: new("select
    h.heat_id,
    h.heat_type,
    h.start_date as start_time,
    CAST(count(l.*) as INT) as amount_of_laps,
    CAST(count(DISTINCT l.driver) AS INT) as amount_of_drivers,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as average_lap_time
from public.session h
         inner join public.laps l on h.id = l.heat
group by h.id")) } pub
struct GetAllHeatsWithStatsStmt(cornucopia_async :: private :: Stmt) ; impl
GetAllHeatsWithStatsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> GetHeatWithStatsQuery < 'a, C,
GetHeatWithStats, 0 >
{
    GetHeatWithStatsQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { GetHeatWithStatsBorrowed { heat_id : row.get(0),heat_type : row.get(1),start_time : row.get(2),amount_of_laps : row.get(3),amount_of_drivers : row.get(4),fastest_lap_time : row.get(5),average_lap_time : row.get(6),} }, mapper : | it | { <GetHeatWithStats>::from(it) },
    }
} }pub fn get_heat_with_stats_paginated() -> GetHeatWithStatsPaginatedStmt
{ GetHeatWithStatsPaginatedStmt(cornucopia_async :: private :: Stmt :: new("select
    h.heat_id,
    h.heat_type,
    h.start_date as start_time,
    CAST(count(l.*) as INT) as amount_of_laps,
    CAST(count(DISTINCT l.driver) AS INT) as amount_of_drivers,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as average_lap_time
from public.session h
         inner join public.laps l on h.id = l.heat
group by h.id, start_time
limit $1
offset $2")) } pub
struct GetHeatWithStatsPaginatedStmt(cornucopia_async :: private :: Stmt) ; impl
GetHeatWithStatsPaginatedStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
limit : & 'a i64,offset : & 'a i64,) -> GetHeatWithStatsQuery < 'a, C,
GetHeatWithStats, 2 >
{
    GetHeatWithStatsQuery
    {
        client, params : [limit,offset,], stmt : & mut self.0, extractor :
        | row | { GetHeatWithStatsBorrowed { heat_id : row.get(0),heat_type : row.get(1),start_time : row.get(2),amount_of_laps : row.get(3),amount_of_drivers : row.get(4),fastest_lap_time : row.get(5),average_lap_time : row.get(6),} }, mapper : | it | { <GetHeatWithStats>::from(it) },
    }
} }impl < 'a, C : GenericClient, > cornucopia_async ::
Params < 'a, GetHeatWithStatsPaginatedParams < >, GetHeatWithStatsQuery < 'a,
C, GetHeatWithStats, 2 >, C > for GetHeatWithStatsPaginatedStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    GetHeatWithStatsPaginatedParams < >) -> GetHeatWithStatsQuery < 'a, C,
    GetHeatWithStats, 2 >
    { self.bind(client, & params.limit,& params.offset,) }
}pub fn get_heat_with_stats() -> GetHeatWithStatsStmt
{ GetHeatWithStatsStmt(cornucopia_async :: private :: Stmt :: new("select
    h.heat_id,
    h.heat_type,
    h.start_date as start_time,
    CAST(count(l.*) as INT) as amount_of_laps,
    CAST(count(DISTINCT l.driver) AS INT) as amount_of_drivers,
    min(l.lap_time) as fastest_lap_time,
    avg(l.lap_time) as average_lap_time
from public.session h
         inner join public.laps l on h.id = l.heat
where h.heat_id = $1
group by h.id")) } pub
struct GetHeatWithStatsStmt(cornucopia_async :: private :: Stmt) ; impl
GetHeatWithStatsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
heat : & 'a T1,) -> GetHeatWithStatsQuery < 'a, C,
GetHeatWithStats, 1 >
{
    GetHeatWithStatsQuery
    {
        client, params : [heat,], stmt : & mut self.0, extractor :
        | row | { GetHeatWithStatsBorrowed { heat_id : row.get(0),heat_type : row.get(1),start_time : row.get(2),amount_of_laps : row.get(3),amount_of_drivers : row.get(4),fastest_lap_time : row.get(5),average_lap_time : row.get(6),} }, mapper : | it | { <GetHeatWithStats>::from(it) },
    }
} }pub fn get_all_chronologicaly() -> GetAllChronologicalyStmt
{ GetAllChronologicalyStmt(cornucopia_async :: private :: Stmt :: new("select * from public.session order by start_date")) } pub
struct GetAllChronologicalyStmt(cornucopia_async :: private :: Stmt) ; impl
GetAllChronologicalyStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> HeatQuery < 'a, C,
Heat, 0 >
{
    HeatQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { HeatBorrowed { id : row.get(0),heat_id : row.get(1),heat_type : row.get(2),start_date : row.get(3),} }, mapper : | it | { <Heat>::from(it) },
    }
} }pub fn delete_heat() -> DeleteHeatStmt
{ DeleteHeatStmt(cornucopia_async :: private :: Stmt :: new("delete from public.session where id = $1 returning *")) } pub
struct DeleteHeatStmt(cornucopia_async :: private :: Stmt) ; impl
DeleteHeatStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
id : & 'a i32,) -> DeleteHeatQuery < 'a, C,
DeleteHeat, 1 >
{
    DeleteHeatQuery
    {
        client, params : [id,], stmt : & mut self.0, extractor :
        | row | { DeleteHeatBorrowed { id : row.get(0),heat_id : row.get(1),heat_type : row.get(2),start_date : row.get(3),} }, mapper : | it | { <DeleteHeat>::from(it) },
    }
} }pub fn create_new_heat() -> CreateNewHeatStmt
{ CreateNewHeatStmt(cornucopia_async :: private :: Stmt :: new("insert into public.session (heat_id, heat_type, start_date) VALUES ($1, $2, $3) returning  *")) } pub
struct CreateNewHeatStmt(cornucopia_async :: private :: Stmt) ; impl
CreateNewHeatStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
heat_id : & 'a T1,heat_type : & 'a T2,start_date : & 'a time::PrimitiveDateTime,) -> HeatQuery < 'a, C,
Heat, 3 >
{
    HeatQuery
    {
        client, params : [heat_id,heat_type,start_date,], stmt : & mut self.0, extractor :
        | row | { HeatBorrowed { id : row.get(0),heat_id : row.get(1),heat_type : row.get(2),start_date : row.get(3),} }, mapper : | it | { <Heat>::from(it) },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, CreateNewHeatParams < T1,T2,>, HeatQuery < 'a,
C, Heat, 3 >, C > for CreateNewHeatStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    CreateNewHeatParams < T1,T2,>) -> HeatQuery < 'a, C,
    Heat, 3 >
    { self.bind(client, & params.heat_id,& params.heat_type,& params.start_date,) }
}}pub mod kart
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CreateKartParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> { pub number : i32,pub brand : T1,pub model : T2,pub horsepower : i32,pub modified : bool,}#[derive( Debug, Clone, PartialEq, )] pub struct Kart
{ pub id : i32,pub number : i32,pub brand : String,pub model : String,pub horsepower : i32,pub modified : bool,}pub struct KartBorrowed < 'a >
{ pub id : i32,pub number : i32,pub brand : &'a str,pub model : &'a str,pub horsepower : i32,pub modified : bool,} impl < 'a > From < KartBorrowed <
'a >> for Kart
{
    fn
    from(KartBorrowed { id,number,brand,model,horsepower,modified,} : KartBorrowed < 'a >)
    -> Self { Self { id,number,brand: brand.into(),model: model.into(),horsepower,modified,} }
}pub struct KartQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> KartBorrowed,
    mapper : fn(KartBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > KartQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(KartBorrowed) -> R) -> KartQuery
    < 'a, C, R, N >
    {
        KartQuery
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
}#[derive( Debug, Clone, PartialEq, )] pub struct GetKartsStatsPerDay
{ pub id : i32,pub number : i32,pub brand : String,pub model : String,pub horsepower : i32,pub modified : bool,pub start_date : time::PrimitiveDateTime,pub min_laptime : f64,pub avg_laptime : f64,pub median_laptime : f64,}pub struct GetKartsStatsPerDayBorrowed < 'a >
{ pub id : i32,pub number : i32,pub brand : &'a str,pub model : &'a str,pub horsepower : i32,pub modified : bool,pub start_date : time::PrimitiveDateTime,pub min_laptime : f64,pub avg_laptime : f64,pub median_laptime : f64,} impl < 'a > From < GetKartsStatsPerDayBorrowed <
'a >> for GetKartsStatsPerDay
{
    fn
    from(GetKartsStatsPerDayBorrowed { id,number,brand,model,horsepower,modified,start_date,min_laptime,avg_laptime,median_laptime,} : GetKartsStatsPerDayBorrowed < 'a >)
    -> Self { Self { id,number,brand: brand.into(),model: model.into(),horsepower,modified,start_date,min_laptime,avg_laptime,median_laptime,} }
}pub struct GetKartsStatsPerDayQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> GetKartsStatsPerDayBorrowed,
    mapper : fn(GetKartsStatsPerDayBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > GetKartsStatsPerDayQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(GetKartsStatsPerDayBorrowed) -> R) -> GetKartsStatsPerDayQuery
    < 'a, C, R, N >
    {
        GetKartsStatsPerDayQuery
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
}#[derive( Debug, Clone, PartialEq, )] pub struct KartWithStats
{ pub id : i32,pub number : i32,pub brand : String,pub model : String,pub horsepower : i32,pub modified : bool,pub lap_count : i32,pub driver_count : i32,}pub struct KartWithStatsBorrowed < 'a >
{ pub id : i32,pub number : i32,pub brand : &'a str,pub model : &'a str,pub horsepower : i32,pub modified : bool,pub lap_count : i32,pub driver_count : i32,} impl < 'a > From < KartWithStatsBorrowed <
'a >> for KartWithStats
{
    fn
    from(KartWithStatsBorrowed { id,number,brand,model,horsepower,modified,lap_count,driver_count,} : KartWithStatsBorrowed < 'a >)
    -> Self { Self { id,number,brand: brand.into(),model: model.into(),horsepower,modified,lap_count,driver_count,} }
}pub struct KartWithStatsQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> KartWithStatsBorrowed,
    mapper : fn(KartWithStatsBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > KartWithStatsQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(KartWithStatsBorrowed) -> R) -> KartWithStatsQuery
    < 'a, C, R, N >
    {
        KartWithStatsQuery
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
}pub struct I32Query < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> i32,
    mapper : fn(i32) -> T,
} impl < 'a, C, T : 'a, const N : usize > I32Query < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(i32) -> R) -> I32Query
    < 'a, C, R, N >
    {
        I32Query
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
}pub fn get_all_karts() -> GetAllKartsStmt
{ GetAllKartsStmt(cornucopia_async :: private :: Stmt :: new("select * from public.cars")) } pub
struct GetAllKartsStmt(cornucopia_async :: private :: Stmt) ; impl
GetAllKartsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> KartQuery < 'a, C,
Kart, 0 >
{
    KartQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { KartBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),} }, mapper : | it | { <Kart>::from(it) },
    }
} }pub fn get_kart_by_id() -> GetKartByIdStmt
{ GetKartByIdStmt(cornucopia_async :: private :: Stmt :: new("select * from public.cars WHERE id = $1")) } pub
struct GetKartByIdStmt(cornucopia_async :: private :: Stmt) ; impl
GetKartByIdStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
id : & 'a i32,) -> KartQuery < 'a, C,
Kart, 1 >
{
    KartQuery
    {
        client, params : [id,], stmt : & mut self.0, extractor :
        | row | { KartBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),} }, mapper : | it | { <Kart>::from(it) },
    }
} }pub fn get_karts_by_ids() -> GetKartsByIdsStmt
{ GetKartsByIdsStmt(cornucopia_async :: private :: Stmt :: new("select * from public.cars WHERE id = any($1::int[])")) } pub
struct GetKartsByIdsStmt(cornucopia_async :: private :: Stmt) ; impl
GetKartsByIdsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::ArraySql<Item = i32>,>
(& 'a mut self, client : & 'a  C,
ids : & 'a T1,) -> KartQuery < 'a, C,
Kart, 1 >
{
    KartQuery
    {
        client, params : [ids,], stmt : & mut self.0, extractor :
        | row | { KartBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),} }, mapper : | it | { <Kart>::from(it) },
    }
} }pub fn get_kart_by_number() -> GetKartByNumberStmt
{ GetKartByNumberStmt(cornucopia_async :: private :: Stmt :: new("select * from public.cars WHERE number = $1")) } pub
struct GetKartByNumberStmt(cornucopia_async :: private :: Stmt) ; impl
GetKartByNumberStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
number : & 'a i32,) -> KartQuery < 'a, C,
Kart, 1 >
{
    KartQuery
    {
        client, params : [number,], stmt : & mut self.0, extractor :
        | row | { KartBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),} }, mapper : | it | { <Kart>::from(it) },
    }
} }pub fn get_karts_by_numbers() -> GetKartsByNumbersStmt
{ GetKartsByNumbersStmt(cornucopia_async :: private :: Stmt :: new("select * from public.cars WHERE number = any($1::int[])")) } pub
struct GetKartsByNumbersStmt(cornucopia_async :: private :: Stmt) ; impl
GetKartsByNumbersStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::ArraySql<Item = i32>,>
(& 'a mut self, client : & 'a  C,
numbers : & 'a T1,) -> KartQuery < 'a, C,
Kart, 1 >
{
    KartQuery
    {
        client, params : [numbers,], stmt : & mut self.0, extractor :
        | row | { KartBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),} }, mapper : | it | { <Kart>::from(it) },
    }
} }pub fn get_kart_from_lap() -> GetKartFromLapStmt
{ GetKartFromLapStmt(cornucopia_async :: private :: Stmt :: new("SELECT k.* FROM public.cars k
    INNER JOIN public.laps l on k.id = l.kart_id
WHERE l.id = $1")) } pub
struct GetKartFromLapStmt(cornucopia_async :: private :: Stmt) ; impl
GetKartFromLapStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
lap : & 'a i32,) -> KartQuery < 'a, C,
Kart, 1 >
{
    KartQuery
    {
        client, params : [lap,], stmt : & mut self.0, extractor :
        | row | { KartBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),} }, mapper : | it | { <Kart>::from(it) },
    }
} }pub fn get_karts_from_laps() -> GetKartsFromLapsStmt
{ GetKartsFromLapsStmt(cornucopia_async :: private :: Stmt :: new("SELECT k.* FROM public.cars k
    INNER JOIN public.laps l on k.id = l.kart_id
WHERE l.id = any($1::int[])")) } pub
struct GetKartsFromLapsStmt(cornucopia_async :: private :: Stmt) ; impl
GetKartsFromLapsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::ArraySql<Item = i32>,>
(& 'a mut self, client : & 'a  C,
laps : & 'a T1,) -> KartQuery < 'a, C,
Kart, 1 >
{
    KartQuery
    {
        client, params : [laps,], stmt : & mut self.0, extractor :
        | row | { KartBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),} }, mapper : | it | { <Kart>::from(it) },
    }
} }pub fn get_karts_stats_per_day() -> GetKartsStatsPerDayStmt
{ GetKartsStatsPerDayStmt(cornucopia_async :: private :: Stmt :: new("select
    k.*,
    h.start_date,
    min(lap_time) as min_laptime,
    avg(lap_time) as avg_laptime,
    percentile_cont(0.5) WITHIN GROUP (ORDER BY lap_time) as median_laptime
from public.cars k
         inner join public.laps l on k.id = l.kart_id
         inner join public.session h on h.id = l.heat
group by k.id, k.number, h.start_date")) } pub
struct GetKartsStatsPerDayStmt(cornucopia_async :: private :: Stmt) ; impl
GetKartsStatsPerDayStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> GetKartsStatsPerDayQuery < 'a, C,
GetKartsStatsPerDay, 0 >
{
    GetKartsStatsPerDayQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { GetKartsStatsPerDayBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),start_date : row.get(6),min_laptime : row.get(7),avg_laptime : row.get(8),median_laptime : row.get(9),} }, mapper : | it | { <GetKartsStatsPerDay>::from(it) },
    }
} }pub fn get_kart_with_stats() -> GetKartWithStatsStmt
{ GetKartWithStatsStmt(cornucopia_async :: private :: Stmt :: new("select
    k.*,
    CAST(count(l.id) AS INT) as lap_count,
    CAST(count(DISTINCT l.driver) AS INT) as driver_count
from public.cars k
inner join public.laps l on k.id = l.kart_id
where k.number = $1
group by k.id")) } pub
struct GetKartWithStatsStmt(cornucopia_async :: private :: Stmt) ; impl
GetKartWithStatsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
kart_number : & 'a i32,) -> KartWithStatsQuery < 'a, C,
KartWithStats, 1 >
{
    KartWithStatsQuery
    {
        client, params : [kart_number,], stmt : & mut self.0, extractor :
        | row | { KartWithStatsBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),lap_count : row.get(6),driver_count : row.get(7),} }, mapper : | it | { <KartWithStats>::from(it) },
    }
} }pub fn get_all_karts_with_stats() -> GetAllKartsWithStatsStmt
{ GetAllKartsWithStatsStmt(cornucopia_async :: private :: Stmt :: new("select
    k.*,
    CAST(count(l.id) AS INT) as lap_count,
    CAST(count(DISTINCT l.driver) AS INT) as driver_count
from public.cars k
         inner join public.laps l on k.id = l.kart_id
group by k.id
order by $1")) } pub
struct GetAllKartsWithStatsStmt(cornucopia_async :: private :: Stmt) ; impl
GetAllKartsWithStatsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
order_column : & 'a T1,) -> KartWithStatsQuery < 'a, C,
KartWithStats, 1 >
{
    KartWithStatsQuery
    {
        client, params : [order_column,], stmt : & mut self.0, extractor :
        | row | { KartWithStatsBorrowed { id : row.get(0),number : row.get(1),brand : row.get(2),model : row.get(3),horsepower : row.get(4),modified : row.get(5),lap_count : row.get(6),driver_count : row.get(7),} }, mapper : | it | { <KartWithStats>::from(it) },
    }
} }pub fn create_kart() -> CreateKartStmt
{ CreateKartStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO public.cars (number, brand, model, horsepower, modified) values ($1, $2, $3, $4, $5) RETURNING id")) } pub
struct CreateKartStmt(cornucopia_async :: private :: Stmt) ; impl
CreateKartStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
number : & 'a i32,brand : & 'a T1,model : & 'a T2,horsepower : & 'a i32,modified : & 'a bool,) -> I32Query < 'a, C,
i32, 5 >
{
    I32Query
    {
        client, params : [number,brand,model,horsepower,modified,], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,> cornucopia_async ::
Params < 'a, CreateKartParams < T1,T2,>, I32Query < 'a,
C, i32, 5 >, C > for CreateKartStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    CreateKartParams < T1,T2,>) -> I32Query < 'a, C,
    i32, 5 >
    { self.bind(client, & params.number,& params.brand,& params.model,& params.horsepower,& params.modified,) }
}}pub mod laps
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertLapsBulkParams < T1 : cornucopia_async::ArraySql<Item = i32>,T2 : cornucopia_async::ArraySql<Item = i32>,T3 : cornucopia_async::ArraySql<Item = i32>,T4 : cornucopia_async::ArraySql<Item = f64>,T5 : cornucopia_async::ArraySql<Item = i32>,> { pub heats : T1,pub drivers : T2,pub laps_in_heat : T3,pub lap_times : T4,pub karts : T5,}#[derive(Clone,Copy, Debug)] pub struct InsertLapParams < > { pub heat : i32,pub driver : i32,pub lap_in_heat : i32,pub laptime : f64,pub kart_id : i32,}#[derive( Debug, Clone, PartialEq, Copy)] pub struct Lap
{ pub id : i32,pub heat : i32,pub driver : i32,pub lap_in_heat : i32,pub lap_time : f64,pub kart_id : i32,}pub struct LapQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> Lap,
    mapper : fn(Lap) -> T,
} impl < 'a, C, T : 'a, const N : usize > LapQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(Lap) -> R) -> LapQuery
    < 'a, C, R, N >
    {
        LapQuery
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
}pub fn get_lap() -> GetLapStmt
{ GetLapStmt(cornucopia_async :: private :: Stmt :: new("select * from public.laps where id = $1")) } pub
struct GetLapStmt(cornucopia_async :: private :: Stmt) ; impl
GetLapStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
id : & 'a i32,) -> LapQuery < 'a, C,
Lap, 1 >
{
    LapQuery
    {
        client, params : [id,], stmt : & mut self.0, extractor :
        | row | { Lap { id : row.get(0),heat : row.get(1),driver : row.get(2),lap_in_heat : row.get(3),lap_time : row.get(4),kart_id : row.get(5),} }, mapper : | it | { <Lap>::from(it) },
    }
} }pub fn get_laps_from_driver() -> GetLapsFromDriverStmt
{ GetLapsFromDriverStmt(cornucopia_async :: private :: Stmt :: new("select * from public.laps where driver = $1")) } pub
struct GetLapsFromDriverStmt(cornucopia_async :: private :: Stmt) ; impl
GetLapsFromDriverStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
driver : & 'a i32,) -> LapQuery < 'a, C,
Lap, 1 >
{
    LapQuery
    {
        client, params : [driver,], stmt : & mut self.0, extractor :
        | row | { Lap { id : row.get(0),heat : row.get(1),driver : row.get(2),lap_in_heat : row.get(3),lap_time : row.get(4),kart_id : row.get(5),} }, mapper : | it | { <Lap>::from(it) },
    }
} }pub fn get_laps_from_kart() -> GetLapsFromKartStmt
{ GetLapsFromKartStmt(cornucopia_async :: private :: Stmt :: new("select * from public.laps where kart_id = $1")) } pub
struct GetLapsFromKartStmt(cornucopia_async :: private :: Stmt) ; impl
GetLapsFromKartStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
kart : & 'a i32,) -> LapQuery < 'a, C,
Lap, 1 >
{
    LapQuery
    {
        client, params : [kart,], stmt : & mut self.0, extractor :
        | row | { Lap { id : row.get(0),heat : row.get(1),driver : row.get(2),lap_in_heat : row.get(3),lap_time : row.get(4),kart_id : row.get(5),} }, mapper : | it | { <Lap>::from(it) },
    }
} }pub fn get_laps_from_drivers() -> GetLapsFromDriversStmt
{ GetLapsFromDriversStmt(cornucopia_async :: private :: Stmt :: new("select * from public.laps where driver = any($1::int[])")) } pub
struct GetLapsFromDriversStmt(cornucopia_async :: private :: Stmt) ; impl
GetLapsFromDriversStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::ArraySql<Item = i32>,>
(& 'a mut self, client : & 'a  C,
driver : & 'a T1,) -> LapQuery < 'a, C,
Lap, 1 >
{
    LapQuery
    {
        client, params : [driver,], stmt : & mut self.0, extractor :
        | row | { Lap { id : row.get(0),heat : row.get(1),driver : row.get(2),lap_in_heat : row.get(3),lap_time : row.get(4),kart_id : row.get(5),} }, mapper : | it | { <Lap>::from(it) },
    }
} }pub fn get_laps_from_heat() -> GetLapsFromHeatStmt
{ GetLapsFromHeatStmt(cornucopia_async :: private :: Stmt :: new("select * from public.laps where heat = $1")) } pub
struct GetLapsFromHeatStmt(cornucopia_async :: private :: Stmt) ; impl
GetLapsFromHeatStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
heat : & 'a i32,) -> LapQuery < 'a, C,
Lap, 1 >
{
    LapQuery
    {
        client, params : [heat,], stmt : & mut self.0, extractor :
        | row | { Lap { id : row.get(0),heat : row.get(1),driver : row.get(2),lap_in_heat : row.get(3),lap_time : row.get(4),kart_id : row.get(5),} }, mapper : | it | { <Lap>::from(it) },
    }
} }pub fn get_laps_from_heats() -> GetLapsFromHeatsStmt
{ GetLapsFromHeatsStmt(cornucopia_async :: private :: Stmt :: new("select * from public.laps where heat = any($1::int[])")) } pub
struct GetLapsFromHeatsStmt(cornucopia_async :: private :: Stmt) ; impl
GetLapsFromHeatsStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::ArraySql<Item = i32>,>
(& 'a mut self, client : & 'a  C,
heats : & 'a T1,) -> LapQuery < 'a, C,
Lap, 1 >
{
    LapQuery
    {
        client, params : [heats,], stmt : & mut self.0, extractor :
        | row | { Lap { id : row.get(0),heat : row.get(1),driver : row.get(2),lap_in_heat : row.get(3),lap_time : row.get(4),kart_id : row.get(5),} }, mapper : | it | { <Lap>::from(it) },
    }
} }pub fn delete_laps_from_heat() -> DeleteLapsFromHeatStmt
{ DeleteLapsFromHeatStmt(cornucopia_async :: private :: Stmt :: new("delete from public.laps where heat = $1")) } pub
struct DeleteLapsFromHeatStmt(cornucopia_async :: private :: Stmt) ; impl
DeleteLapsFromHeatStmt { pub async fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
heat : & 'a i32,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [heat,]) .await
} }pub fn insert_laps_bulk() -> InsertLapsBulkStmt
{ InsertLapsBulkStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO
    public.laps (heat, driver, lap_in_heat, lap_time, kart_id)
select
    unnest($1::int[]),
    unnest($2::int[]),
    unnest($3::int[]),
    unnest($4::float[]),
    unnest($5::int[])
returning *")) } pub
struct InsertLapsBulkStmt(cornucopia_async :: private :: Stmt) ; impl
InsertLapsBulkStmt { pub fn bind < 'a, C : GenericClient, T1 : cornucopia_async::ArraySql<Item = i32>,T2 : cornucopia_async::ArraySql<Item = i32>,T3 : cornucopia_async::ArraySql<Item = i32>,T4 : cornucopia_async::ArraySql<Item = f64>,T5 : cornucopia_async::ArraySql<Item = i32>,>
(& 'a mut self, client : & 'a  C,
heats : & 'a T1,drivers : & 'a T2,laps_in_heat : & 'a T3,lap_times : & 'a T4,karts : & 'a T5,) -> LapQuery < 'a, C,
Lap, 5 >
{
    LapQuery
    {
        client, params : [heats,drivers,laps_in_heat,lap_times,karts,], stmt : & mut self.0, extractor :
        | row | { Lap { id : row.get(0),heat : row.get(1),driver : row.get(2),lap_in_heat : row.get(3),lap_time : row.get(4),kart_id : row.get(5),} }, mapper : | it | { <Lap>::from(it) },
    }
} }impl < 'a, C : GenericClient, T1 : cornucopia_async::ArraySql<Item = i32>,T2 : cornucopia_async::ArraySql<Item = i32>,T3 : cornucopia_async::ArraySql<Item = i32>,T4 : cornucopia_async::ArraySql<Item = f64>,T5 : cornucopia_async::ArraySql<Item = i32>,> cornucopia_async ::
Params < 'a, InsertLapsBulkParams < T1,T2,T3,T4,T5,>, LapQuery < 'a,
C, Lap, 5 >, C > for InsertLapsBulkStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertLapsBulkParams < T1,T2,T3,T4,T5,>) -> LapQuery < 'a, C,
    Lap, 5 >
    { self.bind(client, & params.heats,& params.drivers,& params.laps_in_heat,& params.lap_times,& params.karts,) }
}pub fn insert_lap() -> InsertLapStmt
{ InsertLapStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO
    public.laps (heat, driver, lap_in_heat, lap_time, kart_id)
VALUES ($1, $2, $3, $4, $5)
returning *")) } pub
struct InsertLapStmt(cornucopia_async :: private :: Stmt) ; impl
InsertLapStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
heat : & 'a i32,driver : & 'a i32,lap_in_heat : & 'a i32,laptime : & 'a f64,kart_id : & 'a i32,) -> LapQuery < 'a, C,
Lap, 5 >
{
    LapQuery
    {
        client, params : [heat,driver,lap_in_heat,laptime,kart_id,], stmt : & mut self.0, extractor :
        | row | { Lap { id : row.get(0),heat : row.get(1),driver : row.get(2),lap_in_heat : row.get(3),lap_time : row.get(4),kart_id : row.get(5),} }, mapper : | it | { <Lap>::from(it) },
    }
} }impl < 'a, C : GenericClient, > cornucopia_async ::
Params < 'a, InsertLapParams < >, LapQuery < 'a,
C, Lap, 5 >, C > for InsertLapStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertLapParams < >) -> LapQuery < 'a, C,
    Lap, 5 >
    { self.bind(client, & params.heat,& params.driver,& params.lap_in_heat,& params.laptime,& params.kart_id,) }
}}}