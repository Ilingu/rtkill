use tui::{
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
};

use crate::utils::generate_random_color;

const LOGO: [&str; 7] = [
    r" ________     
|\   __  \    
\ \  \|\  \   
 \ \   _  _\  
  \ \  \\  \| 
   \ \__\\ _\ 
    \|__|\|__|",
    r" _________   
|\___   ___\ 
\|___ \  \_| 
     \ \  \  
      \ \  \ 
       \ \__\
        \|__|",
    r"          
          
          
          
          
          
          ",
    r" ___  __       
|\  \|\  \     
\ \  \/  /|_   
 \ \   ___  \  
  \ \  \\ \  \ 
   \ \__\\ \__\
    \|__| \|__|",
    r" ___     
|\  \    
\ \  \   
 \ \  \  
  \ \  \ 
   \ \__\
    \|__|",
    r" ___          
|\  \         
\ \  \        
 \ \  \       
  \ \  \____  
   \ \_______\
    \|_______|",
    r" ___          
|\  \         
\ \  \        
 \ \  \       
  \ \  \____  
   \ \_______\
    \|_______|",
];

/// build the app logo with random colors and returns the ui component containing it
///
/// may be improved by storing/caching the "colored_text_chunks" in a const (lazy_static)
pub fn welcome_logo() -> Paragraph<'static> {
    let colors_by_char_id = LOGO
        .iter()
        .map(|_| {
            Style::default()
                .fg(generate_random_color())
                .add_modifier(Modifier::BOLD)
        })
        .collect::<Vec<_>>();

    let mut colored_text_chunks: Vec<Vec<Span>> = vec![vec![Span::raw(""); 7]; 7];
    for (char_id, char) in LOGO.iter().enumerate() {
        for (line_id, line) in char.lines().enumerate() {
            colored_text_chunks[line_id][char_id] =
                Span::styled(line.to_string(), colors_by_char_id[char_id])
        }
    }

    let colored_text = colored_text_chunks
        .into_iter()
        .map(Spans::from)
        .collect::<Vec<_>>();
    Paragraph::new(colored_text)
}
