use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 140, 119, 205], OperandSize::Dword)
}

#[test]
fn vpermi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 141, 119, 9], OperandSize::Dword)
}

#[test]
fn vpermi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 159, 119, 52, 122], OperandSize::Dword)
}

#[test]
fn vpermi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 37, 140, 119, 194], OperandSize::Qword)
}

#[test]
fn vpermi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 101, 132, 119, 20, 71], OperandSize::Qword)
}

#[test]
fn vpermi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 2070160044, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 45, 158, 119, 172, 64, 172, 34, 100, 123], OperandSize::Qword)
}

#[test]
fn vpermi2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 170, 119, 219], OperandSize::Dword)
}

#[test]
fn vpermi2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 423003805, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 119, 148, 145, 157, 134, 54, 25], OperandSize::Dword)
}

#[test]
fn vpermi2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 119, 52, 247], OperandSize::Dword)
}

#[test]
fn vpermi2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 85, 162, 119, 218], OperandSize::Qword)
}

#[test]
fn vpermi2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 5, 172, 119, 4, 72], OperandSize::Qword)
}

#[test]
fn vpermi2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 179, 119, 59], OperandSize::Qword)
}

#[test]
fn vpermi2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 206, 119, 237], OperandSize::Dword)
}

#[test]
fn vpermi2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 203, 119, 36, 113], OperandSize::Dword)
}

#[test]
fn vpermi2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDX, 224625294, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 220, 119, 146, 142, 130, 99, 13], OperandSize::Dword)
}

#[test]
fn vpermi2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 37, 206, 119, 212], OperandSize::Qword)
}

#[test]
fn vpermi2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 45, 207, 119, 28, 129], OperandSize::Qword)
}

#[test]
fn vpermi2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1648084341, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 29, 215, 119, 148, 114, 117, 197, 59, 98], OperandSize::Qword)
}

