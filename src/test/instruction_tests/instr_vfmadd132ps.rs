use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 152, 225], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 648287421, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 152, 20, 205, 189, 20, 164, 38], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 152, 196], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1063789455, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 152, 188, 147, 143, 35, 104, 63], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 152, 252], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 624370136, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 152, 168, 216, 33, 55, 37], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 152, 206], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 714953423, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 152, 12, 125, 207, 82, 157, 42], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 152, 209], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 628561482, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 152, 164, 78, 74, 22, 119, 37], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 154, 152, 20, 248], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 13, 143, 152, 228], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 109, 143, 152, 4, 199], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1534185483, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 37, 157, 152, 20, 197, 11, 208, 113, 91], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 175, 152, 203], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 152, 4, 249], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 187, 152, 4, 203], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 69, 161, 152, 206], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RSI, 305787762, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 101, 165, 152, 174, 114, 243, 57, 18], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1047027445, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 125, 186, 152, 12, 77, 245, 94, 104, 62], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 185, 152, 246], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDX, 1305835596, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 201, 152, 178, 76, 120, 213, 77], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1194573250, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 222, 152, 164, 178, 194, 189, 51, 71], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 21, 253, 152, 197], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1170873089, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 45, 204, 152, 140, 144, 1, 27, 202, 69], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1999574209, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 61, 221, 152, 172, 67, 193, 20, 47, 119], OperandSize::Qword)
}

