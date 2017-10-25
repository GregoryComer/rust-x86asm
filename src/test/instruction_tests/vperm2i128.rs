use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vperm2i128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 70, 193, 22], OperandSize::Dword)
}

fn vperm2i128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 676667494, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 70, 52, 85, 102, 32, 85, 40, 6], OperandSize::Dword)
}

fn vperm2i128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 70, 229, 103], OperandSize::Qword)
}

fn vperm2i128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2I128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 707114910, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 70, 44, 205, 158, 183, 37, 42, 11], OperandSize::Qword)
}

