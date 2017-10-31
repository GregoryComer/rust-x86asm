use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 172, 255], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 1424708937, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 172, 182, 73, 85, 235, 84], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 172, 246], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 223614145, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 172, 168, 193, 20, 84, 13], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 172, 215], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 846844661, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 172, 52, 77, 245, 210, 121, 50], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 172, 198], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDI, 1593136224, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 172, 143, 96, 84, 245, 94], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 172, 211], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 584674287, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 172, 140, 74, 239, 107, 217, 34], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 155, 172, 60, 214], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 172, 219], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 77, 129, 172, 28, 71], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RDX, 1662048989, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 77, 149, 172, 154, 221, 218, 16, 99], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 173, 172, 219], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDI, 2121081283, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 171, 172, 135, 195, 33, 109, 126], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 191, 172, 28, 251], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 29, 164, 172, 206], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 61, 174, 172, 50], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RBX, 1441005420, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 21, 178, 172, 171, 108, 255, 227, 85], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 251, 172, 215], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1785281148, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 207, 172, 52, 85, 124, 58, 105, 106], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(ECX, 1308641516, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 219, 172, 169, 236, 72, 0, 78], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 45, 212, 172, 204], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1659928122, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 13, 196, 172, 60, 245, 58, 126, 240, 98], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 791063378, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 61, 211, 172, 28, 149, 82, 171, 38, 47], OperandSize::Qword)
}

