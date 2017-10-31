use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 158, 200], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1164982497, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 158, 20, 125, 225, 56, 112, 69], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 158, 234], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 158, 12, 211], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 158, 208], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 158, 17], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 158, 209], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 158, 52, 216], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 158, 231], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 932784619, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 140, 158, 156, 211, 235, 41, 153, 55], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 159, 158, 16], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 237, 129, 158, 234], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 158, 12, 248], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDI, 1019730467, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 159, 158, 183, 35, 218, 199, 60], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 158, 193], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 1875649199, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 158, 160, 175, 34, 204, 111], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 801071396, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 190, 158, 155, 36, 97, 191, 47], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 189, 172, 158, 198], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RSI, 236681473, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 141, 164, 158, 182, 1, 121, 27, 14], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1952658704, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 181, 190, 158, 140, 187, 16, 53, 99, 116], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 222, 158, 249], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 43664151, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 205, 158, 132, 154, 23, 67, 154, 2], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1946862563, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 219, 158, 44, 197, 227, 195, 10, 116], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 197, 148, 158, 218], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 5192307, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 189, 193, 158, 60, 189, 115, 58, 79, 0], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 237, 222, 158, 17], OperandSize::Qword)
}

