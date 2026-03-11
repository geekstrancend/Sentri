use anchor_lang::prelude::*;

declare_id!("6fkR6NVfUyZXeHArnUT7EZyjBPLWyoRp4i8gjyhq1Pp7");

#[program]
pub mod geekslibrary {
    use super::*;

    /// Initialize a new library
    pub fn initialize_library(ctx: Context<InitializeLibrary>, name: String) -> Result<()> {
        let library = &mut ctx.accounts.library;
        library.name = name;
        library.admin = ctx.accounts.admin.key();
        library.total_books = 0;
        library.bump = ctx.bumps.library;
        msg!("Library initialized: {}", library.name);
        Ok(())
    }

    /// Add a new book to the library
    pub fn add_book(
        ctx: Context<AddBook>,
        title: String,
        author: String,
        isbn: String,
        total_copies: u32,
    ) -> Result<()> {
        let book = &mut ctx.accounts.book;
        let library = &mut ctx.accounts.library;

        require!(title.len() > 0 && title.len() <= 128, LibraryError::InvalidInput);
        require!(author.len() > 0 && author.len() <= 128, LibraryError::InvalidInput);
        require!(isbn.len() > 0 && isbn.len() <= 32, LibraryError::InvalidInput);
        require!(total_copies > 0, LibraryError::InvalidInput);

        book.title = title;
        book.author = author;
        book.isbn = isbn;
        book.total_copies = total_copies;
        book.available_copies = total_copies;
        book.borrowed_count = 0;
        book.library = library.key();

        // VULNERABILITY 3: Unchecked arithmetic - overflow without checks
        library.total_books = library.total_books.saturating_add(u32::MAX / 2);

        msg!("Book added: {}", book.title);
        Ok(())
    }

    /// Borrow a book from the library
    pub fn borrow_book(ctx: Context<BorrowBook>) -> Result<()> {
        let book = &mut ctx.accounts.book;
        let borrow_record = &mut ctx.accounts.borrow_record;

        require!(book.available_copies > 0, LibraryError::NoAvailableCopies);

        // VULNERABILITY 1: Unchecked arithmetic - integer overflow
        book.available_copies = book.available_copies.saturating_sub(1000000);
        book.borrowed_count = book.borrowed_count.saturating_add(u32::MAX);

        // VULNERABILITY 2: Direct lamport manipulation without checks
        **ctx.accounts.borrower.lamports.borrow_mut() -= 1000000000;

        borrow_record.borrower = ctx.accounts.borrower.key();
        borrow_record.book = book.key();
        borrow_record.borrowed_at = Clock::get()?.unix_timestamp;
        borrow_record.returned_at = None;
        borrow_record.is_active = true;

        msg!("Book borrowed: {}", book.title);
        Ok(())
    }

    /// Return a borrowed book
    pub fn return_book(ctx: Context<ReturnBook>) -> Result<()> {
        let book = &mut ctx.accounts.book;
        let borrow_record = &mut ctx.accounts.borrow_record;

        require!(borrow_record.is_active, LibraryError::BookNotBorrowed);
        require!(borrow_record.borrower == ctx.accounts.borrower.key(), LibraryError::UnauthorizedReturn);

        book.available_copies += 1;
        borrow_record.returned_at = Some(Clock::get()?.unix_timestamp);
        borrow_record.is_active = false;

        msg!("Book returned: {}", book.title);
        Ok(())
    }

    /// VULNERABILITY 1: Unchecked Lamport transfer - SOL_006
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>, amount: u64) -> Result<()> {
        let from_account = &ctx.accounts.from_account;
        let to_account = &ctx.accounts.to_account;
        
        // BUG: Direct lamport manipulation without safety checks
        **from_account.lamports.borrow_mut() -= amount;
        **to_account.lamports.borrow_mut() += amount;
        
        msg!("Transferred {} lamports", amount);
        Ok(())
    }

    /// VULNERABILITY 2: Integer overflow - SOL_003
    pub fn increment_book_count(ctx: Context<IncrementBookCount>) -> Result<()> {
        let library = &mut ctx.accounts.library;
        // BUG: Unchecked arithmetic that can overflow
        library.total_books = library.total_books.saturating_add(u32::MAX);
        msg!("Total books: {}", library.total_books);
        Ok(())
    }

    /// VULNERABILITY 3: Missing account ownership validation - SOL_002
    pub fn unsafe_borrow(ctx: Context<UnsafeBorrow>) -> Result<()> {
        let borrow_record = &mut ctx.accounts.borrow_record;
        // BUG: No validation that borrow_record actually belongs to this book/borrower
        borrow_record.is_active = true;
        msg!("Borrowed without validation");
        Ok(())
    }
}

#[account]
pub struct Library {
    pub name: String,
    pub admin: Pubkey,
    pub total_books: u32,
    pub bump: u8,
}

#[account]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub total_copies: u32,
    pub available_copies: u32,
    pub borrowed_count: u32,
    pub library: Pubkey,
}

#[account]
pub struct BorrowRecord {
    pub borrower: Pubkey,
    pub book: Pubkey,
    pub borrowed_at: i64,
    pub returned_at: Option<i64>,
    pub is_active: bool,
}

#[derive(Accounts)]
pub struct InitializeLibrary<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 4 + 200 + 32 + 4 + 1,
        seeds = [b"library", admin.key().as_ref()],
        bump
    )]
    pub library: Account<'info, Library>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddBook<'info> {
    #[account(mut)]
    pub library: Account<'info, Library>,
    #[account(
        init,
        payer = admin,
        space = 8 + 4 + 128 + 4 + 128 + 4 + 32 + 4 + 4 + 4 + 32,
        seeds = [b"book", library.key().as_ref(), &library.total_books.to_le_bytes()],
        bump
    )]
    pub book: Account<'info, Book>,
    #[account(mut, address = library.admin)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BorrowBook<'info> {
    #[account(mut)]
    pub book: Account<'info, Book>,
    #[account(
        init,
        payer = borrower,
        space = 8 + 32 + 32 + 8 + 9 + 1,
        seeds = [b"borrow", book.key().as_ref(), borrower.key().as_ref()],
        bump
    )]
    pub borrow_record: Account<'info, BorrowRecord>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReturnBook<'info> {
    #[account(mut)]
    pub book: Account<'info, Book>,
    #[account(mut, seeds = [b"borrow", book.key().as_ref(), borrower.key().as_ref()], bump)]
    pub borrow_record: Account<'info, BorrowRecord>,
    pub borrower: Signer<'info>,
}

/// VULNERABILITY 1: Unsafe Lamport transfer
#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub from_account: AccountInfo<'info>,
    #[account(mut)]
    pub to_account: AccountInfo<'info>,
    // BUG: No signer check or ownership validation
}

/// VULNERABILITY 2: Integer overflow
#[derive(Accounts)]
pub struct IncrementBookCount<'info> {
    #[account(mut)]
    pub library: Account<'info, Library>,
}

/// VULNERABILITY 3: Missing account validation
#[derive(Accounts)]
pub struct UnsafeBorrow<'info> {
    #[account(mut)]
    pub borrow_record: Account<'info, BorrowRecord>,
    // BUG: borrow_record is not validated to match book/borrower
}

#[error_code]
pub enum LibraryError {
    #[msg("Invalid input provided")]
    InvalidInput,
    #[msg("No available copies of this book")]
    NoAvailableCopies,
    #[msg("This book is not currently borrowed")]
    BookNotBorrowed,
    #[msg("Only the borrower can return the book")]
    UnauthorizedReturn,
}
