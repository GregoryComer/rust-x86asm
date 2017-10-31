use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 138, 20, 222], OperandSize::Dword)
}

#[test]
fn vprorvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1256761018, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 138, 20, 180, 113, 186, 166, 232, 74], OperandSize::Dword)
}

#[test]
fn vprorvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 153, 20, 40], OperandSize::Dword)
}

#[test]
fn vprorvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 45, 137, 20, 243], OperandSize::Qword)
}

#[test]
fn vprorvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 21, 140, 20, 6], OperandSize::Qword)
}

#[test]
fn vprorvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 4128558, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 77, 157, 20, 156, 194, 46, 255, 62, 0], OperandSize::Qword)
}

#[test]
fn vprorvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 169, 20, 204], OperandSize::Dword)
}

#[test]
fn vprorvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1332043929, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 173, 20, 132, 176, 153, 96, 101, 79], OperandSize::Dword)
}

#[test]
fn vprorvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 293723586, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 186, 20, 132, 64, 194, 221, 129, 17], OperandSize::Dword)
}

#[test]
fn vprorvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 167, 20, 249], OperandSize::Qword)
}

#[test]
fn vprorvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 53, 171, 20, 59], OperandSize::Qword)
}

#[test]
fn vprorvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 174157506, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 29, 185, 20, 44, 253, 194, 110, 97, 10], OperandSize::Qword)
}

#[test]
fn vprorvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 20, 192], OperandSize::Dword)
}

#[test]
fn vprorvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 554314632, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 203, 20, 44, 69, 136, 43, 10, 33], OperandSize::Dword)
}

#[test]
fn vprorvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1442023917, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 220, 20, 28, 253, 237, 137, 243, 85], OperandSize::Dword)
}

#[test]
fn vprorvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 125, 194, 20, 228], OperandSize::Qword)
}

#[test]
fn vprorvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 77, 201, 20, 7], OperandSize::Qword)
}

#[test]
fn vprorvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 61, 215, 20, 55], OperandSize::Qword)
}

