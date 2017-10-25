use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vextracti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 227, 118], OperandSize::Dword)
}

fn vextracti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(IndirectDisplaced(EAX, 68156064, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 152, 160, 250, 15, 4, 43], OperandSize::Dword)
}

fn vextracti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 247, 122], OperandSize::Qword)
}

fn vextracti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 28, 119, 43], OperandSize::Qword)
}

