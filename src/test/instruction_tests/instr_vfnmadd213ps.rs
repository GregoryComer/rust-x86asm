use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 172, 217], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1406257767, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 172, 44, 125, 103, 202, 209, 83], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 172, 227], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 43420269, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 172, 4, 69, 109, 138, 150, 2], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 172, 253], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 1684655656, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 172, 175, 40, 206, 105, 100], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 172, 230], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 281631799, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 172, 28, 117, 55, 92, 201, 16], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 172, 223], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 1558464589, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 172, 179, 77, 72, 228, 92], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 53359810, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 156, 172, 60, 117, 194, 52, 46, 3], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 132, 172, 204], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 101, 129, 172, 0], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectDisplaced(RAX, 193448794, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 85, 150, 172, 144, 90, 203, 135, 11], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 172, 208], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 427583610, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 172, 187, 122, 104, 124, 25], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 186, 172, 36, 83], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 77, 175, 172, 209], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 172, 172, 28, 223], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectDisplaced(RDX, 1513720286, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 77, 178, 172, 162, 222, 137, 57, 90], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 252, 172, 193], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ECX, 48788824, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 172, 137, 88, 117, 232, 2], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 223, 172, 46], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 101, 149, 172, 231], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 37, 194, 172, 36, 89], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RDI, 1832498120, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 69, 212, 172, 143, 200, 179, 57, 109], OperandSize::Qword)
}

