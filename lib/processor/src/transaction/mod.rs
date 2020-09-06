use models::{Request, Response};
use crate::Error;

//TODO: these 2 functions should be one using a simple trait
//also the find and then handle, could be problematic, and could possibly be hidden behind the same
//function?
pub trait TransactionLayer {
    fn find_transaction_for_request(_request: &Request) -> Option<Transaction>;
    fn find_transaction_for_response(_response: &Response) -> Option<Transaction>;
    fn handle_request(&self, _request: Request) -> Result<(), Error>;
    fn handle_response(&self, _response: Response) -> Result<(), Error>;
}

pub struct Transaction;
impl TransactionLayer for Transaction {
    //TODO: these 2 functions should be one using a simple trait
    fn find_transaction_for_request(_request: &Request) -> Option<Transaction> {
        None
    }
    fn find_transaction_for_response(_response: &Response) -> Option<Transaction> {
        None
    }
    fn handle_request(&self, _request: Request) -> Result<(), Error> {
        //Ok(crate::presets::create_unauthorized_from(request)?)
        Ok(())
    }
    fn handle_response(&self, _response: Response) -> Result<(), Error> {
        //Ok(crate::presets::create_unauthorized_from(request)?)
        Ok(())
    }
}

