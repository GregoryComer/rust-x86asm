use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 248, 225], OperandSize::Dword)
}

#[test]
fn vpsubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1222841767, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 248, 60, 117, 167, 21, 227, 72], OperandSize::Dword)
}

#[test]
fn vpsubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 248, 246], OperandSize::Qword)
}

#[test]
fn vpsubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RCX, 1782529083, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 248, 129, 59, 60, 63, 106], OperandSize::Qword)
}

#[test]
fn vpsubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 248, 249], OperandSize::Dword)
}

#[test]
fn vpsubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 248, 44, 91], OperandSize::Dword)
}

#[test]
fn vpsubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 248, 216], OperandSize::Qword)
}

#[test]
fn vpsubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 248, 52, 155], OperandSize::Qword)
}

#[test]
fn vpsubb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 248, 201], OperandSize::Dword)
}

#[test]
fn vpsubb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 248, 59], OperandSize::Dword)
}

#[test]
fn vpsubb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 101, 137, 248, 253], OperandSize::Qword)
}

#[test]
fn vpsubb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 53, 137, 248, 30], OperandSize::Qword)
}

#[test]
fn vpsubb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 248, 239], OperandSize::Dword)
}

#[test]
fn vpsubb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EBX, 349736471, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 175, 248, 187, 23, 142, 216, 20], OperandSize::Dword)
}

#[test]
fn vpsubb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 45, 165, 248, 241], OperandSize::Qword)
}

#[test]
fn vpsubb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1705005560, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 5, 172, 248, 52, 253, 248, 81, 160, 101], OperandSize::Qword)
}

#[test]
fn vpsubb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 248, 207], OperandSize::Dword)
}

#[test]
fn vpsubb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1728229067, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 205, 248, 132, 217, 203, 174, 2, 103], OperandSize::Dword)
}

#[test]
fn vpsubb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 61, 196, 248, 231], OperandSize::Qword)
}

#[test]
fn vpsubb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 369476535, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 13, 204, 248, 172, 94, 183, 195, 5, 22], OperandSize::Qword)
}

