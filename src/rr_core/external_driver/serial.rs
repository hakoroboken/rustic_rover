use serialport;

pub struct SerialDriver
{
    is_im920:bool,
    enable_smoother:bool,
    path:String
}