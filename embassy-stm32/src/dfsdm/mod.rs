//! Discrete Filter for Delta Sigma Converter (DFSDM)
//! check if i can push
#![macro_use]

mod _version;
use embassy_hal_internal::{Peri,SetConfig};
pub mod defines;
use crate::time::Hertz;
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;


pub enum HAL_DFSDM_Channel_StateTypeDef{
    DFSDM_CHANNEL_STATE_REST = 0x00,
    DFSDM_CHANNEL_STATE_READY = 0x01,
    DFSDM_CHANNEL_STATE_ERROR = 0xFF,
}
struct DFSDM_Channel_AwdTypeDef{
    FilterOrder: u32,
    Oversampling: u32,
}

struct DFSDM_Channel_SerialInterfaceTypeDef{
    Type: u32,
    SpiClock: u32,
}

struct DFSDM_Channel_InputTypeDef{
    Multiplexer: u32,
    DataPacking: u32,
    Pins: u32,
}

struct DFSDM_Channel_OutputClockTypeDef{
    Activation: bool,
    Selection: u32,
    Divider: u32
}


struct DFSDM_Channel_InitTypeDef {
    OutputClock: DFSDM_Channel_OutputClockTypeDef,
    Input: DFSDM_Channel_InputTypeDef,
    SerialInterface: DFSDM_Channel_SerialInterfaceTypeDef,
    Awd: DFSDM_Channel_AwdTypeDef,
    Offset: u32,
    RightBitShift: u32
}


pub struct DFSDM_Channel_TypeDef{
    pub CHCFGR1: u32,
    pub CHCFGR2: u32,
    pub CHAWSCDR: u32,
    pub CHWDATAR: u32,
    pub CHDATINR: u32,
}

pub struct Config{
    pub frequency: Hertz
}

pub struct DfsdmChannelHandleTypeDef {
    pub dfsdm_instance: DFSDM_Channel_TypeDef,
    pub Init: DFSDM_Channel_InitTypeDef,
    pub State: HAL_DFSDM_Channel_StateTypeDef,

}
// Memory map of DFSDM1 = 0x4001 6000 - 0x4000 63FF
impl<'a>DfsdmChannelHandleTypeDef<'a>{

    pub fn new<T:Instance>(){

    }

    const F_32: u32 = 0xFFFFFFFF;


    pub fn GPIO_INIT(&mut self){
        //INIT GPIO A PIN

        let GPIO_PIN_A_MODE  = 0x48000000;
        let mut GPIO_PIN_A_MODE_DATA = (GPIO_PIN_A_MODE as *const u32).readvolatile();
        const GPIO_MODE_MASK: u32 = 0x3;
        const GPIO_PIN_5: u32 = 0x0020;
        //clear the mode pins
        GPIO_PIN_A_MODE_DATA &= !(GPIO_MODE_MASK << GPIO_PIN_5);

        GPIO_PIN_A_MODE_DATA |= (GPIO_MODE_AF_PP << GPIO_PIN_5);
        //mode
        const GPIO_MODE_PP: u32 = 0x0<<4;
        const GPIO_MODE_AF: u32 = 0x2<<0;
        const GPIO_MODE_AF_PP: u32 = GPIO_MODE_PP|GPIO_MODE_AF;
        (GPIO_PIN_A_MODE as *mut u32).write_volatile(GPIO_PIN_A_MODE_DATA);

        //no pull
        const GPIO_NO_PULL: u32 = 0x0;

        //sset the speed as slow
        let GPIO_PIN_A_SPEED  = 0x48000008;
        let mut GPIO_PIN_A_SPEED_DATA = (GPIO_PIN_A_SPEED as *const u32).readvolatile();
        const SPEED_FREQ_MASK: u32 = 0x3;
        const GPIO_SPEED_FREQ_LOW: u32= 0x0;
        GPIO_PIN_A_SPEED_DATA &= !((SPEED_FREQ_MASK)<<GPIO_PIN_5);
        (GPIO_PIN_A_SPEED as *mut u32).write_volatile(GPIO_PIN_A_SPEED_DATA);

        let GPIO_PIN_A_AF  = 0x48000020;
        let mut GPIO_PIN_A_AF_DATA = (GPIO_PIN_A_AF as *const u32).readvolatile();
        const GPIO_AF_MASK: u32 = 0xF;
        const GPIO_AF6_DFSDM1: u32 = 0x6;
        GPIO_PIN_A_AF_DATA &= !((GPIO_AF_MASK)<<20);
        GPIO_PIN_A_AF_DATA |= GPIO_AF6_DFSDM1 << 20;
        (GPIO_PIN_A_AF as *mut u32).write_volatile(GPIO_PIN_A_AF_DATA);


        let GPIO_PIN_B_MODE  = 0x48000400;
        let mut GPIO_PIN_A_MODE_DATA = (GPIO_PIN_A_MODE as *const u32).readvolatile();
        const GPIO_MODE_MASK: u32 = 0x3;
        const GPIO_PIN_1: u32 = 0x002;
        //clear the mode pins
        GPIO_PIN_A_MODE_DATA &= !(GPIO_MODE_MASK << GPIO_PIN_1);

        GPIO_PIN_A_MODE_DATA |= (GPIO_MODE_AF_PP << GPIO_PIN_1);
        //mode
        const GPIO_MODE_PP: u32 = 0x0<<4;
        const GPIO_MODE_AF: u32 = 0x2<<0;
        const GPIO_MODE_AF_PP: u32 = GPIO_MODE_PP|GPIO_MODE_AF;
        (GPIO_PIN_A_MODE as *mut u32).write_volatile(GPIO_PIN_A_MODE_DATA);

        //no pull
        const GPIO_NO_PULL: u32 = 0x0;

        //sset the speed as slow
        let GPIO_PIN_A_SPEED  = 0x48000408;
        let mut GPIO_PIN_A_SPEED_DATA = (GPIO_PIN_A_SPEED as *const u32).readvolatile();
        const SPEED_FREQ_MASK: u32 = 0x3;
        const GPIO_SPEED_FREQ_LOW: u32= 0x0;
        GPIO_PIN_A_SPEED_DATA &= !((SPEED_FREQ_MASK)<<GPIO_PIN_1);
        (GPIO_PIN_A_SPEED as *mut u32).write_volatile(GPIO_PIN_A_SPEED_DATA);

        let GPIO_PIN_A_AF  = 0x48000420;
        let mut GPIO_PIN_A_AF_DATA = (GPIO_PIN_A_AF as *const u32).readvolatile();
        const GPIO_AF_MASK: u32 = 0xF;
        const GPIO_AF6_DFSDM1: u32 = 0x6;
        GPIO_PIN_A_AF_DATA &= !((GPIO_AF_MASK)<<5);
        GPIO_PIN_A_AF_DATA |= GPIO_AF6_DFSDM1 << 5;
        (GPIO_PIN_A_AF as *mut u32).write_volatile(GPIO_PIN_A_AF_DATA);


    }
    pub fn DFSM_Clock_Select( &mut self) {
        //clear the bit for dfsdm
        let RCC_CCIPR = 0x88;
        let mut RCC_CCIPR_DATA = (RCC_CCIPR as *const u32).readvolatile();
        const RCC_DFSDM1CLKSOURCE_PCLK: u64 = 0x0;
        const RCC_CCIPR_DFSDM1SELL_CLEAR: u64 = 0x80000000;
        RCC_CCIPR_DATA &= (RCC_CCIPR_DFSDM1SELL_CLEAR ^ F_32);
        //select the clock mux as PCLK
        RCC_CCIPR_DATA |= RCC_DFSDM1CLKSOURCE_PCLK;

        (RCC_CCIPR as *mut u32).write_volatile(RCC_CCIPR_DATA);


        //bit to enable the clock
        let RCC_APB2ENR = 0x60;
        let mut RCC_APB2ENR_DATA = (RCC_APB2ENR as *const u32).readvolatile();
        const RCC_APB2ENR_DFSDM1EN: u64 = 0x01000000;
        //enable the clock
        RCC_APB2ENR_DATA |= RCC_APB2ENR_DFSDM1EN;

        (RCC_APB2ENR as *mut u32).write_volatile(RCC_APB2ENR_DATA);
    }


    pub fn DFSDM_FilterInit( &mut self) {

        //FLTCR1 = 0x40016000 = + 0x100 + 0x80*x x = 0
        FLTCR1 = 0x40016100;
        let mut FLTCR1_DATA = (FLTCR1 as *const u32).readvolatile();
        const DFSDM_FLTCR1_RSYNC: u64 = 0x00080000;
        FLTCR1_DATA &= (DFSDM_FLTCR1_RSYNC^ F_32);
        const DFSDM_FLTCR1_FAST: u64 = 0x20000000;
        FLTCR1_DATA &= (DFSDM_FLTCR1_FAST ^ F_32); //fast mode disabled
        const DFSDM_FLTCR1_RDMAEN: u64 = 0x00200000;
        FLTCR1_DATA &= (DFSDM_FLTCR1_RDMAEN ^ F_32);


        //clear all injected params
        const DFSDM_FLTCR1_JSYNC: u64 = 0x00000008;
        const DFSDM_FLTCR1_JEXTEN: u64 = 0x00006000;
        const DFSDM_FLTCR1_JEXTSEL: u64 = 0x00000700;
        FLTCR1_DATA &= !(DFSDM_FLTCR1_JSYNC | DFSDM_FLTCR1_JEXTEN | DFSDM_FLTCR1_JEXTSEL);

        const DFSDM_FLTCR1_JSCAN: u64 = 0x00000010;
        FLTCR1_DATA &=  !DFSDM_FLTCR1_JSCAN;
        const DFSDM_FLTCR1_JDMAEN: u64 = 0x00000020;
        FLTCR1_DATA != !(DFSDM_FLTCR1_JDMAEN);

        (FLTCR1 as *mut u32).write_volatile(FLTCR1_DATA);



        //FLTxFCR = 0x40016000 + 0x114 + 0x80*x x= 0
        let FLT0FCR = 0x40016114;
        //clear the paramters
        let mut FLT0FCR_DATA = (FLT0FCR as *const u32).readvolatile();
        const DFSDM_FLTFCR_FORD: u64 = 0xE0000000;
        const DFSDM_FLTFCR_FOSR: u64 = 0x03FF0000;
        const DFSDM_FLTFCR_IOSR: u64 = 0x000000FF;
        FLT0FCR_DATA &= !(DFSDM_FLTFCR_FORD | DFSDM_FLTFCR_FOSR | DFSDM_FLTFCR_IOSR);
        //inject the parameters
        const DFSDM_FILTER_FASTSINC_ORDER: u64 = 0x0;
        const OVERSAMPLING: u64 = 100;
        const IntOversampling: u64 = 70;
        const DFSDM_FLTFCR_FOSR_Pos: u64 = 16;
        FLT0FCR_DATA |= (DFSDM_FILTER_FASTSINC_ORDER | ((OVERSAMPLING - 1) <<DFSDM_FLTFCR_FOSR_Pos) | (IntOversampling - 1));

        (FLT0FCR as *mut u32).write_volatile(FLT0FCR_DATA);




        //enable DFSDM Filter
        const DFSDM_FLTCR1_DFEN: u64 = 0x00000001;

        FLTCR1_DATA = (FLTCR1 as *const u32).readvolatile();
        FLTCR1_DATA |= DFSDM_FLTCR1_DFEN;
        (FLTCR1 as *mut u32).write_volatile(FLTCR1_DATA);

    }
    pub fn DFSDM_ChannelInnit(&mut self,){
        // Need to set the clock for it

        //CHyCFGR1 = 0x40016000 + 0x00 + 0x20 * y
        let CH0CFGR1 = 0x40016000;
        //reset clock mask
        CH0CFGR1_DATA = (CH0CFGR1 as *const u32).readvolatile();
        const DFSDM_CHCFGR1_CKOUTDIV: u64 = 0x40000000;
        const DFSDM_CHANNEL_OUTPUT_CLOCK_SYSTEM:  u64 = 0x0;
        CH0CFGR1_DATA &= !(DFSDM_CHCFGR1_CKOUTDIV);
        CH0CFGR1_DATA |= DFSDM_CHANNEL_OUTPUT_CLOCK_SYSTEM;

        //reset clock divider
        const DFSDM_CHCFGR1_CKOUTDIV: u64 = 0x00FF0000;
        const DIVIDER: u64 = 0x2;
        const DIVIDER_SHIFT: u32 = 16;
        CH0CFGR1_DATA &= !(DFSDM_CHCFGR1_CKOUTDIV);
        CH0CFGR1_DATA |= ((DIVIDER- 1) << DIVIDER_SHIFT);
        const DFSDM_CHCFGR1_DFSDMEN: u64 = 0x80000000;
        //enable the global interface
        CH0CFGR1_DATA |= DFSDM_CHCFGR1_DFSDMEN;


        //clear channel input paramters
        const DFSDM_CHCFGR1_DATPACK: u64 = 0x0000C000;
        const DFSDM_CHCFGR1_DATMPX: u64 = 0x00003000;
        const DFSDM_CHCFGR1_CHINSEL: u64 = 0x00000100;
        CH0CFGR1_DATA &= !(DFSDM_CHCFGR1_DATPACK | DFSDM_CHCFGR1_DATMPX | DFSDM_CHCFGR1_CHINSEL);
        //set the input params
        const DFSDM_CHANNEL_EXTERNAL_INPUTS: u64 = 0x0;
        const DFSDM_CHANNEL_STANDARD_MODE: u64 = 0x0;
        const DFSDM_CHANNEL_SAME_CHANNEL_PINS: u64 = 0x0;
        CH0CFGR1_DATA = DFSDM_CHANNEL_EXTERNAL_INPUTS | DFSDM_CHANNEL_STANDARD_MODE | DFSDM_CHANNEL_SAME_CHANNEL_PINS;

        //set serial interface params
        const DFSDM_CHCFGR1_SITP: u64 = 0x00000003;
        const DFSDM_CHCFGR1_SPICKSEL: u64 = 0x0000000C;
        CH0CFGR1_DATA &= !(DFSDM_CHCFGR1_SITP | DFSDM_CHCFGR1_SPICKSEL);

        const DFSDM_CHANNEL_SPI_RISING: u64 = 0x0;
        const DFSDM_CHANNEL_SPI_CLOCK_INTERNAL: u64 = 0x00000004;
        CH0CFGR1_DATA |= (DFSDM_CHANNEL_SPI_RISING |DFSDM_CHANNEL_SPI_CLOCK_INTERNAL);


        (CH0CFGR1 as *mut u32).write_volatile(CH0CFGR1_DATA);
        //analog watchdog
        //CHyAWSCDR = 0x40016000 + 0x08 + 0x20*y
        let CH0AWSCDR = 0x40016008;
        let mut CH0AWSCDR_DATA = (CH0AWSCDR as *const u32).read_volatile();
        const DFSDM_CHAWSCDR_AWFORD: u64 = 0x00C00000;
        const DFSDM_CHAWSCDR_AWFOSR: u64 = 0x001F0000;
        CH0AWSCDR_DATA &= !(DFSDM_CHAWSCDR_AWFORD | DFSDM_CHAWSCDR_AWFOSR);
        const DFSDM_CHANNEL_FASTSINC_ORDER: u64 = 0x0;
        const OVERSAMPLING: u64 = 0x1;
        const DFSDM_CHAWSCDR_AWFOSR_Pos: u64 = 16;
        CH0AWSCDR_DATA |= (DFSDM_CHANNEL_FASTSINC_ORDER |
            ((OVERSAMPLING - 1) << DFSDM_CHAWSCDR_AWFOSR_Pos));


        //CHyCFGR2 = 0x40016000 + 0x04 + 0x20 * y, (y = 0 to 7)
        let CH0CFGR2 = 0x40016004;
        let mut CH0CFGR2_DATA = (CH0CFGR2 as *const u32).read_volatile();
        const DFSDM_CHCFGR2_OFFSET: u64 = 0x001F0000;
        const DFSDM_CHCFGR2_DTRBS: u64 = 0x000000F8;
        //offset and right bit shift
        CH0CFGR2_DATA &= !(DFSDM_CHCFGR2_OFFSET | DFSDM_CHCFGR2_DTRBS);


        const DFSDM_CHCFGR2_OFFSET_Pos: u64 = 8;
        const DFSDM_CHCFGR2_DTRBS_Pos: u64 = 3;
        const OFFSET:  u64 = 0x0;
        const RIGHT_BIT_SHIFT: u64 = 0x0;
        CH0CFGR2_DATA |= (OFFSET << DFSDM_CHCFGR2_OFFSET_Pos) |(RIGHT_BIT_SHIFT << DFSDM_CHCFGR2_DTRBS_Pos);

        (CH0CFGR2 as *mut u32).write_volatile(CH0CFGR2_DATA);
        //CHyCFGR1 = 0x40016000 + 0x00 + 0x20 * y
        //CH0CFGR1 = 0x40016000
        const DFSDM_CHCFGR1_CHEN: u64 = 0x00000080;
        CH0CFGR1_DATA |=DFSDM_CHCFGR1_CHEN;
        (CH0CFGR1 as *mut u32).write_volatile(CH0CFGR1_DATA);
    }


    pub fn HAL_DFSDM_FilterConfig_RegChannel(&mut self){
        //set the filter into contiunous mode

        //FLTyCR1 = 0x40016000 + 0x100 + 0x80 * y
        let FLT0CR1 = 0x40016100;
        let mut FLT0CR1_DATA = (FLT0CR1 as *const u32).readvolatile();
        const DFSDM_FLTCR1_RCH: u64 =0x07000000;
        const DFSDM_FLTCR1_RCONT: u64 =0x00040000;
        //clear the bits first
        FLT0CR1_DATA &= !(DFSDM_FLTCR1_RCH|DFSDM_FLTCR1_RCONT);
        const DFSDM_MSB_MASK: u64 = 0xFFFF0000;
        const Channel: u64 = 0x00000001;
        const DFSDM_FLTCR1_MSB_RCH_OFFSET: u64 = 8;
        const DFSDM_FLTCR1_RCONT: u64 = 0x00040000;
        FLT0CR1_DATA |=  (((Channel & DFSDM_MSB_MASK) << DFSDM_FLTCR1_MSB_RCH_OFFSET) | DFSDM_FLTCR1_RCONT);

        (FLT0CR1 as *mut u32).write_volatile(FLT0CR1_DATA);

    }


    //this will start the DFSDM into regular conversion polling mode, only to be used when filter is
    //in an idle state
    pub fn DFSDM_Filter_Regular_Start(&mut self,) {
        //software start of regular conversion
        //FLTyCR1 = 0x40016000 + 0x100 + 0x80 * y
        let FLT0CR1 = 0x40016100;
        const DFSDM_FLTCR1_RSWSTART: u32 = 0x00020000;
        let mut FLT0CR1_DATA = (FLT0CR1 as *const u32).readvolatile();
        (FLT0CR1 as *mut u32).write_volatile(FLT0CR1_DATA);
    }

    pub fn DFSDM_PollConversion(&mut self,)->bool {
        //FLTxISR = 0x40016000 + 0x108 + 0x80 * y
        let FLT0ISR = 0x40016108;
        let mut FLT0ISR_DATA = (FLT0ISR as *const u32).read_volatile();
        const DFSDM_FLTISR_REOCF: u64 =0x00000002;
        let mut tries = 0;
        while((FLT0ISR_DATA & DFSDM_FLTISR_REOCF) != DFSDM_FLTISR_REOCF)
        {
            Delay.delay_ms(2000);
            tries = tries + 1;
            if(tries == 5) {
                break;
            }
            FLT0ISR_DATA = (FLT0ISR as *const u32).read_volatile();
        }
        const DFSDM_FLTISR_ROVRF: u64 = 0x00000008;
        if (FLT0ISR_DATA & DFSDM_FLTISR_ROVRF == DFSDM_FLTISR_ROVRF){
            return false;
        }
        else{
            return true;
        }

    }

    pub fn DFSDM_Reg_ChannelConversion(&mut self,)->u32 {

        //FLTxRDATAR = 0x40016000 + 0x11C + 0x80 * x, (x = 0 to 3)
        let FLT0RDATAR = 0x4001611C;
        let mut FLT0RDATAR_DATA = (FLT0RDATAR as *const u32).read_volatile();
        const DFSDM_FLTRDATAR_RDATACH: u32 = 0x00000007;
        const DFSDM_FLTRDATAR_RDATA: u32 = 0xFFFFFF00;
        //let mut u32 reg = FLTRDATAR;
        //DFSDM1_Channel0 = reg & DFSDM_FLTRDATAR_RDATACH;

        FLT0RDATAR_DATA &= DFSDM_FLTRDATAR_RDATA;
        FLT0RDATAR_DATA = FLT0RDATAR_DATA/256; //bit shift value down
        return FLT0RDATAR_DATA;


    }
}