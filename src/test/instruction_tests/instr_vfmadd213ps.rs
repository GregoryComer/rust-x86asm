use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 168, 250], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 168, 60, 200], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 168, 245], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 168, 24], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 168, 196], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 168, 56], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 168, 239], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 166550558, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 168, 180, 71, 30, 92, 237, 9], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 168, 235], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 382897695, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 140, 168, 36, 125, 31, 142, 210, 22], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 410864887, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 156, 168, 156, 88, 247, 76, 125, 24], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 21, 142, 168, 206], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 93, 135, 168, 59], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RCX, 1608486097, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 77, 154, 168, 177, 209, 140, 223, 95], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 168, 195], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 168, 52, 192], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 185, 168, 60, 81], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 37, 164, 168, 223], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1464850395, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 85, 167, 168, 52, 181, 219, 215, 79, 87], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDI, 283069896, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 186, 168, 167, 200, 77, 223, 16], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 252, 168, 231], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1830309731, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 207, 168, 156, 143, 99, 79, 24, 109], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ESI, 1685167476, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 168, 158, 116, 157, 113, 100], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 21, 157, 168, 245], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectDisplaced(RAX, 1947887648, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 53, 199, 168, 136, 32, 104, 26, 116], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 21, 219, 168, 12, 139], OperandSize::Qword)
}

