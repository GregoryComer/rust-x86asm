use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 141, 209], OperandSize::Dword)
}

#[test]
fn vpermw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 141, 60, 82], OperandSize::Dword)
}

#[test]
fn vpermw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 165, 138, 141, 252], OperandSize::Qword)
}

#[test]
fn vpermw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 133, 141, 54], OperandSize::Qword)
}

#[test]
fn vpermw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 141, 203], OperandSize::Dword)
}

#[test]
fn vpermw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 141, 52, 198], OperandSize::Dword)
}

#[test]
fn vpermw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 164, 141, 242], OperandSize::Qword)
}

#[test]
fn vpermw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 423662074, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 181, 172, 141, 20, 77, 250, 145, 64, 25], OperandSize::Qword)
}

#[test]
fn vpermw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 207, 141, 253], OperandSize::Dword)
}

#[test]
fn vpermw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1200560704, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 203, 141, 172, 194, 64, 26, 143, 71], OperandSize::Dword)
}

#[test]
fn vpermw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 181, 202, 141, 212], OperandSize::Qword)
}

#[test]
fn vpermw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 929362088, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 189, 205, 141, 172, 118, 168, 240, 100, 55], OperandSize::Qword)
}

