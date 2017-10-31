use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 154, 248], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 154, 25], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 154, 231], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 594328799, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 154, 180, 144, 223, 188, 108, 35], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 154, 192], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 743682189, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 154, 128, 141, 176, 83, 44], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 154, 197], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1537108000, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 154, 36, 117, 32, 104, 158, 91], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 142, 154, 251], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDX, 1330510780, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 154, 146, 188, 251, 77, 79], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 156, 154, 20, 240], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 189, 140, 154, 195], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 221, 138, 154, 12, 222], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 181, 157, 154, 57], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 154, 244], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 154, 42], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 187, 154, 9], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 149, 170, 154, 220], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 165, 170, 154, 4, 74], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1024056463, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 221, 183, 154, 60, 77, 143, 220, 9, 61], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 159, 154, 253], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1529102349, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 202, 154, 20, 253, 13, 64, 36, 91], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 459902391, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 218, 154, 20, 189, 183, 141, 105, 27], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 213, 190, 154, 228], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectDisplaced(RCX, 690299554, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 149, 193, 154, 177, 162, 34, 37, 41], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 50904585, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 205, 223, 154, 164, 191, 9, 190, 8, 3], OperandSize::Qword)
}

