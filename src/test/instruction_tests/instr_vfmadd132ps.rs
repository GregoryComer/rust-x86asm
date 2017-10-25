use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 152, 215], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 152, 12, 119], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 152, 238], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 131577914, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 152, 52, 77, 58, 184, 215, 7], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 152, 212], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 152, 26], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 152, 228], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 152, 22], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 152, 215], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 939823246, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 142, 152, 52, 93, 142, 144, 4, 56], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1919720918, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 154, 152, 180, 254, 214, 157, 108, 114], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 37, 143, 152, 217], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 13, 137, 152, 52, 190], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 1909484940, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 21, 153, 152, 156, 128, 140, 109, 208, 113], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 152, 204], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 170, 152, 39], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 426739699, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 188, 152, 164, 199, 243, 135, 111, 25], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 117, 173, 152, 253], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1270476982, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 93, 172, 152, 60, 133, 182, 240, 185, 75], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RAX, 337810091, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 13, 189, 152, 160, 171, 146, 34, 20], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 159, 152, 206], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 152, 20, 81], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 221, 152, 52, 86], OperandSize::Dword)
}

#[test]
fn vfmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 69, 188, 152, 229], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1123882185, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 109, 201, 152, 12, 205, 201, 20, 253, 66], OperandSize::Qword)
}

#[test]
fn vfmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD132PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 91445091, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 21, 215, 152, 36, 133, 99, 87, 115, 5], OperandSize::Qword)
}

