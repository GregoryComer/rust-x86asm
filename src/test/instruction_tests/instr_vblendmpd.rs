use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 143, 101, 253], OperandSize::Dword)
}

#[test]
fn vblendmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 101, 20, 176], OperandSize::Dword)
}

#[test]
fn vblendmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 153, 101, 4, 65], OperandSize::Dword)
}

#[test]
fn vblendmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 149, 139, 101, 193], OperandSize::Qword)
}

#[test]
fn vblendmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1825214513, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 141, 142, 101, 140, 195, 49, 144, 202, 108], OperandSize::Qword)
}

#[test]
fn vblendmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1886213308, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 154, 101, 172, 209, 188, 84, 109, 112], OperandSize::Qword)
}

#[test]
fn vblendmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 172, 101, 196], OperandSize::Dword)
}

#[test]
fn vblendmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1294383833, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 172, 101, 140, 122, 217, 186, 38, 77], OperandSize::Dword)
}

#[test]
fn vblendmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 424762730, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 188, 101, 156, 75, 106, 93, 81, 25], OperandSize::Dword)
}

#[test]
fn vblendmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 173, 172, 101, 219], OperandSize::Qword)
}

#[test]
fn vblendmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 613748783, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 174, 101, 12, 189, 47, 16, 149, 36], OperandSize::Qword)
}

#[test]
fn vblendmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectDisplaced(RBX, 217152804, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 197, 178, 101, 139, 36, 125, 241, 12], OperandSize::Qword)
}

#[test]
fn vblendmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 201, 101, 253], OperandSize::Dword)
}

#[test]
fn vblendmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 188675020, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 101, 52, 85, 204, 243, 62, 11], OperandSize::Dword)
}

#[test]
fn vblendmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 101, 44, 86], OperandSize::Dword)
}

#[test]
fn vblendmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 213, 206, 101, 196], OperandSize::Qword)
}

#[test]
fn vblendmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 909840575, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 221, 197, 101, 52, 245, 191, 16, 59, 54], OperandSize::Qword)
}

#[test]
fn vblendmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1112542042, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 181, 223, 101, 44, 149, 90, 11, 80, 66], OperandSize::Qword)
}

