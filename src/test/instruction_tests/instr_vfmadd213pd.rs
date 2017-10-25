use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 168, 231], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 168, 42], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 168, 194], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 168, 60, 66], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 168, 204], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1748706762, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 168, 140, 215, 202, 37, 59, 104], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 168, 225], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 168, 32], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 168, 207], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1537616439, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 140, 168, 36, 253, 55, 42, 166, 91], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 156, 168, 60, 79], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 189, 139, 168, 211], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1110639307, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 237, 135, 168, 52, 157, 203, 2, 51, 66], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 146, 168, 44, 241], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 168, 235], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 262599814, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 174, 168, 140, 247, 134, 244, 166, 15], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 189, 168, 41], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 197, 172, 168, 203], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 245, 161, 168, 1], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 181, 179, 168, 22], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 222, 168, 226], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EBX, 147134636, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 203, 168, 131, 172, 24, 197, 8], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 168, 60, 91], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 181, 213, 168, 205], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 173, 199, 168, 6], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 157, 213, 168, 57], OperandSize::Qword)
}

