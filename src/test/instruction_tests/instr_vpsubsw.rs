use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 233, 251], OperandSize::Dword)
}

#[test]
fn vpsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 233, 23], OperandSize::Dword)
}

#[test]
fn vpsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 233, 223], OperandSize::Qword)
}

#[test]
fn vpsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 233, 12, 177], OperandSize::Qword)
}

#[test]
fn vpsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 233, 220], OperandSize::Dword)
}

#[test]
fn vpsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 1026407885, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 233, 137, 205, 189, 45, 61], OperandSize::Dword)
}

#[test]
fn vpsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 233, 254], OperandSize::Qword)
}

#[test]
fn vpsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1517677923, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 233, 52, 77, 99, 237, 117, 90], OperandSize::Qword)
}

#[test]
fn vpsubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 233, 254], OperandSize::Dword)
}

#[test]
fn vpsubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 537794452, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 233, 172, 95, 148, 23, 14, 32], OperandSize::Dword)
}

#[test]
fn vpsubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 109, 137, 233, 253], OperandSize::Qword)
}

#[test]
fn vpsubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1636647624, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 37, 134, 233, 148, 75, 200, 66, 141, 97], OperandSize::Qword)
}

#[test]
fn vpsubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 174, 233, 254], OperandSize::Dword)
}

#[test]
fn vpsubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 175, 233, 60, 119], OperandSize::Dword)
}

#[test]
fn vpsubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 166, 233, 253], OperandSize::Qword)
}

#[test]
fn vpsubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBSW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1651890400, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 5, 172, 233, 164, 202, 224, 216, 117, 98], OperandSize::Qword)
}

