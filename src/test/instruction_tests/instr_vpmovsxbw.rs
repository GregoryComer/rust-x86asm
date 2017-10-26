use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 236], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 2135099849, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 188, 130, 201, 9, 67, 127], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 215], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 213519814, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 32, 164, 66, 198, 13, 186, 12], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 221], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1982646836, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 148, 178, 52, 202, 44, 118], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 199], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 32, 28, 195], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 32, 225], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EAX, 890865017, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 32, 176, 121, 133, 25, 53], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 125, 143, 32, 210], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1051027842, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 137, 32, 148, 185, 130, 105, 165, 62], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 32, 202], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1531506981, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 32, 44, 213, 37, 241, 72, 91], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 125, 171, 32, 205], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RBX, 757446162, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 32, 171, 18, 182, 37, 45], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 32, 195], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 541951518, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 32, 140, 185, 30, 134, 77, 32], OperandSize::Dword)
}

#[test]
fn vpmovsxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 204, 32, 226], OperandSize::Qword)
}

#[test]
fn vpmovsxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXBW, operand1: Some(Direct(ZMM29)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 32, 41], OperandSize::Qword)
}

