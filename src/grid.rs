pub mod grid_config
{
	pub const SCREEN_WIDTH:u32 = 500;
	pub const SCREEN_HEIGHT:u32 = 500;
	pub const CELL_SIZE:u32 = 4;
	pub const COLUMN_COUNT:u32 = SCREEN_WIDTH / CELL_SIZE;
	pub const ROW_COUNT:u32 = SCREEN_HEIGHT / CELL_SIZE;
}
