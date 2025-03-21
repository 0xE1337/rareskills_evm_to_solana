use anchor_lang::prelude::*;

declare_id!("Dcj7tabphEvhjUyHzYUz6g61qQC4XhLqqfVFQG1SAJhf");

#[program]
pub mod tryrust {
    use super::*;

    // Import HashMap library
    use std::collections::HashMap;

    pub fn initialize(_ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        // Defining a struct in Solana
        struct Person {
            my_name: String,
            my_age: u64,
        }
    
        // Creating an instance of the struct
        let mut person1: Person = Person {
            my_name: name,
            my_age: age,
        };
    
        msg!("{} is {} years old", person1.my_name, person1.my_age);
    
        // Accessing and modifying struct fields
        person1.my_name = "Bob".to_string();
        person1.my_age = 18;
    
        msg!("{} is {} years old", person1.my_name, person1.my_age);
    
        Ok(())
    }

    pub fn age_checker(ctx: Context<Initialize>, age: u64) -> Result<()> {
        match age {
            1 => msg!("The age is 1"),
            2 | 3 => msg!("The age is either 2 or 3"),
            4..=6 => msg!("The age is between 4 and 6"),
            _ => msg!("The age is something else"),
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
