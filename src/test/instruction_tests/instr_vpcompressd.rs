use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcompressd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 139, 210], OperandSize::Dword)
}

#[test]
fn vpcompressd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 949976575, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 8, 139, 36, 221, 255, 125, 159, 56], OperandSize::Dword)
}

#[test]
fn vpcompressd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 137, 139, 215], OperandSize::Qword)
}

#[test]
fn vpcompressd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 125, 8, 139, 14], OperandSize::Qword)
}

#[test]
fn vpcompressd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 139, 227], OperandSize::Dword)
}

#[test]
fn vpcompressd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 40, 139, 44, 186], OperandSize::Dword)
}

#[test]
fn vpcompressd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 172, 139, 207], OperandSize::Qword)
}

#[test]
fn vpcompressd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1900948785, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 40, 139, 172, 126, 49, 45, 78, 113], OperandSize::Qword)
}

#[test]
fn vpcompressd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 139, 239], OperandSize::Dword)
}

#[test]
fn vpcompressd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1119465236, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 139, 12, 133, 20, 175, 185, 66], OperandSize::Dword)
}

#[test]
fn vpcompressd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 207, 139, 194], OperandSize::Qword)
}

#[test]
fn vpcompressd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSD, operand1: Some(IndirectScaledDisplaced(RDX, Two, 2096887930, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 72, 139, 44, 85, 122, 248, 251, 124], OperandSize::Qword)
}

