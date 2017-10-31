use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 249, 234], OperandSize::Dword)
}

#[test]
fn vpsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1265925700, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 249, 164, 115, 68, 126, 116, 75], OperandSize::Dword)
}

#[test]
fn vpsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 249, 228], OperandSize::Qword)
}

#[test]
fn vpsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 249, 38], OperandSize::Qword)
}

#[test]
fn vpsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 249, 249], OperandSize::Dword)
}

#[test]
fn vpsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 249, 36, 179], OperandSize::Dword)
}

#[test]
fn vpsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 249, 223], OperandSize::Qword)
}

#[test]
fn vpsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDX, 4071821, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 249, 146, 141, 33, 62, 0], OperandSize::Qword)
}

#[test]
fn vpsubw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 138, 249, 200], OperandSize::Dword)
}

#[test]
fn vpsubw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 249, 19], OperandSize::Dword)
}

#[test]
fn vpsubw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 37, 130, 249, 228], OperandSize::Qword)
}

#[test]
fn vpsubw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 93, 142, 249, 52, 95], OperandSize::Qword)
}

#[test]
fn vpsubw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 169, 249, 205], OperandSize::Dword)
}

#[test]
fn vpsubw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1344465292, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 249, 140, 215, 140, 233, 34, 80], OperandSize::Dword)
}

#[test]
fn vpsubw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 109, 167, 249, 213], OperandSize::Qword)
}

#[test]
fn vpsubw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 69, 165, 249, 60, 158], OperandSize::Qword)
}

#[test]
fn vpsubw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 206, 249, 214], OperandSize::Dword)
}

#[test]
fn vpsubw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 696675686, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 249, 12, 253, 102, 109, 134, 41], OperandSize::Dword)
}

#[test]
fn vpsubw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 45, 193, 249, 203], OperandSize::Qword)
}

#[test]
fn vpsubw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 199, 249, 20, 113], OperandSize::Qword)
}

