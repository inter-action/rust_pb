use std::fmt;


#[derive(Debug)]
pub struct Bar<'a> {
    start_delimiter: &'static str, // StartDelimiter for the bar ("|").
    end_delimiter: &'static str,// EndDelimiter for the bar ("|").
    filled: &'static str,// Filled section representation ("█").
    empty: &'static str, // Empty section representation ("░")
    total: f32,// Total value.
    width: i32,// Width of the bar.
    value: f32,
    text: &'a str
}

impl<'a> Bar<'a> {
    pub fn new(total: f32)->Bar<'a>{
        Bar {
            start_delimiter: "|",
            end_delimiter: "|",
            filled: "█",
            empty: "░",
            total,
            width: 60,
            value: 0.0,
            text: ""
        }
    }

    pub fn text(&mut self, text: &'a str) {
        self.text = text;
    }

    pub fn value(&mut self, value: f32) {
        if value > self.total { panic!("Bar update value cannot be greater than the total");}
        self.value = value
    }

    // https://stackoverflow.com/questions/14975661/meaning-of-r-on-linux-systems
    // Control character \r moves caret (a.k.a text cursor) to the leftmost position within current line.
    pub fn write_to(&self, w: &mut std::io::Write)->std::io::Result<usize>{
        let str = format!("\r   {} ", self);
        w.write(str.as_bytes())
    }
}

impl <'a> fmt::Display for Bar<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = self.value / self.total;
        let filled = (p * self.width as f32).ceil() as i32;
        let unfilled = self.width - filled;

        write!(f, "{percentage:3.0}% {sd}{filled}{unfilled}{ed} {txt}", 
            percentage = p * 100.0, 
            sd = self.start_delimiter, 
            filled = self.filled.repeat(filled as usize),
            unfilled =self.empty.repeat(unfilled as usize),
            ed=self.end_delimiter,
            txt=self.text)
    }
}

#[cfg(test)]
mod tests {
    use ::Bar;

    #[test]
    fn test_bar_string() {
        let mut b = Bar::new(1000.0);
        assert_eq!("  0% |░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░| ", format!("{}", b));

        b.value(250.0);
        assert_eq!(" 25% |███████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░| ", format!("{}", b));

        b.value(750.0);
        assert_eq!(" 75% |█████████████████████████████████████████████░░░░░░░░░░░░░░░| ", format!("{}", b));

        b.value(1000.0);
        assert_eq!("100% |████████████████████████████████████████████████████████████| " , format!("{}", b));
    }

    #[test]
    fn test_bar_text() {
        let mut b = Bar::new(1000.0);

        b.text("Building");
        assert_eq!("  0% |░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░| Building", format!("{}", b));

        b.text("Installing");
        b.value(250.0);
        assert_eq!(" 25% |███████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░| Installing", format!("{}", b));
    }
}
