use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vperm2f128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 6, 209, 108], OperandSize::Dword)
}

fn vperm2f128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 6, 46, 69], OperandSize::Dword)
}

fn vperm2f128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 6, 220, 92], OperandSize::Qword)
}

fn vperm2f128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERM2F128, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 550017324, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 6, 140, 145, 44, 153, 200, 32, 126], OperandSize::Qword)
}

