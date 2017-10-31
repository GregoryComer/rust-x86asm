use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 168, 226], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 168, 17], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 168, 244], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1192273624, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 168, 36, 205, 216, 166, 16, 71], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 168, 251], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 961189960, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 168, 175, 72, 152, 74, 57], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 168, 192], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDI, 199396411, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 168, 183, 59, 140, 226, 11], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 168, 249], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 168, 60, 81], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1050927218, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 154, 168, 132, 184, 114, 224, 163, 62], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 61, 135, 168, 209], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 140, 168, 25], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 101, 156, 168, 4, 217], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 168, 228], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 168, 52, 78], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 1790651834, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 187, 168, 180, 247, 186, 45, 187, 106], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 29, 173, 168, 235], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RCX, 1738487824, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 37, 170, 168, 129, 16, 56, 159, 103], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 187, 168, 44, 78], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 222, 168, 231], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 2125799158, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 206, 168, 4, 157, 246, 30, 181, 126], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1998018631, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 223, 168, 20, 69, 71, 88, 23, 119], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 53, 243, 168, 204], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1478363370, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 13, 202, 168, 164, 80, 234, 8, 30, 88], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1636318932, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 21, 213, 168, 188, 250, 212, 62, 136, 97], OperandSize::Qword)
}

