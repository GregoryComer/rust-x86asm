use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 248], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1696891640, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 36, 149, 248, 130, 36, 101], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 235], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 638177132, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 35, 140, 78, 108, 207, 9, 38], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 250], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EBX, 1677089423, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 163, 143, 90, 246, 99], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 252], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 571787365, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 35, 44, 205, 101, 200, 20, 34], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 35, 247], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 35, 14], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 125, 142, 35, 255], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1541389098, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 125, 143, 35, 188, 67, 42, 187, 223, 91], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 35, 229], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1508066152, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 35, 4, 253, 104, 67, 227, 89], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 125, 170, 35, 208], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(YMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1416798702, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 171, 35, 180, 86, 238, 161, 114, 84], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 35, 226], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 35, 36, 191], OperandSize::Dword)
}

#[test]
fn vpmovsxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 206, 35, 221], OperandSize::Qword)
}

#[test]
fn vpmovsxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXWD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(RSI, 92118330, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 35, 174, 58, 157, 125, 5], OperandSize::Qword)
}

