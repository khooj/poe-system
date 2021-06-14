fn main() {
    let _ = build_info.select(id).load::<CustomStruct>(&conn)?;
}
