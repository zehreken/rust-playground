struct Point
{
	x:i32,
	y:i32,
}

struct Cell
{
	position:Point,
	neighbours:[Point, 8],
	current_state:i32,
	future_state:i32,
}

pub mod cell
{
	fn setPosition(cell: Cell, row:i32, column:i32)
	{

	}
}
