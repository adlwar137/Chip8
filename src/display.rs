const SCREEN_HEIGHT: u8 = 32;
const SCREEN_WIDTH: u8 = 64;

pub struct Screen {
    data: [u64; 32],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            data: [0; 32],
        }
    }

    pub fn build(data: [u64; 32]) -> Screen {
        Screen {
            data: data,
        }
    }

    pub fn clear_screen(&mut self) {
        self.data = [0; 32];
    }

    pub fn toggle_pixel(&mut self, xcor: u8, ycor: u8) {
        let shift_left = SCREEN_WIDTH - (xcor % SCREEN_WIDTH) - 1;
        let shift_down = ycor % SCREEN_HEIGHT;
        let lowest: u64 = 1;

        let xor_mask = lowest << shift_left;

        self.data[shift_down as usize] = self.data[shift_down as usize] ^ xor_mask;
    }

    pub fn get_pixel(&self, xcor: u8, ycor: u8) -> bool {
        let shift_left = SCREEN_WIDTH - (xcor % SCREEN_WIDTH) - 1;
        let shift_down = ycor % SCREEN_HEIGHT;
        let lowest: u64 = 1;

        let and_mask = lowest << shift_left;

        let result = self.data[shift_down as usize] & and_mask;
        result > 0
    }

    pub fn print_display(&self) {
        for col in 0..SCREEN_HEIGHT {
            let mut col_out = String::new();
            for row in 0..SCREEN_WIDTH {
                if self.get_pixel(row, col) {
                    col_out.push_str("█");
                } else {
                    col_out.push_str(" ");
                }
            }
            println!("{}", col_out);
        }
    }
}