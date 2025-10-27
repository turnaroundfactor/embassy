//! Discrete Filter for Delta Sigma Converter (DFSDM)
//! check if i can push
#![macro_use]

mod _version;
use embassy_hal_internal::{Peri,SetConfig};
pub mod defines;
use crate::rcc::{RccInfo, SealedRccPeripheral};
use crate::gpio::{AnyPin, SealedPin as _};
use crate::mode::Mode as PeriMode;
use crate::time::Hertz;

use embassy_stm32::pac::rcc;


pub(crate) struct Info {
    pub(crate) regs: Regs,
    pub(crate) rcc: RccInfo,
}
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
    pub _peri:Peri<'a,T>,
    pub clk: Peri<'a,AnyPin>,
    pub data: Peri<'a,AnyPin>,

}

impl<'a>DfsdmChannelHandleTypeDef<'a>{

    pub fn new<T:Instance>(
        info: T::info(),
        _peri:Peri<'a,T>,
        clk: Peri<'a,AnyPin>,
        data: Peri<'a,AnyPin>,
    ){

    }
    pub fn DFSM_Clock_Select( &mut self) {
        //clear the bit for dfsdm
        const RCC_DFSDM1CLKSOURCE_PCLK: u64 = 0x0;
        const RCC_CCIPR_DFSDM1SELL_CLEAR: u64 = 0x80000000;
        //RCC->CCIPR &= ~(RCC_CCIPR_DFSDM1SELL_CLEAR);
        //select the clock mux as PCLK
        //RCC->CCIPR |= RCC_DFSDM1CLKSOURCE_PCLK;

        //bit to enable the clock
        const RCC_APB2ENR_DFSDM1EN: u64 = 0x01000000;
        //enable the clock
        //RCC->APB2ENR |= RCC_APB2ENR_DFSDM1EN;
    }


    pub fn DFSDM_FilterInit( &mut self) {
        const DFSDM_FLTCR1_RSYNC: u64 = 0x00080000;
        //FLTCR1 &= ~DFSDM_FLTCR1_RSYNC;
        const DFSDM_FLTCR1_FAST: u64 = 0x20000000;
        //FLTCR1 &= ~DFSDM_FLTCR1_FAST; //fast mode disabled
        const DFSDM_FLTCR1_RDMAEN: u64 = 0x00200000;
        //FLTCR1 &= ~DFSDM_FLTCR1_RDMAEN;


        //clear all injected params
        const DFSDM_FLTCR1_JSYNC: u64 = 0x00000008;
        const DFSDM_FLTCR1_JEXTEN: u64 = 0x00006000;
        const DFSDM_FLTCR1_JEXTSEL: u64 = 0x00000700;
        //FLTCR1 &= ~(DFSDM_FLTCR1_JSYNC | DFSDM_FLTCR1_JEXTEN | DFSDM_FLTCR1_JEXTSEL);

        const DFSDM_FLTCR1_JSCAN: u64 = 0x00000010;
        //FLTCR1 &=  ~DFSDM_FLTCR1_JSCAN;
        const DFSDM_FLTCR1_JDMAEN: u64 = 0x00000020;
        //FLTCR1 &= ~(DFSDM_FLTCR1_JDMAEN);


        //clear the paramters
        const DFSDM_FLTFCR_FORD: u64 = 0xE0000000;
        const DFSDM_FLTFCR_FOSR: u64 = 0x03FF0000;
        const DFSDM_FLTFCR_IOSR: u64 = 0x000000FF;
        //FLTFCR &= ~(DFSDM_FLTFCR_FORD | DFSDM_FLTFCR_FOSR | DFSDM_FLTFCR_IOSR);
        //inject the parameters
        const DFSDM_FILTER_FASTSINC_ORDER: u64 = 0x0;
        const OVERSAMPLING: u64 = 100;
        const IntOversampling: u64 = 70;
        const DFSDM_FLTFCR_FOSR_Pos: u64 = 16;
        //FLTFCR |= (SincOrder | ((OVERSAMPLING - 1) <<DFSDM_FLTFCR_FOSR_Pos) | (IntOversampling - 1);

        //enable DFSDM Filter
        const DFSDM_FLTCR1_DFEN: u64 = 0x00000001;
        //FLTCR1 |= DFSDM_FLTCR1_DFEN;
    }
    pub fn DFSDM_ChannelInnit(&mut self,){
        // Need to set the clock for it
        //reset clock mask
        const DFSDM_CHCFGR1_CKOUTDIV: u64 = 0x40000000;
        const DFSDM_CHANNEL_OUTPUT_CLOCK_SYSTEM:  u64 = 0x0;
        //DFSDM1_Channel0->CHCFGR1 &= ~(DFSDM_CHCFGR1_CKOUTDIV);
        // DFSDM1_Channel0->CHCFGR1 |= DFSDM_CHANNEL_OUTPUT_CLOCK_SYSTEM

        //reset clock divider
        const DFSDM_CHCFGR1_CKOUTDIV: u64 = 0x00FF0000;
        const DIVIDER: u64 = 0x2;
        const DIVIDER_SHIFT: u32 = 16;
        //DFSDM1_channel0->CHCFGR1 &= ~(DFSDM_CHCFGR1_CKOUTDIV);
        //DFSDM1_Channel0->CHCFGR1 |= ((DIVIDER- 1U) <<
        //                                              DIVIDER_SHIFT);
        const DFSDM_CHCFGR1_DFSDMEN: u64 = 0x80000000;
        //enable the global interface
        //DFSDM1_Channel0->CHCFGR1 |= DFSDM_CHCFGR1_DFSDMEN;

        //clear channel input paramters
        const DFSDM_CHCFGR1_DATPACK: u64 = 0x0000C000;
        const DFSDM_CHCFGR1_DATMPX: u64 = 0x00003000;
        const DFSDM_CHCFGR1_CHINSEL: u64 = 0x00000100;
        //CHCFGR1 &= DFSDM_CHCFGR1_DATPACK | DFSDM_CHCFGR1_DATMPX | DFSDM_CHCFGR1_CHINSEL
        //set the input params
        const DFSDM_CHANNEL_EXTERNAL_INPUTS: u64 = 0x0;
        const DFSDM_CHANNEL_STANDARD_MODE: u64 = 0x0;
        const DFSDM_CHANNEL_SAME_CHANNEL_PINS: u64 = 0x0;
        //CHCFGR1 = DFSDM_CHANNEL_EXTERNAL_INPUTS | DFSDM_CHANNEL_STANDARD_MODE | DFSDM_CHANNEL_SAME_CHANNEL_PINS

        //set serial interface params
        const DFSDM_CHCFGR1_SITP: u64 = 0x00000003;
        const DFSDM_CHCFGR1_SPICKSEL: u64 = 0x0000000C;
        //CHCFGR1 &=~(DFSDM_CHCFGR1_SITP | DFSDM_CHCFGR1_SPICKSEL)

        const DFSDM_CHANNEL_SPI_RISING: u64 = 0x0;
        const DFSDM_CHANNEL_SPI_CLOCK_INTERNAL: u64 = 0x00000004;
        //CHCFGR1 |= (DFSDM_CHANNEL_SPI_RISING |DFSDM_CHANNEL_SPI_CLOCK_INTERNAL);

        //analog watchdog
        const DFSDM_CHAWSCDR_AWFORD: u64 = 0x00C00000;
        const DFSDM_CHAWSCDR_AWFOSR: u64 = 0x001F0000;
        //CHAWSCDR &= ~(DFSDM_CHAWSCDR_AWFORD | DFSDM_CHAWSCDR_AWFOSR);
        const DFSDM_CHANNEL_FASTSINC_ORDER: u64 = 0x0;
        const OVERSAMPLING: u64 = 0x1;
        const DFSDM_CHAWSCDR_AWFOSR_Pos: u64 = 16;
        //CHAWSCDR |= (hdfsdm_channel->Init.Awd.FilterOrder |
        //    ((hdfsdm_channel->Init.Awd.Oversampling - 1U) << DFSDM_CHAWSCDR_AWFOSR_Pos));
        const DFSDM_CHCFGR2_OFFSET: u64 = 0x001F0000;
        const DFSDM_CHCFGR2_DTRBS: u64 = 0x000000F8;
        //offset and right bit shift
        //CHCFGR2 &= ~(DFSDM_CHCFGR2_OFFSET | DFSDM_CHCFGR2_DTRBS);

        const DFSDM_CHCFGR2_OFFSET_Pos: u64 = 8;
        const DFSDM_CHCFGR2_DTRBS_Pos: u64 = 3;
        const OFFSET:  u64 = 0x0;
        const RIGHT_BIT_SHIFT: u64 = 0x0;
        //CHCFGR2 |= (OFFSET << DFSDM_CHCFGR2_OFFSET_Pos) |(RIGHT_BIT_SHIFT << DFSDM_CHCFGR2_DTRBS_Pos);

        const DFSDM_CHCFGR1_CHEN: u64 = 0x00000080;
        //CHCFGR1 |=DFSDM_CHCFGR1_CHEN;
    }


    pub fn HAL_DFSDM_FilterConfig_RegChannel(&mut self){
        //set the filter into contiunous mode
        const DFSDM_FLTCR1_RCH: u64 =0x07000000;
        const DFSDM_FLTCR1_RCONT: u64 =0x00040000;
        //clear the bits first
        //FLTCR1 &= ~(DFSDM_FLTCR1_RCH|DFSDM_FLTCR1_RCONT);
        const DFSDM_MSB_MASK: u64 = 0xFFFF0000;
        const Channel: u64 = 0x00000001;
        const DFSDM_FLTCR1_MSB_RCH_OFFSET: u64 = 8;
        const DFSDM_FLTCR1_RCONT: u64 = 0x00040000;
        //FLTCR1 |=  (((Channel & DFSDM_MSB_MASK) << DFSDM_FLTCR1_MSB_RCH_OFFSET) |
        //                                                     DFSDM_FLTCR1_RCONT)

    }


    //this will start the DFSDM into regular conversion polling mode, only to be used when filter is
    //in an idle state
    pub fn DFSDM_Filter_Regular_Start(&mut self,) {
        //software start of regular conversion
        const DFSDM_FLTCR1_RSWSTART: u64 = 0x00020000;
        //FLTCR1 |= DFSDM_FLTCR1_RSWSTART
    }

    pub fn DFSDM_PollConversion(&mut self,) {
        const DFSDM_FLTISR_REOCF: u64 =0x00000002;
        //while(FLTISR & DFSDM_FLTISR_REOCF != DFSDM_FLTISR_REOCF)
        //{
            //keep waiting but only for x amount of time
        //}
        const DFSDM_FLTISR_ROVRF: u64 = 0x00000008;
        //if FLTISR & DFSDM_FLTISR_ROVRF == DFSDM_FLTISR_ROVRF
        //call an error function?
        //return ok if no errors
    }

    pub fn DFSDM_Reg_ChannelConversion(&mut self,) {
        const DFSDM_FLTRDATAR_RDATACH: u64 = 0x00000007;
        //let mut u32 reg = FLTRDATAR;
            //DFSDM1_Channel0 = reg & DFSDM_FLTRDATAR_RDATACH;
        //reg &= DFSDM_FLTRDATAR_RDATACH;
        // let mut ret_val = reg/256;
        //return ret_val;


    }
}