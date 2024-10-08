#[cfg(test)]
mod tests{
    use sh::sh;
    
    #[test]
    fn test_sh() {
        let world = "world";
        let mut out = String::new();
        sh!(echo hello {world} > {&mut out});
        assert_eq!(out, "hello world\n");
        
        // We can also pipe a String/&str or Vec<u8>/&[u8] to a command
        out.clear();
        let input = "foo bar baz";
        sh!(cat < {input} > {&mut out});
        assert_eq!(&out, input);
        
        // We can execute many commands at once
        let mut out1 = String::new();
        let mut out2 = String::new();
        let mut out3 = String::new();
        
        sh! {
          echo hello world 1 > {&mut out1}; // Note the `;`
          echo hello world 2 > {&mut out2};
          echo hello world 3 > {&mut out3};
        }
        
        assert_eq!(&out1, "hello world 1\n");
        assert_eq!(&out2, "hello world 2\n");
        assert_eq!(&out3, "hello world 3\n");

    }
}